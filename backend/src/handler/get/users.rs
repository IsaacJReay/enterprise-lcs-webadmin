use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    HttpRequest,
};
use crate::{
    handler,
    structs::UserName
};

#[get("/private/api/user/query")]
pub async fn get_logindata(req: HttpRequest) -> Result<HttpResponse> {

    let (current_username, _password) = handler::handle_validate_token_response(&req)?;

    Ok(
        HttpResponse::Ok().json(
            UserName{
                username: current_username
            }
        )
    )
}