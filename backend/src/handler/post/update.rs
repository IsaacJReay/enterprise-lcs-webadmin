use actix_web::{
    web,
    post,
    Result,
    HttpResponse,
    HttpRequest,
    http,
    error
};
use fork::{daemon, Fork};
use crate::{
    handler,
    config::{
        write_file,
        update::update_content_server,
    },
    structs::SystemUpdateRequest
};

#[post("/private/api/settings/update/update")]
pub async fn post_update_content_server(req: HttpRequest, update_request_struct: web::Json<SystemUpdateRequest>) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;
    match std::path::Path::new("/tmp/update_db.lock").exists() {
        true => Err(error::ErrorInternalServerError("Another update is in progress")),
        false => {
            write_file(" ".as_bytes(), "/tmp/update_db.lock");
            if let Ok(Fork::Child) = daemon(false, false) {
                update_content_server(&password, &update_request_struct.id, update_request_struct.sys_update);
            }
            Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap()))
        }
    }
}
