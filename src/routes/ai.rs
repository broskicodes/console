use actix_web::{post, web, Error};
use async_openai::types::{ChatCompletionResponseMessage, CreateChatCompletionRequestArgs};
// use tracing::info;

use crate::{types::ai::SendMessageRequest, AppState};

#[post("/send-message")]
async fn send_message(
    app_state: web::Data<AppState>,
    req_body: web::Json<SendMessageRequest>
) -> Result<web::Json<ChatCompletionResponseMessage>, Error> {
    let body = req_body.into_inner();
    let model = body.model.clone();
    let messages = body.messages.clone();
    
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

    Ok(web::Json(response.choices[0].message.clone()))
}