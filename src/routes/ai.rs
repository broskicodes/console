use actix_web::{post, web, Error};
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionRequestUserMessageContent, ChatCompletionResponseMessage, CreateChatCompletionRequestArgs};
// use tracing::info;

use crate::{model::{chat::Chat, message::Message}, types::ai::SendMessageRequest, utils::insert_message, AppState};

#[post("/send-message")]
async fn send_message(
    app_state: web::Data<AppState>,
    req_body: web::Json<SendMessageRequest>
) -> Result<web::Json<ChatCompletionResponseMessage>, Error> {
    let body = req_body.into_inner();
    let chat_id = body.chat_id.clone();
    let model = body.model.clone();
    let flavour = body.flavour.clone();
    let mut messages: Vec<ChatCompletionRequestMessage> = vec![
        ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessageArgs::default()
            .content(flavour.prompt().to_string())
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

            Message::new(&app_state.pool, chat_id.clone(), String::from("system"), flavour.prompt().to_string())
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
            
            insert_message(&app_state.pool, &app_state.openai_client, chat_id.clone(), role, content)
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
    
    insert_message(&app_state.pool, &app_state.openai_client, chat_id.clone(), String::from("assistant"), response_content)
        .await
        .map_err(|e| Error::from(actix_web::error::ErrorInternalServerError(e.to_string())))?;

    Ok(web::Json(response_message))
}