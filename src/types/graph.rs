use std::collections::HashMap;

use async_openai::{config::OpenAIConfig, Client};
use neo4rs::{EndNodeId, Labels, StartNodeId, Type};
use serde::{Deserialize, Serialize};

use crate::utils::get_embedding;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CypherQueries {
    pub queries: Vec<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphNode {
    pub id: String,
    pub label: String,
    #[serde(rename = "props")]
    pub properties: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphRelationship {
    #[serde(rename = "source")]
    pub source_id: String,
    #[serde(rename = "target")]
    pub target_id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphData {
    pub nodes: Vec<GraphNode>,
    pub relationships: Vec<GraphRelationship>,
}

impl GraphData {
    pub async fn into_queries(self, openai_client: &Client<OpenAIConfig>) -> Result<CypherQueries, anyhow::Error> {
        let mut node_queries: Vec<String> = vec![];
        
        for node in self.nodes {
            let embedding_content: Option<String> = match node.label.as_str() {
                "Interest" => Some(
                    node.properties
                        .get("name")
                        .ok_or_else(|| anyhow::anyhow!("Interest name not found"))?
                        .to_string(),
                ),
                "Goal" => Some(
                    node.properties
                        .get("description")
                        .ok_or_else(|| anyhow::anyhow!("Goal description not found"))?
                        .to_string(),
                ),
                "Motivation" => {
                    let title = node.properties
                        .get("title")
                        .ok_or_else(|| anyhow::anyhow!("Motivation title not found"))?
                        .to_string();
                    let reason = node.properties
                        .get("reason")
                        .ok_or_else(|| anyhow::anyhow!("Motivation reason not found"))?
                        .to_string();

                    Some(format!("{}. {}", title, reason))
                },
                "Task" => Some(
                    node.properties
                        .get("action")
                        .ok_or_else(|| anyhow::anyhow!("Task action not found"))?
                        .to_string(),
                ),
                "Date" => {
                    let day = node.properties
                        .get("day")
                        .ok_or_else(|| anyhow::anyhow!("Date day not found"))?
                        .to_string();
                    let month = node.properties
                        .get("month")
                        .ok_or_else(|| anyhow::anyhow!("Date month not found"))?
                        .to_string();
                    let year = node.properties
                        .get("year")
                        .ok_or_else(|| anyhow::anyhow!("Date year not found"))?
                        .to_string();

                    Some(format!("Today is the {} of {}, {}.", day, month, year))
                },
                _ => None,
            };

            if let Some(embedding_content) = embedding_content {
                let embedding = get_embedding(openai_client, embedding_content).await?;

                node_queries.push(format!(
                    "CREATE ({}:{} {{ id: \"{}\", embedding: {:?}, {} }})", 
                    node.id, 
                    node.label, 
                    node.id, 
                    embedding,
                    node.properties
                        .into_iter()
                        .map(|(k, v) | format!("{}: {}", k, v))
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            } else {
                node_queries.push(format!(
                    "CREATE ({}:{} {{ id: \"{}\", {} }})", 
                    node.id, 
                    node.label, 
                    node.id, 
                    node.properties
                        .into_iter()
                        .map(|(k, v) | format!("{}: {}", k, v))
                        .collect::<Vec<String>>()
                        .join(", ")
                ));
            }
        }

        let rel_queries = self.relationships.into_iter().map(|rel| {
            format!(
                r#"
                MATCH (n), (m)
                WHERE n.id = "{}" AND m.id = "{}"
                CREATE (n)-[:{}]->(m)
                "#, 
                rel.source_id,
                rel.target_id,
                rel.label
            )
        })
        .collect::<Vec<String>>();

        Ok(CypherQueries {
            queries: node_queries.into_iter().chain(rel_queries).collect(),
        })
    }
}

#[derive(Debug, Clone, Deserialize)]
pub enum Neo4jNode {
    User(User),
    Interest(Interest),
    Goal(Goal),
    Motivation(Motivation),
    Task(Task),
    Date(Date),
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct Neo4jRelation {
    #[serde(rename = "start_node_id")]
    src_id: StartNodeId,
    #[serde(rename = "end_node_id")]
    dst_id: EndNodeId,
    #[serde(rename = "typ")]
    rel: Type,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    labels: Labels,
    user_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Interest {
    labels: Labels,
    name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Goal {
    labels: Labels,
    description: String,
    timeframe: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Motivation {
    labels: Labels,
    title: String,
    reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    labels: Labels,
    action: String,
    status: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Date {
    labels: Labels,
    day: u8,
    month: u8,
    year: u16,
}