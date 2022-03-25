use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    HttpRequest,
};
use crate::{
    handler,
    config, 
    structs::SystemUpdateResponse, 
};

#[get("/private/api/settings/update/status")]
pub async fn get_content_server_update(req: HttpRequest) -> Result<HttpResponse> {

    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    Ok(
        HttpResponse::Ok().json(
            SystemUpdateResponse {
                update_list: config::update::display_new_update_lists()
            } 
        )
    )

}
