use actix_web::{
    web,
    post,
    Result,
    HttpRequest,
    HttpResponse,
};
use crate::{
    db,
    handler,
    linux, 
    structs::{
        HttpResponseCustom, 
        LoginParam, 
        LoginResponse, 
        PasswdParam
    }
};

#[post("/private/api/user/login")]
pub async fn post_pam_login(logindata: web::Json<LoginParam>) -> Result<HttpResponse> {

    match db::users::login(logindata.username.as_ref(), logindata.password.as_ref()) {
        Ok(current_token) => Ok(
            HttpResponse::Ok().json(
                LoginResponse {
                    operation_status: "Success".to_string(),
                    token: current_token,
                }
            )
        ),
        Err(_) => Ok(
            HttpResponse::Unauthorized().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "wrong_username_or_password".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/user/logout")]
pub async fn post_logout(req: HttpRequest) -> Result<HttpResponse> {

    let token = match req.headers().get("AUTHORIZATION") {
        Some(token) => Ok(token.to_str().unwrap().split_whitespace().last().unwrap()),
        None => Err(
            HttpResponse::Unauthorized().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "wrong_username_or_password".to_string(),
                }
            )
        )
    }?;

    let claims = match db::users::extract_claims_from_token(&token) {
        Ok(claims) => Ok(claims),
        Err((code, message)) => match code {
            401 
            => Err(
                    HttpResponse::Gone().json(
                        HttpResponseCustom{
                            operation_status: "Failed".to_string(),
                            reason: message,
                        }
                    )
                ),
            _ 
            => Err(
                    HttpResponse::Unauthorized().json(
                        HttpResponseCustom{
                            operation_status: "Failed".to_string(),
                            reason: message,
                        }
                    )
                )
        }
    }?;

    match db::users::logout(&claims) {
        Ok(()) => Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Success".to_string(),
                    reason: "".to_string(),
                }
            )
        ),
        Err(_) => Ok(
            HttpResponse::Unauthorized().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "wrong_username_or_password".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/user/password")]
pub async fn post_reset_password(req: HttpRequest, passwdparam: web::Json<PasswdParam>) -> Result<HttpResponse> {

    let (username, _password) = handler::handle_validate_token_response(&req)?;

    let (code, _output, error) = linux::passwd(&username, &passwdparam.old_password, &passwdparam.new_password);
    if code == 0 {
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Success".to_string(),
                    reason: "".to_string(),
                }
            )
        )
    }
    else{
        Ok(
            HttpResponse::InternalServerError().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: error,
                }
            )
        )
    }
}
