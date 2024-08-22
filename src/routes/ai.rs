use std::str::FromStr;

use actix_web::{post, web, Error};
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageContent, ChatCompletionResponseMessage, CreateChatCompletionRequestArgs};
use tracing::info;
use uuid::Uuid;

use crate::{model::{chat::Chat, message::Message}, types::ai::SendMessageRequest, utils::{create_knowledge_from_chat, insert_message_with_embedding}, AppState};

#[post("/send-message")]
async fn send_message(
    app_state: web::Data<AppState>,
    req_body: web::Json<SendMessageRequest>
) -> Result<web::Json<ChatCompletionResponseMessage>, Error> {
    let body = req_body.into_inner();
    let chat_id = body.chat_id.clone();
    let model = body.model.clone();
    let flavour = body.flavour.clone();
    let chat_sys_prompt = flavour.prompt_template();
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessageArgs::default()
            .content(chat_sys_prompt)
            .build()
            .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?
        )
    ];
    
    messages.extend(body.messages.clone().iter().cloned());

    let existing_chat = Chat::get(&app_state.pool, chat_id.clone())
        .await
        .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;

    match existing_chat {
        None => {
            Chat::new(&app_state.pool, Some(chat_id.clone()), flavour.clone())
                .await
                .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;

            Message::new(&app_state.pool, chat_id.clone(), String::from("system"), chat_sys_prompt.to_string())
                .await
                .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;
        }
        _ => {}
    };

    match body.messages.last() {
        Some(message) => {
            let (role, content): (String, String) = match message {
                ChatCompletionRequestMessage::User(user) => {
                    match user.content.clone() {
                        ChatCompletionRequestUserMessageContent::Text(text) => (String::from("user"), text),
                        _ => return Err(Error::from(actix_web::error::ErrorInternalServerError(String::from("Only text messages are supported")))),
                    }
                }
                _ => return Err(Error::from(actix_web::error::ErrorInternalServerError(String::from("Last message must be a user message")))),
            };
            
            insert_message_with_embedding(&app_state.pool, &app_state.openai_client, chat_id.clone(), role, content)
                .await
                .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;
        }
        None => {}
    }
            
    let client = app_state.openai_client.clone();

    let request = CreateChatCompletionRequestArgs::default()
        .model(model)
        .messages(messages)
        .build()
        .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;

    let response = client
        .chat()
        .create(request)
        .await
        .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;

    let response_message = response.choices[0].message.clone();
    let response_content = response_message.clone().content.ok_or(Error::from(actix_web::error::ErrorInternalServerError(String::from("No content in AI response"))))?;
    
    let final_message: Option<String> = regex::Regex::new(r"<final_message>((?s).*?)</final_message>")
        .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?
        .captures(&response_content)
        .map(|cap| Some(cap.get(1).unwrap().as_str().to_string()))
        .unwrap_or(None);

    if let Some(_) = final_message {
        info!("Spawning thread to create knowledge graph.");

        let chat_id = chat_id.clone();
        let app_state = app_state.clone();

        tokio::spawn(async move {
            let _ = create_knowledge_from_chat(app_state.into_inner(), chat_id);
        });
    }
    
    insert_message_with_embedding(&app_state.pool, &app_state.openai_client, chat_id.clone(), String::from("assistant"), response_content)
        .await
        .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;

    Ok(web::Json(response_message))
}

#[post("/create-knowledge-graph")]
async fn create_knowledge_graph(
    app_state: web::Data<AppState>,
) -> Result<web::Json<String>, Error> {
    let chat_id = Uuid::from_str("004a7905-f15f-4ddb-be83-6958cd4a3fa8")
        .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;

    let schema = create_knowledge_from_chat(app_state.into_inner(), chat_id)
        .await
        .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;

    Ok(web::Json(schema))
}