use actix_web::error::ErrorInternalServerError;
use actix_web::{post, web, Error};
use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionResponseMessage, CreateChatCompletionRequestArgs,
};
use chrono::Local;
use tracing::info;
use uuid::Uuid;

use crate::utils::config::{Convinience, Parsable};
use crate::{
    middleware::auth::AuthenticatedUser,
    model::{Chat, Message},
    types::{ChatPrompts, SendMessageRequest},
    utils::graph::create_knowledge_from_chat,
    AppState,
};

#[post("/send-message")]
async fn send_message(
    app_state: web::Data<AppState>,
    req_body: web::Json<SendMessageRequest>,
    user: AuthenticatedUser,
) -> Result<web::Json<ChatCompletionResponseMessage>, Error> {
    let body = req_body.into_inner();
    let chat_id = body.chat_id.clone();
    let model = body.model.clone();
    let flavour = body.flavour.clone();

    let last_message = body.messages.last().cloned();

    let chat_sys_prompt = match flavour {
        ChatPrompts::InitialGoals => ChatPrompts::InitialGoals.prompt_template(),
        ChatPrompts::DailyOutline => {
            let (embedding_content, threshold) = match &last_message {
                Some(message) => {
                    let (_, content) = app_state
                        .openai_client
                        .get_data_from_message_request(message.clone())
                        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

                    (content, 0.4)
                }
                None => (String::from(""), 0.0),
            };

            let embedding = app_state
                .openai_client
                .get_embedding(embedding_content)
                .await
                .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

            let context = app_state
                .graph
                .semantic_search(&user.user_id, embedding, threshold)
                .await
                .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?
                .to_context()
                .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

            &ChatPrompts::DailyOutline
                .prompt_template()
                .replace("{date}", &Local::now().format("%B %d, %Y").to_string())
                .replace("{context}", &context)
        }
    };

    info!("sys prompt: {}", chat_sys_prompt);

    let mut messages: Vec<ChatCompletionRequestMessage> =
        vec![ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(chat_sys_prompt)
                .build()
                .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?,
        )];

    messages.extend(body.messages.clone().iter().cloned());

    let existing_chat = Chat::get(&app_state.pool, chat_id.clone())
        .await
        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

    match existing_chat {
        None => {
            Chat::new(
                &app_state.pool,
                Some(chat_id.clone()),
                user.user_id.clone(),
                flavour.clone(),
            )
            .await
            .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

            Message::new(
                &app_state.pool,
                chat_id.clone(),
                String::from("system"),
                chat_sys_prompt.to_string(),
            )
            .await
            .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;
        }
        _ => {}
    };

    match &last_message {
        Some(message) => {
            let (role, content) = app_state
                .openai_client
                .get_data_from_message_request(message.clone())
                .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

            Message::new_with_embedding(
                &app_state.pool,
                &app_state.openai_client,
                chat_id.clone(),
                role,
                content,
            )
            .await
            .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;
        }
        None => {}
    }

    let client = app_state.openai_client.clone();

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .build()
        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

    let response_message = response.choices[0].message.clone();
    let response_content =
        response_message
            .clone()
            .content
            .ok_or(Error::from(ErrorInternalServerError(String::from(
                "No content in AI response",
            ))))?;

    let final_message: Option<String> =
        regex::Regex::new(r"<final_message>((?s).*?)</final_message>")
            .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?
            .captures(&response_content)
            .map(|cap| Some(cap.get(1).unwrap().as_str().to_string()))
            .unwrap_or(None);

    if let Some(_) = final_message {
        match flavour {
            ChatPrompts::InitialGoals => {
                info!("Spawning thread to create knowledge graph.");

                let chat_id = chat_id.clone();
                let app_state = app_state.clone();

                tokio::spawn(async move {
                    let _ = create_knowledge_from_chat(
                        app_state.into_inner(),
                        user.user_id.clone(),
                        chat_id,
                    );
                });
            }
            _ => {}
        }
    }

    Message::new_with_embedding(
        &app_state.pool,
        &app_state.openai_client,
        chat_id.clone(),
        String::from("assistant"),
        response_content,
    )
    .await
    .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

    Ok(web::Json(response_message))
}

#[post("/create-knowledge-graph")]
async fn create_knowledge_graph(
    app_state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<web::Json<String>, Error> {
    // let chat_id = Uuid::try_parse("004a7905-f15f-4ddb-be83-6958cd4a3fa8")
    let chat_id = Uuid::try_parse("91550d27-87ca-4005-9580-03ab2ef4edf5")
        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

    let schema = create_knowledge_from_chat(app_state.into_inner(), user.user_id.clone(), chat_id)
        .await
        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

    Ok(web::Json(schema))
}

#[post("/search-graph")]
async fn search_knowledge_graph(
    app_state: web::Data<AppState>,
    user: AuthenticatedUser,
    // req_body: web::Json<SearchGraphRequest>
) -> Result<web::Json<String>, Error> {
    let embedding = app_state
        .openai_client
        .get_embedding(String::from("What are my goals?"))
        .await
        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

    let graph = app_state
        .graph
        .semantic_search(&user.user_id, embedding, 0.3)
        .await
        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

    let context = graph
        .to_context()
        .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;
    Ok(web::Json(context))
}
