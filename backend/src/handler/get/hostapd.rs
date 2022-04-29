use crate::{db, handler, structs::HostapdParam};
use actix_web::{get, HttpRequest, HttpResponse, Result};

#[get("/private/api/settings/hostapd/status")]
pub async fn get_wifipage(req: HttpRequest) -> Result<HttpResponse> {
    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let (
        read_ssid,
        read_hide_ssid,
        read_wpa,
        read_hw_mode,
        read_channel,
        read_passphrase,
        read_hw_n_mode,
        read_qos,
    ) = db::hostapd::read_hostapd();

    Ok(HttpResponse::Ok().json(HostapdParam {
        ssid: read_ssid,
        hide_ssid: read_hide_ssid,
        wpa: read_wpa,
        hw_mode: read_hw_mode,
        channel: read_channel,
        passphrase: read_passphrase,
        hw_n_mode: read_hw_n_mode,
        qos: read_qos,
    }))
}
