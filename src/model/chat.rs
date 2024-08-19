use chrono::{DateTime, Utc};
use sqlx::{FromRow, query, query_as, Pool, Postgres};
use uuid::Uuid;

use crate::types::ai::PromptFlavour;

#[derive(Debug, Clone, FromRow)]
pub struct Chat {
    pub id: Uuid,
    pub flavour: PromptFlavour,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Chat {
    pub async fn new(
        pool: &Pool<Postgres>,
        chat_id: Option<Uuid>,
        flavour: PromptFlavour,
    ) -> Result<Self, sqlx::Error> {
        let chat = Self {
            id: chat_id.unwrap_or(Uuid::new_v4()),
            flavour,
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            deleted_at: None,
        };

        query!(
            r#"
            INSERT INTO chats (id, flavour, created_at, updated_at, deleted_at) 
            VALUES ($1, $2, $3, $4, $5)
            "#,
            chat.id,
            chat.flavour.clone() as PromptFlavour,
            chat.created_at,
            chat.updated_at,
            chat.deleted_at
        )
        .execute(pool)
        .await?; 

        Ok(chat)
    }

    pub async fn get(pool: &Pool<Postgres>, chat_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        let chat = query_as!(
            Self,
            r#"
            SELECT id, flavour as "flavour: PromptFlavour", created_at, updated_at, deleted_at 
            FROM chats 
            WHERE id = $1
            "#,
            chat_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(chat)
    }
}