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

    let deserial_param: HostapdParam = HostapdParam {
        ssid: hostapdparam.ssid.clone(),
        hide_ssid: hostapdparam.hide_ssid,
        hw_mode: hostapdparam.hw_mode.clone(),
        channel: hostapdparam.channel.clone(),
        wpa: hostapdparam.wpa,
        passphrase: hostapdparam.passphrase.clone(),
        hw_n_mode: hostapdparam.hw_n_mode,
        qos: hostapdparam.qos,
    };

    let (
        write_hostapd_status,
        move_hostapd_status, 
        restart_hostapd_status
    ) = config::config_hostapd(password.as_ref(), deserial_param);

    match write_hostapd_status &&move_hostapd_status && restart_hostapd_status {
        true => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        false => Err(error::ErrorInternalServerError("file_operation_error"))
    }
}
