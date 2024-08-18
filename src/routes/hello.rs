use actix_web::{get, web, Error};

#[get("")]
async fn hello() -> Result<web::Json<String>, Error> {
    Ok(web::Json("Welcome to the console, Buddy!".to_string()))
}