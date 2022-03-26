use actix_web::{
    HttpRequest,
    HttpResponse,
    Result,
    web,
    post,
    http,
    error,
};
use crate::{
    handler,
    config,
    structs::{
        WirelessNetworkParam,
        StaticWiredNetworkParam,
    },
};

#[post("/private/api/settings/wirelessnetwork")]
pub async fn post_wireless_network_settings(req: HttpRequest, wirelessnetworkparam: web::Json<WirelessNetworkParam>) -> Result<HttpResponse> {
    
    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let deserial_param: WirelessNetworkParam = WirelessNetworkParam {
        router_ip: wirelessnetworkparam.router_ip.clone(),
        netmask: wirelessnetworkparam.netmask.clone(),
        range_start: wirelessnetworkparam.range_start.clone(),
        range_end: wirelessnetworkparam.range_end.clone(),
        default_lease: wirelessnetworkparam.default_lease.clone(),
        max_lease: wirelessnetworkparam.max_lease.clone(),
        dns: wirelessnetworkparam.dns.clone(),
        timezone: wirelessnetworkparam.timezone.clone(),
    };

    let (
        write_default_named_status, 
        move_default_named_status
    ) = config::config_named(password.as_ref());

    let (
        write_networkd_status, 
        write_named_status, 
        move_networkd_status, 
        move_named_status, 
        restart_networkd_status, 
        restart_named_service
    ) = config::config_systemd_networkd_wireless(password.as_ref(), deserial_param);

    match 
        write_default_named_status && 
        move_default_named_status && 
        write_networkd_status && 
        write_named_status && 
        move_networkd_status && 
        move_named_status && 
        restart_networkd_status && 
        restart_named_service 
    {
        true => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        false => Err(error::ErrorUnauthorized("file_ops_error"))    
    }
}

#[post("/private/api/settings/wirednetwork/static")]
pub async fn post_static_wired_network(req: HttpRequest, staticwirednetworkparam: web::Json<StaticWiredNetworkParam>) -> Result<HttpResponse> {
    
    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let deserialparam = StaticWiredNetworkParam {
        internet_ip: staticwirednetworkparam.internet_ip.clone(),
        netmask: staticwirednetworkparam.netmask.clone(),
        gateway: staticwirednetworkparam.gateway.clone(),
        dns: staticwirednetworkparam.dns.clone(),
    };

    let (
        write_networkd_status, 
        move_networkd_status, 
        restart_networkd_status
    ) = config::config_systemd_networkd_wired_static(password.as_ref(), deserialparam);

    match  write_networkd_status && move_networkd_status && restart_networkd_status {
        true => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        false => Err(error::ErrorUnauthorized("file_ops_error"))
    }
}

#[post("/private/api/settings/wirednetwork/dynamic")]
pub async fn post_dynamic_wired_network(req: HttpRequest) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let (
        write_networkd_status, 
        move_networkd_status, 
        restart_networkd_status
    ) = config::config_systemd_networkd_wired_dynamic(password.as_ref());

    match  write_networkd_status && move_networkd_status && restart_networkd_status {
        true => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        false => Err(error::ErrorUnauthorized("file_ops_error"))
    }
}

