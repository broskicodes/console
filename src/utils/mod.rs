pub mod constants;
pub mod graph;

use std::sync::Arc;

use async_openai::{config::OpenAIConfig, types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionResponseFormat, ChatCompletionResponseFormatType, CreateChatCompletionRequestArgs, CreateEmbeddingRequestArgs}, Client};
use tracing::info;
use uuid::Uuid;

use crate::{model::{Message, MessageEmbedding}, types::{CypherQueries, GraphData, PromptFlavour}, utils::constants::GRAPH_DATA_DEF, AppState};

use self::constants::GRAPH_SCHEMA;
use self::graph::run_graph_queries;

pub async fn get_embedding(openai_client: &Client<OpenAIConfig>, content: String) -> Result<Vec<f32>, anyhow::Error> {
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

    Ok(embedding)
}

pub async fn insert_message_with_embedding(
    app_state: Arc<AppState>,
    chat_id: Uuid,
    role: String,
    content: String,
) -> Result<(), anyhow::Error> {
    let message = Message::new(&app_state.pool, chat_id.clone(), role.clone(), content.clone())
        .await?;

    let embedding = get_embedding(&app_state.openai_client, content).await?;

    MessageEmbedding::new(&app_state.pool, message.id, embedding, None).await?;

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
                        .replace("{graph_data}", GRAPH_DATA_DEF)
                        .replace("{date}", &chrono::Local::now().format("%B %d, %Y").to_string())
                )
                .build()?
            )
        ])
        .build()?;

    let response = client.chat().create(request).await?;
    let content = response.choices[0].message.content.clone().ok_or(anyhow::anyhow!("No content in AI response"))?;

    info!("Generated AI response.");

    let graph_data: GraphData = serde_json::from_str(&content)?;
    let queries: CypherQueries = graph_data.into_queries(&app_state.openai_client).await?;

    info!("Generated {} Cypher queries.", queries.queries.len());

    let graph = app_state.graph.clone();
    run_graph_queries(&graph, queries.queries).await?;

    info!("Knowledge graph created.");

    Ok(content)
}
