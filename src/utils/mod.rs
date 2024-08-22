pub mod constants;

use std::sync::Arc;

use async_openai::{config::OpenAIConfig, types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionResponseFormat, ChatCompletionResponseFormatType, CreateChatCompletionRequestArgs, CreateEmbeddingRequestArgs}, Client};
use neo4rs::{query, Query};
use sqlx::PgPool;
use tracing::info;
use uuid::Uuid;

use crate::{model::{message::Message, message_embedding::MessageEmbedding}, types::ai::{CypherQueries, PromptFlavour}, AppState};

use self::constants::GRAPH_SCHEMA;

pub async fn insert_message_with_embedding(
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

pub async fn create_knowledge_from_chat(app_state: Arc<AppState>, chat_id: Uuid) -> Result<String, anyhow::Error> {
    let messages = Message::get_all_messages_for_chat(&app_state.pool, chat_id.clone()).await?;

    let interview = messages.iter()
        .map(|m| format!("{}: {}", m.role, m.content))
        .collect::<Vec<String>>()
        .join("\n");

    let client = app_state.openai_client.clone();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .response_format(ChatCompletionResponseFormat {
            r#type: ChatCompletionResponseFormatType::JsonObject,
        })
        .messages(vec![
            ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessageArgs::default()
                .content(
                    PromptFlavour::ExtractEntities.prompt_template()
                        .replace("{interview}", &interview)
                        .replace("{graph_schema}", GRAPH_SCHEMA)
                )
                .build()?
            )
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    let content = response.choices[0].message.content.clone().ok_or(anyhow::anyhow!("No content in AI response"))?;

    let queries: CypherQueries = serde_json::from_str(&content)
        .map_err(|e| anyhow::anyhow!("Failed to parse JSON: {}", e))?;

    info!("Running queries: {:?}", queries.queries);
    
    let graph = app_state.graph.clone();
    let mut txn = graph.start_txn().await?;
    let _ = txn.run_queries(
        queries.queries
            .iter()
            .map(|q| query(q))
            .collect::<Vec<Query>>()
    )
    .await?;

    txn.commit().await?;

    Ok(content)
}
