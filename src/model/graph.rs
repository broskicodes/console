use std::collections::HashMap;

use neo4rs::{EndNodeId, Error, Node, StartNodeId, Type};
use serde::Deserialize;
use serde_json::json;

use crate::types::{GraphData, GraphNode, GraphRelationship};

#[derive(Debug, Clone, Deserialize)]
pub enum Neo4jNode {
    User(UserNode),
    Interest(Interest),
    Goal(Goal),
    Motivation(Motivation),
    Task(Task),
    Date(Date),
}

impl Neo4jNode {
    pub fn id(&self) -> String {
        match self {
            Neo4jNode::User(user) => user.id.clone(),
            Neo4jNode::Interest(interest) => interest.id.clone(),
            Neo4jNode::Goal(goal) => goal.id.clone(),
            Neo4jNode::Motivation(motivation) => motivation.id.clone(),
            Neo4jNode::Task(task) => task.id.clone(),
            Neo4jNode::Date(date) => date.id.clone(),
        }
    }

    pub fn to_context(&self) -> String {
        match self {
            Neo4jNode::User(user) => user.to_context(),
            Neo4jNode::Interest(interest) => interest.to_context(),
            Neo4jNode::Goal(goal) => goal.to_context(),
            Neo4jNode::Motivation(motivation) => motivation.to_context(),
            Neo4jNode::Task(task) => task.to_context(),
            Neo4jNode::Date(date) => date.to_context(),
        }
    }
}

impl Into<GraphNode> for Neo4jNode {
    fn into(self) -> GraphNode {
        match self {
            Neo4jNode::User(user) => user.into(),
            Neo4jNode::Interest(interest) => interest.into(),
            Neo4jNode::Goal(goal) => goal.into(),
            Neo4jNode::Motivation(motivation) => motivation.into(),
            Neo4jNode::Task(task) => task.into(),
            Neo4jNode::Date(date) => date.into(),
        }
    }
}

impl TryInto<Neo4jNode> for Node {
    type Error = Error;

    fn try_into(self) -> Result<Neo4jNode, Error> {
        let labels = self.labels();
        let node_type = *labels
            .first()
            .ok_or_else(|| Error::UnsupportedScheme("Node has no labels".to_string()))?;

        let entity = match node_type {
            "User" => {
                let user: UserNode = self
                    .to::<UserNode>()
                    .map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::User(user)
            }
            "Interest" => {
                let interest: Interest = self
                    .to::<Interest>()
                    .map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Interest(interest)
            }
            "Goal" => {
                let goal: Goal = self
                    .to::<Goal>()
                    .map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Goal(goal)
            }
            "Motivation" => {
                let motivation: Motivation = self
                    .to::<Motivation>()
                    .map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Motivation(motivation)
            }
            "Task" => {
                let task: Task = self
                    .to::<Task>()
                    .map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Task(task)
            }
            "Date" => {
                let date: Date = self
                    .to::<Date>()
                    .map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Date(date)
            }
            _ => {
                return Err(Error::UnsupportedScheme(format!(
                    "Node type {} not supported",
                    node_type
                )));
            }
        };

        Ok(entity)
    }
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
pub struct Neo4jGraph {
    pub nodes: HashMap<i64, Neo4jNode>,
    pub relations: Vec<Neo4jRelation>,
}

impl TryInto<GraphData> for Neo4jGraph {
    type Error = Error;

