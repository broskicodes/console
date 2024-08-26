use neo4rs::{EndNodeId, Labels, Node, StartNodeId, Type, Error};
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