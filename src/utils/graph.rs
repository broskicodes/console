use std::collections::HashMap;
use std::sync::Arc;

use neo4rs::{query, Error, Graph};
use async_openai::types::{ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs, ChatCompletionResponseFormat, ChatCompletionResponseFormatType, CreateChatCompletionRequestArgs};
use tracing::info;
use uuid::Uuid;

use crate::model::{Neo4jNode, Neo4jRelation, Message};
use crate::utils::{config::{Parsable, AppState}, constants::{GRAPH_SCHEMA, GRAPH_DATA_DEF}};
use crate::types::{CypherQueries, GraphData, ToolPrompts};


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
                    ToolPrompts::ExtractEntities.prompt_template()
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

    app_state.graph.run_queries(queries.queries).await?;

    info!("Knowledge graph created.");

    Ok(content)
}

pub async fn search_graph(graph: &Graph, user_id: &str, search_query_embedding: Vec<f32>, threshold: f32) -> Result<(HashMap<i64, Neo4jNode>, Vec<Neo4jRelation>), Error> {
    let graph_query = query(
        r#"
        MATCH (u:User {id: $id})
        MATCH (u)-[*]-(n)
        WHERE n.embedding IS NOT NULL
        WITH n, n.embedding as vec1, $embedding as vec2
        WITH n, vec1, vec2,
            reduce(dot = 0.0, i IN range(0, size(vec1)-1) | dot + vec1[i] * vec2[i]) AS dotProduct,
            sqrt(reduce(norm1 = 0.0, i IN range(0, size(vec1)-1) | norm1 + vec1[i] * vec1[i])) AS norm1,
            sqrt(reduce(norm2 = 0.0, i IN range(0, size(vec2)-1) | norm2 + vec2[i] * vec2[i])) AS norm2
        WITH n, dotProduct / (norm1 * norm2) AS score
        WHERE score > $threshold
        MATCH (n)-[r]-(m)
        RETURN DISTINCT n, r as rel, m, score
        ORDER BY score DESC
        "#
    );

    let (entities, relations) = graph.parse_query_result(
        graph_query
            .param("id", user_id)
            .param("embedding", search_query_embedding)
            .param("threshold", threshold)
    )
    .await?;
        
    Ok((entities, relations))
}