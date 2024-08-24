use std::collections::{HashMap, HashSet};

use neo4rs::{query, Error, Graph, Node, Query};
use tracing::info;

use crate::types::{Date, Goal, Interest, Motivation, Neo4jNode, Neo4jRelation, Task, User};

pub fn classify_graph_node(node: &Node) -> Result<Neo4jNode, Error> {
    let labels = node.labels();
    let node_type = *labels
        .first()
        .ok_or_else(|| Error::UnsupportedScheme("Node has no labels".to_string()))?;

    let entity = match node_type {
        "User" => {
            let user: User = node.to::<User>().map_err(|e| Error::DeserializationError(e))?;
            Neo4jNode::User(user)
        },
        "Interest" => {
            let interest: Interest = node.to::<Interest>().map_err(|e| Error::DeserializationError(e))?;
            Neo4jNode::Interest(interest)
        },
        "Goal" => {
            let goal: Goal = node.to::<Goal>().map_err(|e| Error::DeserializationError(e))?;
            Neo4jNode::Goal(goal)
        },
        "Motivation" => {
            let motivation: Motivation = node.to::<Motivation>().map_err(|e| Error::DeserializationError(e))?;
            Neo4jNode::Motivation(motivation)
        },
        "Task" => {
            let task: Task = node.to::<Task>().map_err(|e| Error::DeserializationError(e))?;
            Neo4jNode::Task(task)
        },
        "Date" => {
            let date: Date = node.to::<Date>().map_err(|e| Error::DeserializationError(e))?;
            Neo4jNode::Date(date)
        },
        _ => {
            return Err(Error::UnsupportedScheme(format!("Node type {} not supported", node_type)));
        }
    };

    Ok(entity)
}

pub async fn run_graph_queries(graph: &Graph, queries: Vec<String>) -> Result<(), Error> {
    let mut txn = graph.start_txn().await?;
    let _ = txn.run_queries(
        queries
            .iter()
            .map(|q| query(q))
            .collect::<Vec<Query>>()
    )
    .await?;

    txn.commit().await?;

    Ok(())
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

    let mut result = graph.execute(
        graph_query
            .param("id", user_id)
            .param("embedding", search_query_embedding)
            .param("threshold", threshold)
    ).await?;
        
    let mut entities: HashMap<i64, Neo4jNode> = HashMap::new();
    let mut relations: HashSet<Neo4jRelation> = HashSet::new();

    let mut count = 0;
    while let Some(record) = result.next().await? {
        // let score = record.get::<f32>("score").map_err(|e| Error::DeserializationError(e))?;
        // info!("score: {:?}", score);
        let src_node: Node = record.get("n").map_err(|e| Error::DeserializationError(e))?;
        let dst_node: Node = record.get("m").map_err(|e| Error::DeserializationError(e))?;
        let relation: Neo4jRelation = record.get("rel").map_err(|e| Error::DeserializationError(e))?;

        let src_entity = classify_graph_node(&src_node)?;
        let dst_entity = classify_graph_node(&dst_node)?;
        
        let src_id = src_node.id();
        let dst_id = dst_node.id();

        if !entities.contains_key(&src_id) {
            entities.insert(src_id, src_entity.clone());
        }
        if !entities.contains_key(&dst_id) {
            entities.insert(dst_id, dst_entity.clone());
        }

        relations.insert(relation);
        count += 1;
    }

    info!("{} rows returned. {} entities, {} relations", count, entities.len(), relations.len());

    Ok((entities, relations.into_iter().collect()))
}