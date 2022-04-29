use crate::{
    db, handler, linux,
    structs::{LoginParam, PasswdParam},
};
use actix_web::{body, error, http, post, web, HttpRequest, HttpResponse, Result};

#[post("/private/api/user/login")]
pub async fn post_pam_login(logindata: web::Json<LoginParam>) -> Result<HttpResponse> {
    match db::users::login(logindata.username.as_ref(), logindata.password.as_ref()) {
        Ok(current_token) => Ok(HttpResponse::with_body(
            http::StatusCode::from_u16(200).unwrap(),
            body::BoxBody::new(current_token),
        )),
        Err(_) => Err(error::ErrorUnauthorized("wrong_username_or_password")),
    }
}

#[post("/private/api/user/logout")]
pub async fn post_logout(req: HttpRequest) -> Result<HttpResponse> {
    let token = match req.headers().get("AUTHORIZATION") {
        Some(token) => Ok(token.to_str().unwrap().split_whitespace().last().unwrap()),
        None => Err(error::ErrorUnauthorized("wrong_username_or_password")),
    }?;

    let claims = match db::users::extract_claims_from_token(&token) {
        Ok(claims) => Ok(claims),
        Err((code, message)) => match code {
            401 => Err(error::ErrorGone(message)),
            _ => Err(error::ErrorUnauthorized(message)),
        },
    }?;

    match db::users::logout(&claims) {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(err) => Err(error::ErrorUnauthorized(err)),
    }
}

#[post("/private/api/user/password")]
pub async fn post_reset_password(
    req: HttpRequest,
    passwdparam: web::Json<PasswdParam>,
) -> Result<HttpResponse> {
    let (username, _password) = handler::handle_validate_token_response(&req)?;

    let (code, output, error) = linux::passwd(
        &username,
        &passwdparam.old_password,
        &passwdparam.new_password,
    );
    match code {
        0 => Ok(HttpResponse::with_body(
            http::StatusCode::from_u16(200).unwrap(),
            body::BoxBody::new(output),
        )),
        _ => Err(error::ErrorInternalServerError(error)),
    }
}
