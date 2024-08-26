use std::{collections::{HashMap, HashSet}, future::Future};

use async_openai::{config::OpenAIConfig, types::CreateEmbeddingRequestArgs, Client};
use neo4rs::{query, Graph, Node, Query, Error};
use serde::Deserialize;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tracing::info;

use crate::model::{Neo4jNode, Neo4jRelation};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub graph: Graph,
    pub openai_client: Client<OpenAIConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppEnv {
    pub database_url: String,
    pub openai_api_key: String,
    pub neo4j_uri: String,
    pub neo4j_password: String,
}

impl AppEnv {
    pub fn new(secret_store: &SecretStore) -> Result<Self, anyhow::Error> {
        Ok(AppEnv {
            database_url: secret_store.get("DATABASE_URL").ok_or_else(|| {
                anyhow::anyhow!("DATABASE_URL is not set")
            })?,
            openai_api_key: secret_store.get("OPENAI_API_KEY").ok_or_else(|| {
                anyhow::anyhow!("OPENAI_API_KEY is not set")
            })?,
            neo4j_uri: secret_store.get("NEO4J_URI").ok_or_else(|| {
                anyhow::anyhow!("NEO4J_URI is not set")
            })?,
            neo4j_password: secret_store.get("NEO4J_PASSWORD").ok_or_else(|| {
                anyhow::anyhow!("NEO4J_PASSWORD is not set")
            })?,
        })
    }
}

pub trait Parsable {
    fn run_queries(&self, queries: Vec<String>) -> impl Future<Output = Result<(), Error>>;
    fn parse_query_result(&self, query: Query) -> impl Future<Output = Result<(HashMap<i64, Neo4jNode>, Vec<Neo4jRelation>), Error>>;
}

impl Parsable for Graph {
    async fn run_queries(&self, queries: Vec<String>) -> Result<(), Error> {
        let mut txn = self.start_txn().await?;
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

    async fn parse_query_result(&self, query: Query) -> Result<(HashMap<i64, Neo4jNode>, Vec<Neo4jRelation>), Error> {
        let mut result = self.execute(query).await?;

        let mut entities: HashMap<i64, Neo4jNode> = HashMap::new();
        let mut relations: HashSet<Neo4jRelation> = HashSet::new();

        let mut count = 0;
        while let Some(record) = result.next().await? {
            // let score = record.get::<f32>("score").map_err(|e| Error::DeserializationError(e))?;
            // info!("score: {:?}", score);
            let src_node: Node = record.get("n").map_err(|e| Error::DeserializationError(e))?;
            let dst_node: Node = record.get("m").map_err(|e| Error::DeserializationError(e))?;
            let relation: Neo4jRelation = record.get("rel").map_err(|e| Error::DeserializationError(e))?;
            
            let src_id = src_node.id();
            let dst_id = dst_node.id();

            let src_entity: Neo4jNode = src_node.clone().try_into()?;
            let dst_entity: Neo4jNode = dst_node.clone().try_into()?;

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
}

pub trait Convinience {
    fn get_embedding(&self, content: String) -> impl Future<Output = Result<Vec<f32>, anyhow::Error>>;
}

impl Convinience for Client<OpenAIConfig> {
    async fn get_embedding(&self, content: String) -> Result<Vec<f32>, anyhow::Error> {
        let request = CreateEmbeddingRequestArgs::default()
            .model("text-embedding-3-small")
            .dimensions(384 as u32)
            .input(content.clone())
            .build()?;

        let embedding = self
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
}