    fn try_into(self) -> Result<GraphData, Error> {
        let nodes: Vec<GraphNode> = self
            .nodes
            .clone()
            .into_iter()
            .map(|(_, node)| node.into())
            .collect();
        let relationships: Vec<GraphRelationship> = self
            .relations
            .into_iter()
            .map(|rel| -> Result<GraphRelationship, Error> {
                let src_node = self.nodes.get(&(rel.src_id.0 as i64)).ok_or_else(|| {
                    Error::UnsupportedScheme(format!("Source node {} not found", rel.src_id.0))
                })?;
                let dst_node = self.nodes.get(&(rel.dst_id.0 as i64)).ok_or_else(|| {
                    Error::UnsupportedScheme(format!("Destination node {} not found", rel.dst_id.0))
                })?;

                Ok(GraphRelationship {
                    source_id: src_node.id(),
                    target_id: dst_node.id(),
                    label: rel.rel.0.to_string(),
                })
            })
            .collect::<Result<Vec<GraphRelationship>, Error>>()?;

        Ok(GraphData {
            nodes,
            relationships,
        })
    }
}

impl Neo4jGraph {
    pub fn to_context(&self) -> Result<String, anyhow::Error> {
        let mut context = String::new();

        for rel in &self.relations {
            let relation = &rel.rel.0;
            let src_node = self
                .nodes
                .get(&(rel.src_id.0 as i64))
                .ok_or_else(|| anyhow::anyhow!("Source node {} not found", rel.src_id.0))?;
            let dst_node = self
                .nodes
                .get(&(rel.dst_id.0 as i64))
                .ok_or_else(|| anyhow::anyhow!("Destination node {} not found", rel.dst_id.0))?;

            context.push_str(&format!(
                "{} - {} -> {}\n",
                src_node.to_context(),
                relation,
                dst_node.to_context()
            ));
        }

        Ok(context)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserNode {
    id: String,
    user_id: String,
}

impl UserNode {
    pub fn to_context(&self) -> String {
        format!("User: {}", self.user_id)
    }
}

impl Into<GraphNode> for UserNode {
    fn into(self) -> GraphNode {
        GraphNode {
            id: self.id,
            label: "User".to_string(),
            properties: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Interest {
    id: String,
    name: String,
}

impl Interest {
    pub fn to_context(&self) -> String {
        format!("Interest: {}", self.name)
    }
}

impl Into<GraphNode> for Interest {
    fn into(self) -> GraphNode {
        GraphNode {
            id: self.id,
            label: "Interest".to_string(),
            properties: HashMap::from([("name".to_string(), json!(self.name))]),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Goal {
    id: String,
    description: String,
    timeframe: Option<String>,
}

impl Goal {
    pub fn to_context(&self) -> String {
        match &self.timeframe {
            Some(timeframe) => format!("Goal: {} with timeframe {}", self.description, timeframe),
            None => format!("Goal: {}", self.description),
        }
    }
}

impl Into<GraphNode> for Goal {
    fn into(self) -> GraphNode {
        GraphNode {
            id: self.id,
            label: "Goal".to_string(),
            properties: HashMap::from([
                ("description".to_string(), json!(self.description)),
                ("timeframe".to_string(), json!(self.timeframe)),
            ]),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Motivation {
    id: String,
    title: String,
    reason: String,
}

impl Motivation {
    pub fn to_context(&self) -> String {
        format!("Motivation: {} with reason {}", self.title, self.reason)
    }
}

impl Into<GraphNode> for Motivation {
    fn into(self) -> GraphNode {
        GraphNode {
            id: self.id,
            label: "Motivation".to_string(),
            properties: HashMap::from([
                ("title".to_string(), json!(self.title)),
                ("reason".to_string(), json!(self.reason)),
            ]),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Task {
    id: String,
    action: String,
    status: Option<String>,
}

impl Task {
    pub fn to_context(&self) -> String {
        match &self.status {
            Some(status) => format!("Task: {} with status {}", self.action, status),
            None => format!("Task: {}", self.action),
        }
    }
}

impl Into<GraphNode> for Task {
    fn into(self) -> GraphNode {
        GraphNode {
            id: self.id,
            label: "Task".to_string(),
            properties: HashMap::from([
                ("action".to_string(), json!(self.action)),
                ("status".to_string(), json!(self.status)),
            ]),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Date {
    id: String,
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    pub fn to_context(&self) -> String {
        format!("Date: {} of {}, {}", self.day, self.month, self.year)
    }
}

impl Into<GraphNode> for Date {
    fn into(self) -> GraphNode {
        GraphNode {
            id: self.id,
            label: "Date".to_string(),
            properties: HashMap::from([
                ("day".to_string(), json!(self.day)),
                ("month".to_string(), json!(self.month)),
                ("year".to_string(), json!(self.year)),
            ]),
        }
    }
}
