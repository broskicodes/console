use chrono::{DateTime, Utc};
use sqlx::{FromRow, query, query_as, Pool, Postgres};
use uuid::Uuid;

use crate::types::ai::ChatPrompts;

#[derive(Debug, Clone, FromRow)]
pub struct Chat {
    pub id: Uuid,
    pub user_id: Uuid,
    pub flavour: ChatPrompts,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl Chat {
    pub async fn new(
        pool: &Pool<Postgres>,
        chat_id: Option<Uuid>,
        user_id: Uuid,
        flavour: ChatPrompts,
    ) -> Result<Self, sqlx::Error> {
        let chat = Self {
            id: chat_id.unwrap_or(Uuid::new_v4()),
            flavour,
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            deleted_at: None,
            user_id,
        };

        query!(
            r#"
            INSERT INTO chats (id, flavour, created_at, updated_at, deleted_at, user_id) 
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            chat.id,
            chat.flavour.clone() as ChatPrompts,
            chat.created_at,
            chat.updated_at,
            chat.deleted_at,
            chat.user_id
        )
        .execute(pool)
        .await?; 

        Ok(chat)
    }

    pub async fn get(pool: &Pool<Postgres>, chat_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        let chat = query_as!(
            Self,
            r#"
            SELECT id, flavour as "flavour: ChatPrompts", created_at, updated_at, deleted_at, user_id
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