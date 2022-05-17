use crate::{
    config::{update::update_content_server, write_file},
    handler,
    structs::SystemUpdateRequest,
};
use actix_web::{error, http, post, rt, web, HttpRequest, HttpResponse, Result};
// use fork::{daemon, Fork};

#[post("/private/api/settings/update/update")]
pub async fn post_update_content_server(
    req: HttpRequest,
    update_request_struct: web::Json<SystemUpdateRequest>,
) -> Result<HttpResponse> {
    let (_username, password) = handler::handle_validate_token_response(&req)?;
    match std::path::Path::new("/tmp/update_db.lock").exists() {
        true => Err(error::ErrorInternalServerError(
            "Another update is in progress",
        )),
        false => {
            write_file(" ".as_bytes(), "/tmp/update_db.lock");
            rt::task::spawn_blocking(move || {
                update_content_server(
                    &password,
                    &update_request_struct.id,
                    update_request_struct.sys_update,
                );
            });
            Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap()))
        }
    }
}
