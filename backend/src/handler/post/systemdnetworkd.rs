use crate::{
    config, handler,
    structs::{StaticWiredNetworkParam, WirelessNetworkParam},
};
use actix_web::{error, http, post, web, HttpRequest, HttpResponse, Result};

#[post("/private/api/settings/wirelessnetwork")]
pub async fn post_wireless_network_settings(
    req: HttpRequest,
    wlanparam: web::Json<WirelessNetworkParam>,
) -> Result<HttpResponse> {
    let (_username, password) = handler::handle_validate_token_response(&req)?;
    match config::config_systemd_networkd_wireless(password.as_ref(), wlanparam.into_inner()) {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(error) => Err(error::ErrorInternalServerError(error)),
    }
}

#[post("/private/api/settings/wirednetwork/static")]
pub async fn post_static_wired_network(
    req: HttpRequest,
    s_wnetparam: web::Json<StaticWiredNetworkParam>,
) -> Result<HttpResponse> {
    let (_username, password) = handler::handle_validate_token_response(&req)?;

    match config::config_systemd_networkd_wired_static(password.as_ref(), s_wnetparam.into_inner())
    {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(error) => Err(error::ErrorInternalServerError(error)),
    }
}

#[post("/private/api/settings/wirednetwork/dynamic")]
pub async fn post_dynamic_wired_network(req: HttpRequest) -> Result<HttpResponse> {
    let (_username, password) = handler::handle_validate_token_response(&req)?;

    match config::config_systemd_networkd_wired_dynamic(password.as_ref()) {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}
