use actix_web::middleware::{from_fn, Logger};
use actix_web::web::{self, ServiceConfig};
use async_openai::config::OpenAIConfig;
use async_openai::Client;
use neo4rs::{ConfigBuilder, Graph};
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;
use utils::config::{AppEnv, AppState};

pub mod middleware;
pub mod model;
pub mod routes;
pub mod types;
pub mod utils;

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_env = AppEnv::new(&secret_store)?;

    // init postgres db
    let pool = PgPool::connect(&app_env.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to the postgres database: {}", e))?;

    // init neo4j db
    let neo4j_config = ConfigBuilder::default()
        .uri(&app_env.neo4j_uri)
        .user("neo4j")
        .password(&app_env.neo4j_password)
        .db("neo4j")
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to connect to the neo4j database: {}", e))?;

    let graph = Graph::connect(neo4j_config).await.unwrap();

    // init openai client
    let api_key = app_env.openai_api_key.clone();
    let openai_config = OpenAIConfig::new().with_api_key(api_key);

    let client = Client::with_config(openai_config);

    let app_state = web::Data::new(AppState {
        pool,
        graph,
        openai_client: client,
    });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(web::scope("/").service(routes::hello::hello))
                .service(
                    web::scope("/ai")
                        .service(routes::ai::send_message)
                        .service(routes::ai::create_knowledge_graph)
                        .service(routes::ai::search_knowledge_graph),
                )
                .wrap(from_fn(middleware::auth::authenticate_user))
                .wrap(TracingLogger::default())
                .wrap(Logger::default())
                .app_data(app_state)
                .app_data(app_env),
        );
    };

    Ok(config.into())
}
