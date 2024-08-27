use std::sync::Arc;

use tracing::info;
use uuid::Uuid;

use crate::model::Message;
use crate::types::{CypherQueries, GraphData, ToolPrompts};
use crate::utils::config::Convinience;
use crate::utils::{
    config::{AppState, Parsable},
    constants::{GRAPH_DATA_DEF, GRAPH_SCHEMA},
};

pub async fn create_knowledge_from_chat(
    app_state: Arc<AppState>,
    user_id: Uuid,
    chat_id: Uuid,
) -> Result<String, anyhow::Error> {
    let messages = Message::get_all_messages_for_chat(&app_state.pool, chat_id.clone()).await?;

    let interview = messages
        .iter()
        .map(|m| format!("{}: {}", m.role, m.content))
        .collect::<Vec<String>>()
        .join("\n");

    let content = app_state
        .openai_client
        .get_tool_response(
            ToolPrompts::ExtractEntities
                .prompt_template()
                .replace("{interview}", &interview)
                .replace("{graph_schema}", GRAPH_SCHEMA)
                .replace("{graph_data}", GRAPH_DATA_DEF)
                .replace(
                    "{date}",
                    &chrono::Local::now().format("%B %d, %Y").to_string(),
                ),
        )
        .await?;

    info!("Generated AI response.");

    let new_graph_data: GraphData = serde_json::from_str(&content)?;
    let old_graph_data: GraphData = app_state.graph.get_full_graph(&user_id).await?.try_into()?;

    let queries: CypherQueries = match (
        old_graph_data.nodes.len(),
        old_graph_data.relationships.len(),
    ) {
        (0, 0) => {
            new_graph_data
                .clone()
                .into_queries(&user_id, &app_state.openai_client)
                .await?
        }
        _ => {
            info!("Merging graphs.");

            let content = app_state
                .openai_client
                .get_tool_response(
                    ToolPrompts::MergeGraph
                        .prompt_template()
                        .replace("{graph_schema}", GRAPH_SCHEMA)
                        .replace("{existing_graph}", &serde_json::to_string(&old_graph_data)?)
                        .replace(
                            "{new_graph}",
                            &serde_json::to_string(&new_graph_data.clone())?,
                        )
                        .replace(
                            "{date}",
                            &chrono::Local::now().format("%B %d, %Y").to_string(),
                        ),
                )
                .await?;

            // info!("content: {}", content);

            let graph_data: GraphData = serde_json::from_str(&content)?;
            graph_data
                .into_queries(&user_id, &app_state.openai_client)
                .await?
        }
    };

    info!("Generated {} Cypher queries.", queries.queries.len());

    app_state.graph.run_queries(queries.queries).await?;

    info!("Knowledge graph created.");

    Ok(content)
}
