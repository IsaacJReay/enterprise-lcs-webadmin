use crate::{config, db, handler, structs::StatusPageResult};
use actix_web::{get, HttpRequest, HttpResponse, Result};

#[get("/private/api/settings/status")]
pub async fn get_statuspage(req: HttpRequest) -> Result<HttpResponse> {
    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let (ssid, _hide_ssid, _wpa, hw_mode, channel, _passphrase, hw_n_mode, qos) =
        db::hostapd::read_hostapd();
    let (eth0_macaddr, eth0_ipaddr, eth0_subnetmask, eth0_gateway) =
        db::systemdnetworkd::read_eth0();
    let (wlan0_macaddr, wlan0_ipaddr, wlan0_subnetmask) = db::systemdnetworkd::read_wlan0();
    let wlan0_dns = config::named::read_forward_dns_server();

    Ok(HttpResponse::Ok().json(StatusPageResult {
        wan_macaddress: eth0_macaddr,
        wan_ip: eth0_ipaddr,
        wan_netmask: eth0_subnetmask,
        wan_gateway: eth0_gateway,
        wlan_macaddress: wlan0_macaddr,
        wlan_ip: wlan0_ipaddr,
        wlan_netmask: wlan0_subnetmask,
        wlan_dns: wlan0_dns,
        wlan_ssid: ssid,
        wlan_hw_mode: hw_mode,
        wlan_channel: channel,
        wlan_hw_n_mode: hw_n_mode,
        wlan_qos: qos,
    }))
}
