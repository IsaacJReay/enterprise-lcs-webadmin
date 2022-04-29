// use std::io::Write;
use actix_web::{
    error,
    http,
    // web,
    post,
    HttpRequest,
    HttpResponse,
    Result,
};
// use futures::{
//     StreamExt,
//     TryStreamExt,
// };
use crate::{
    config,
    db,
    handler,
    // security,
    linux,
    structs::{
        DnsZonesInfo,
        // RestoreParam,
        HostapdParam,
        PasswdParam,
        WirelessNetworkParam,
    },
};

#[post("/private/api/settings/reset")]
pub async fn post_settings_reset(req: HttpRequest) -> Result<HttpResponse> {
    let (username, password) = handler::handle_validate_token_response(&req)?;
    let hostapdparam: HostapdParam = HostapdParam::default();
    let wirelessnetworkparam: WirelessNetworkParam = WirelessNetworkParam::default();
    let passwdparam: PasswdParam = PasswdParam::default(password.as_ref());

    match config::config_systemd_networkd_wireless(password.as_ref(), wirelessnetworkparam) {
        Ok(()) => Ok(()),
        Err(error) => Err(error::ErrorInternalServerError(error)),
    }?;
    match config::config_systemd_networkd_wired_dynamic(password.as_ref()) {
        Ok(()) => Ok(()),
        Err(error) => Err(error::ErrorInternalServerError(error)),
    }?;
    match config::config_hostapd(password.as_ref(), hostapdparam) {
        Ok(()) => Ok(()),
        Err(error) => Err(error::ErrorInternalServerError(error)),
    }?;

    let internal_dns = DnsZonesInfo::default(None);
    let external_dns = DnsZonesInfo::default(Some(&db::systemdnetworkd::read_eth0().1));
    match config::named::handle_new_domain_name_and_record(password.as_ref(), internal_dns, true) {
        Ok(()) => Ok(()),
        Err(error) => Err(error::ErrorInternalServerError(error)),
    }?;
    match config::named::handle_new_domain_name_and_record(password.as_ref(), external_dns, false) {
        Ok(()) => Ok(()),
        Err(error) => Err(error::ErrorInternalServerError(error)),
    }?;

    match linux::passwd(
        username.as_str(),
        passwdparam.old_password.as_str(),
        passwdparam.new_password.as_str(),
    )
    .0
    {
        0 => Ok(()),
        _ => Err(error::ErrorInternalServerError("reset_password_error")),
    }?;
    Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap()))
}
