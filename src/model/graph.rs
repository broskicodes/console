use std::collections::HashMap;

use neo4rs::{EndNodeId, Node, StartNodeId, Type, Error};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum Neo4jNode {
    User(User),
    Interest(Interest),
    Goal(Goal),
    Motivation(Motivation),
    Task(Task),
    Date(Date),
}

impl Neo4jNode {
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

impl TryInto<Neo4jNode> for Node {
    type Error = Error;
    
    fn try_into(self) -> Result<Neo4jNode, Error> {
        let labels = self.labels();
        let node_type = *labels
            .first()
            .ok_or_else(|| Error::UnsupportedScheme("Node has no labels".to_string()))?;

        let entity = match node_type {
            "User" => {
                let user: User = self.to::<User>().map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::User(user)
            },
            "Interest" => {
                let interest: Interest = self.to::<Interest>().map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Interest(interest)
            },
            "Goal" => {
                let goal: Goal = self.to::<Goal>().map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Goal(goal)
            },
            "Motivation" => {
                let motivation: Motivation = self.to::<Motivation>().map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Motivation(motivation)
            },
            "Task" => {
                let task: Task = self.to::<Task>().map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Task(task)
            },
            "Date" => {
                let date: Date = self.to::<Date>().map_err(|e| Error::DeserializationError(e))?;
                Neo4jNode::Date(date)
            },
            _ => {
                return Err(Error::UnsupportedScheme(format!("Node type {} not supported", node_type)));
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

impl Neo4jGraph {
    pub fn to_context(&self) -> Result<String, anyhow::Error> {
        let mut context = String::new();

        for rel in &self.relations {
            let relation = &rel.rel.0;
            let src_node = self.nodes.get(&(rel.src_id.0 as i64))
                .ok_or_else(|| anyhow::anyhow!("Source node {} not found", rel.src_id.0))?;
            let dst_node = self.nodes.get(&(rel.dst_id.0 as i64))
                .ok_or_else(|| anyhow::anyhow!("Destination node {} not found", rel.dst_id.0))?;

            context.push_str(&format!("{} - {} -> {}\n", src_node.to_context(), relation, dst_node.to_context()));
        }

        Ok(context)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    user_id: String,
}

impl User {
    pub fn to_context(&self) -> String {
        format!("User: {}", self.user_id)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Interest {
    name: String,
}

impl Interest {
    pub fn to_context(&self) -> String {
        format!("Interest: {}", self.name)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Goal {
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

#[derive(Debug, Clone, Deserialize)]
pub struct Motivation {
    title: String,
    reason: String,
}

impl Motivation {
    pub fn to_context(&self) -> String {
        format!("Motivation: {} with reason {}", self.title, self.reason)
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct Task {
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

#[derive(Debug, Clone, Deserialize)]
pub struct Date {
    day: u8,
    month: u8,
    year: u16,
}

impl Date {
    pub fn to_context(&self) -> String {
        format!("Date: {} of {}, {}", self.day, self.month, self.year)
    }
}