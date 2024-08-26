use sqlx::{prelude::FromRow, query, Pool, Postgres};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct MessageEmbedding {
    pub id: Uuid,
    pub message_id: Uuid,
    pub embedding: Vec<f32>,
    pub section: Option<i16>,
}

impl MessageEmbedding {
    pub async fn new(
        pool: &Pool<Postgres>,
        message_id: Uuid,
        embedding: Vec<f32>,
        section: Option<i16>,
    ) -> Result<Self, sqlx::Error> {
        let me = Self {
            id: Uuid::new_v4(),
            message_id,
            embedding,
            section,
        };

        query(
            r#"INSERT INTO message_embeddings (id, message_id, embedding, section) 
            VALUES ($1, $2, $3, $4)"#,
        )
        .bind(me.id)
        .bind(me.message_id)
        .bind(me.embedding.as_slice())
        .bind(me.section)
        .execute(pool)
        .await?;

        Ok(me)
    }
}
