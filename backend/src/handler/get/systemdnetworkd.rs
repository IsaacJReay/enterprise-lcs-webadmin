use crate::{
    db, handler,
    structs::{StaticWiredNetworkParam, WanPageResult, WirelessNetworkParam},
};
use actix_web::{get, HttpRequest, HttpResponse, Result};

#[get("/private/api/settings/wirednetwork/status")]
pub async fn get_wanpage(req: HttpRequest) -> Result<HttpResponse> {
    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let (dhcp_status, read_internet_ip, read_netmask, read_gateway, read_dns) =
        db::systemdnetworkd::read_wan_networkd();
    Ok(HttpResponse::Ok().json(WanPageResult {
        dhcp: dhcp_status,
        wired_network_param: StaticWiredNetworkParam {
            internet_ip: read_internet_ip,
            netmask: read_netmask,
            gateway: read_gateway,
            dns: read_dns,
        },
    }))
}

#[get("/private/api/settings/wirelessnetwork/status")]
pub async fn get_wlanpage(req: HttpRequest) -> Result<HttpResponse> {
    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let (
        read_router_ip,
        read_netmask,
        read_range_start,
        read_range_end,
        read_dns,
        read_default_lease,
        read_max_lease,
        read_timezone,
    ) = db::systemdnetworkd::read_wlan_networkd();

    Ok(HttpResponse::Ok().json(WirelessNetworkParam {
        router_ip: read_router_ip,
        netmask: read_netmask,
        range_start: read_range_start,
        range_end: read_range_end,
        dns: read_dns,
        default_lease: read_default_lease,
        max_lease: read_max_lease,
        timezone: read_timezone,
    }))
}
