use chrono::{DateTime, Utc};
use sqlx::{prelude::FromRow, query, Pool, Postgres};
use uuid::Uuid;

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
}