use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    error,
    HttpRequest,
};
use crate::{
    handler,
    config
};

#[get("/private/api/settings/update/status")]
pub async fn get_content_server_update(req: HttpRequest) -> Result<HttpResponse> {

    let (_username, _password) = handler::handle_validate_token_response(&req)?;
    match actix_web::rt::task::spawn_blocking(| | config::update::display_new_update_lists()).await.unwrap() {
        Ok(vec_updatable) => Ok(HttpResponse::Ok().json(vec_updatable)),
        Err(err) => Err(error::ErrorInternalServerError(err))
    }
}
