use actix_web::web::{self, ServiceConfig};
use actix_web::middleware::Logger;
use async_openai::config::OpenAIConfig;
use async_openai::Client;
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub mod routes;
pub mod types;
pub mod model;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
    openai_client: Client<OpenAIConfig>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppEnv {
    database_url: String,
    openai_api_key: String,
}

impl AppEnv {
    fn new(secret_store: &SecretStore) -> Result<Self, anyhow::Error> {
        Ok(AppEnv {
            database_url: secret_store.get("DATABASE_URL").ok_or_else(|| {
                anyhow::anyhow!("DATABASE_URL is not set")
            })?,
            openai_api_key: secret_store.get("OPENAI_API_KEY").ok_or_else(|| {
                anyhow::anyhow!("OPENAI_API_KEY is not set")
            })?,
        })
    }
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_env = AppEnv::new(&secret_store)?;
    let api_key = app_env.openai_api_key.clone();
    let config = OpenAIConfig::new()
        .with_api_key(api_key);

    let client = Client::with_config(config);
    let pool = PgPool::connect(&app_env.database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to connect to the database: {}", e))?;
    let app_state = web::Data::new(AppState { pool, openai_client: client });

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(web::scope("/").service(routes::hello::hello))
                .service(web::scope("/ai").service(routes::ai::send_message))
                .wrap(Logger::default())
                .wrap(TracingLogger::default())
                .app_data(app_state)
                .app_data(app_env)
        );
    };

    Ok(config.into())
}