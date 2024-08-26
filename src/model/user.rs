use chrono::{DateTime, Utc};
use sqlx::{FromRow, query, query_as, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}

impl User {
    pub async fn new(
        pool: &Pool<Postgres>,
        user_id: Uuid
    ) -> Result<Self, sqlx::Error> {
        let user = Self {
            id: user_id,
            created_at: Utc::now(),
            updated_at: Some(Utc::now()),
            deleted_at: None,
        };

        query!(
            r#"
            INSERT INTO users (id, created_at, updated_at, deleted_at) 
            VALUES ($1, $2, $3, $4)
            "#,
            user.id,
            user.created_at,
            user.updated_at,
            user.deleted_at
        )
        .execute(pool)
        .await?; 

        Ok(user)
    }

    pub async fn get(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        let user = query_as!(
            Self,
            r#"
            SELECT id, created_at, updated_at, deleted_at 
            FROM users 
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_optional(pool)
        .await?;

        Ok(user)
    }

    pub async fn get_or_create(pool: &Pool<Postgres>, user_id: Uuid) -> Result<Self, sqlx::Error> {
        let user = Self::get(pool, user_id).await?;
        match user {
            Some(user) => Ok(user),
            None => Self::new(pool, user_id).await,
        }
    }
}