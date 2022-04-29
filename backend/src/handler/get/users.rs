use actix_web::{get, HttpRequest, HttpResponse, Result};

#[get("/private/api/user/query")]
pub async fn get_logindata(req: HttpRequest) -> Result<HttpResponse> {
    let (current_username, _password) = crate::handler::handle_validate_token_response(&req)?;

    Ok(HttpResponse::Ok().json(current_username))
}
