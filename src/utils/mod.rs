use async_openai::{config::OpenAIConfig, types::CreateEmbeddingRequestArgs, Client};
use sqlx::PgPool;
use uuid::Uuid;

use crate::model::{message::Message, message_embedding::MessageEmbedding};

pub async fn insert_message(
    pool: &PgPool,
    openai_client: &Client<OpenAIConfig>,
    chat_id: Uuid,
    role: String,
    content: String,
) -> Result<(), anyhow::Error> {
    let message = Message::new(pool, chat_id.clone(), role.clone(), content.clone())
        .await?;

    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-3-small")
        .dimensions(384 as u32)
        .input(content.clone())
        .build()?;

    let embedding = openai_client
        .embeddings()
        .create(request)
        .await?
        .data
        .first()
        .ok_or(anyhow::anyhow!("Error creating embedding"))?
        .embedding
        .clone();

    MessageEmbedding::new(pool, message.id, embedding, None).await?;

    Ok(())
}