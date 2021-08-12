use actix_web::{
    HttpRequest,
    HttpResponse,
    Result,
    web,
    post,
};
use crate::{
    db,
    security,
    tool,
    config,
    structs::{
        WirelessNetworkParam,
        HttpResponseCustom,
        StaticWiredNetworkParam,
    },
};

#[post("/private/api/settings/wirelessnetwork")]
pub async fn post_wireless_network_settings(req: HttpRequest, wirelessnetworkparam: web::Json<WirelessNetworkParam>) -> Result<HttpResponse> {
    
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
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
                ) = config::config_named();

                let (
                    write_networkd_status, 
                    write_named_status, 
                    move_networkd_status, 
                    move_named_status, 
                    restart_networkd_status, 
                    restart_named_service
                ) = config::config_systemd_networkd_wireless(deserial_param);

                if write_networkd_status && write_named_status && write_default_named_status{
                    if move_networkd_status && move_named_status && move_default_named_status{
                        if restart_networkd_status && restart_named_service {
                            Ok(
                                HttpResponse::Ok().json(
                                    HttpResponseCustom {
                                        operation_status: "Success".to_string(),
                                        reason: "".to_string(),
                                    }
                                )
                            )
                        }
                        else{
                            Ok(
                                HttpResponse::InternalServerError().json(
                                    HttpResponseCustom {
                                        operation_status: "Failed".to_string(),
                                        reason: "restart_service_error".to_string(),
                                    }
                                )
                            )
                        }
                    }
                    else {
                        Ok(
                            HttpResponse::InternalServerError().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: "move_file_error".to_string(),
                                }
                            )
                        )
                    }
                }
                else{
                    Ok(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "write_file_error".to_string(),
                            }
                        )
                    )
                }
            }
            else {
                db::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Gone().json(
                        HttpResponseCustom{
                            operation_status: "failed".to_string(),
                            reason: "token-timeout".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::Unauthorized().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "incorrect-token".to_string(),
                    }
                )
            )
        }
    }
    else{
        Ok(
            HttpResponse::Unauthorized().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/wirednetwork/static")]
pub async fn post_static_wired_network(req: HttpRequest, staticwirednetworkparam: web::Json<StaticWiredNetworkParam>) -> Result<HttpResponse> {
    
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, _password) = db::query_logindata();
            let password_status: bool = tool::comparedate(olddate);

            if password_status{
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
                ) = config::config_systemd_networkd_wired_static(deserialparam);

                if write_networkd_status{
                    if move_networkd_status {
                        if restart_networkd_status {
                            Ok(
                                HttpResponse::Ok().json(
                                    HttpResponseCustom {
                                        operation_status: "Success".to_string(),
                                        reason: "".to_string(),
                                    }
                                )
                            )
                        }
                        else{
                            Ok(
                                HttpResponse::InternalServerError().json(
                                    HttpResponseCustom {
                                        operation_status: "Failed".to_string(),
                                        reason: "restart_service_error".to_string(),
                                    }
                                )
                            )
                        }
                    }
                    else {
                        Ok(
                            HttpResponse::InternalServerError().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: "move_file_error".to_string(),
                                }
                            )
                        )
                    }
                }
                else{
                    Ok(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "write_file_error".to_string(),
                            }
                        )
                    )
                }
            }
            else {
                db::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Gone().json(
                        HttpResponseCustom{
                            operation_status: "failed".to_string(),
                            reason: "token-timeout".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::Unauthorized().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "incorrect-token".to_string(),
                    }
                )
            )
        }
    }
    else{
        Ok(
            HttpResponse::Unauthorized().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/wirednetwork/dynamic")]
pub async fn post_dynamic_wired_network(req: HttpRequest) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, _password) = db::query_logindata();
            let password_status: bool = tool::comparedate(olddate);

            if password_status{

                let (
                    write_networkd_status, 
                    move_networkd_status, 
                    restart_networkd_status
                ) = config::config_systemd_networkd_wired_dynamic();

                if write_networkd_status{
                    if move_networkd_status {
                        if restart_networkd_status {
                            Ok(
                                HttpResponse::Ok().json(
                                    HttpResponseCustom {
                                        operation_status: "Success".to_string(),
                                        reason: "".to_string(),
                                    }
                                )
                            )
                        }
                        else{
                            Ok(
                                HttpResponse::InternalServerError().json(
                                    HttpResponseCustom {
                                        operation_status: "Failed".to_string(),
                                        reason: "restart_service_error".to_string(),
                                    }
                                )
                            )
                        }
                    }
                    else {
                        Ok(
                            HttpResponse::InternalServerError().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: "move_file_error".to_string(),
                                }
                            )
                        )
                    }
                }
                else{
                    Ok(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "write_file_error".to_string(),
                            }
                        )
                    )
                }
            }
            else {
                db::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Gone().json(
                        HttpResponseCustom{
                            operation_status: "failed".to_string(),
                            reason: "token-timeout".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::Unauthorized().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "incorrect-token".to_string(),
                    }
                )
            )
        }
    }
    else{
        Ok(
            HttpResponse::Unauthorized().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}

