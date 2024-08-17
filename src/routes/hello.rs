use actix_web::{get, web, Error};
use tracing::{info, warn};

#[get("")]
async fn hello() -> Result<web::Json<String>, Error> {
    info!("saying hello");
    Ok(web::Json("Welcome to the console, Buddy!".to_string()))
}