use std::sync::Arc;

use async_openai::types::{
    ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
    ChatCompletionResponseFormat, ChatCompletionResponseFormatType,
    CreateChatCompletionRequestArgs,
};
use tracing::info;
use uuid::Uuid;

use crate::model::Message;
use crate::types::{CypherQueries, GraphData, ToolPrompts};
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

    let client = app_state.openai_client.clone();

    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4o")
        .response_format(ChatCompletionResponseFormat {
            r#type: ChatCompletionResponseFormatType::JsonObject,
        })
        .messages(vec![ChatCompletionRequestMessage::System(
            ChatCompletionRequestSystemMessageArgs::default()
                .content(
                    ToolPrompts::ExtractEntities
                        .prompt_template()
                        .replace("{interview}", &interview)
                        .replace("{graph_schema}", GRAPH_SCHEMA)
                        .replace("{graph_data}", GRAPH_DATA_DEF)
                        .replace(
                            "{date}",
                            &chrono::Local::now().format("%B %d, %Y").to_string(),
                        )
                        .replace("{user_id}", &user_id.to_string()),
                )
                .build()?,
        )])
        .build()?;

    let response = client.chat().create(request).await?;
    let content = response.choices[0]
        .message
        .content
        .clone()
        .ok_or(anyhow::anyhow!("No content in AI response"))?;

    info!("Generated AI response.");

    let graph_data: GraphData = serde_json::from_str(&content)?;
    let queries: CypherQueries = graph_data.into_queries(&app_state.openai_client).await?;

    info!("Generated {} Cypher queries.", queries.queries.len());

    app_state.graph.run_queries(queries.queries).await?;

    info!("Knowledge graph created.");

    Ok(content)
}
