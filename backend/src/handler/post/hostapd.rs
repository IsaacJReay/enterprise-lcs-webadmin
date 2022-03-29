use actix_web::{
    post,
    web,
    http,
    error,
    HttpRequest,
    HttpResponse,
    Result,
};
use crate::{
    handler,
    config,
    structs::HostapdParam
};

#[post("/private/api/settings/hostapd")]
pub async fn post_hostapd_settings(req: HttpRequest, hostapdparam: web::Json<HostapdParam>) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;

    match config::config_hostapd(password.as_ref(), hostapdparam.into_inner()){
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(err) => Err(error::ErrorInternalServerError(err))
    }
}
