use async_openai::{config::OpenAIConfig, Client};
use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, query, query_as, Pool, Postgres};
use uuid::Uuid;

use crate::utils::config::Convinience;

use super::MessageEmbedding;

#[derive(Debug, Clone, FromRow)]
pub struct Message {
    pub id: Uuid,
    pub chat_id: Uuid,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Message {
    pub async fn new(
        pool: &Pool<Postgres>,
        chat_id: Uuid, 
        role: String, 
        content: String
    ) -> Result<Self, sqlx::Error> {
        let message = Self {
            id: Uuid::new_v4(),
            chat_id,
            role,
            content,
            created_at: Utc::now(),
            updated_at: None,
            deleted_at: None,
        };

        query!(
            r#"
            INSERT INTO messages (id, chat_id, role, content, created_at, updated_at, deleted_at) 
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#, 
            message.id, 
            message.chat_id, 
            message.role, 
            message.content, 
            message.created_at, 
            message.updated_at, 
            message.deleted_at
        )
        .execute(pool)
        .await?;

        Ok(message)
    }

    pub async fn new_with_embedding(
        pool: &Pool<Postgres>,
        openai_client: &Client<OpenAIConfig>,
        chat_id: Uuid, 
        role: String, 
        content: String
    ) -> Result<(Self, MessageEmbedding), sqlx::Error> {
        let message = Self::new(pool, chat_id, role, content.clone()).await?;

        let embedding = openai_client.get_embedding(content.clone())
            .await
            .map_err(|e| sqlx::Error::Decode(e.into()))?;

        let message_embedding = MessageEmbedding::new(pool, message.id, embedding, None).await?;

        Ok((message, message_embedding))
    }

    pub async fn get_all_messages_for_chat(pool: &Pool<Postgres>, chat_id: Uuid) -> Result<Vec<Self>, sqlx::Error> {
        let messages = query_as!(
            Self,
            r#"
            SELECT id, chat_id, role, content, created_at, updated_at, deleted_at 
            FROM messages 
            WHERE chat_id = $1 AND deleted_at IS NULL
            ORDER BY created_at ASC
            "#,
            chat_id
        )
        .fetch_all(pool)
        .await?;

        Ok(messages)
    }
}