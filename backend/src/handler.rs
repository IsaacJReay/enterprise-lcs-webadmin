pub mod delete;
pub mod get;
pub mod post;
pub mod put;

use actix_web::{Error, Result};

pub fn handle_validate_token_response(
    req: &actix_web::HttpRequest,
) -> Result<(String, String), Error> {
    match crate::db::users::validate_token(&req) {
        Ok((username, password)) => Ok((username, password)),
        Err((code, message)) => match code {
            401 => Err(actix_web::error::ErrorGone(message)),
            _ => Err(actix_web::error::ErrorUnauthorized(message)),
        },
    }
}
