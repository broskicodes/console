use std::future::{ready, Ready};

use actix_web::{body::MessageBody, dev::{Payload, ServiceRequest, ServiceResponse}, error::{ErrorInternalServerError, ErrorUnauthorized}, middleware::Next, web::Data, Error, FromRequest, HttpMessage, HttpRequest};
use uuid::Uuid;

use crate::{model::User, utils::config::AppState};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
}

impl FromRequest for AuthenticatedUser {
    type Error = actix_web::Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        if let Some(user) = req.extensions().get::<AuthenticatedUser>() {
            ready(Ok(user.clone()))
        } else {
            ready(Err(ErrorUnauthorized("Authenticated user not found")))
        }
    }
}

pub async fn authenticate_user(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let user_id = req
        .headers()
        .get("user-id")
        .and_then(|h| h.to_str().ok())
        .map(String::from);

    if let Some(user_id) = user_id {
        let uuid: Uuid = Uuid::try_parse(&user_id)
            .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;

        if let Some(app_state) = req.app_data::<Data<AppState>>() {
            let pool = app_state.pool.clone();
            User::get_or_create(&pool, uuid)
                .await
                .map_err(|e| Error::from(ErrorInternalServerError(e.to_string())))?;
        }

        req.extensions_mut().insert(AuthenticatedUser { 
            user_id: uuid
        });
    } else {
        return Err(Error::from(ErrorUnauthorized("User ID is required")));
    }

    let res = next.call(req).await?;
    Ok(res)
}