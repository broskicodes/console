use actix_web::web::{self, ServiceConfig};
use actix_web::middleware::Logger;
use serde::Deserialize;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_runtime::SecretStore;
use sqlx::PgPool;
use tracing_actix_web::TracingLogger;

pub mod routes;

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AppEnv {
    database_url: String,
}

impl AppEnv {
    fn new(secret_store: &SecretStore) -> Result<Self, anyhow::Error> {
        Ok(AppEnv {
            database_url: secret_store.get("DATABASE_URL").ok_or_else(|| {
                anyhow::anyhow!("DATABASE_URL is not set")
            })?,
        })
    }
}

#[shuttle_runtime::main]
async fn actix_web(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_runtime::Secrets] secret_store: SecretStore,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let app_state = web::Data::new(AppState { pool });
    let app_env = AppEnv::new(&secret_store)?;

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("")
                .service(web::scope("/").service(routes::hello::hello))
                .wrap(Logger::default())
                .wrap(TracingLogger::default())
                .app_data(app_state)
                .app_data(app_env)
        );
    };

    Ok(config.into())
}