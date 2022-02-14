use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    HttpRequest,
};
use crate::{
    db, 
    security, 
    tool,
    structs::{ 
        HttpResponseCustom,  
        StaticWiredNetworkParam,  
        WanPageResult, 
        WirelessNetworkParam, 
    }, 
};

#[get("/private/api/settings/wirednetwork/status")]
pub async fn get_wanpage(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);

            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus {
                let (
                    dhcp_status, 
                    read_internet_ip, 
                    read_netmask, 
                    read_gateway, 
                    read_dns
                ) = db::systemdnetworkd::read_wan_networkd();
                Ok(
                    HttpResponse::Ok().json(
                        WanPageResult{
                            dhcp: dhcp_status,
                            wired_network_param: StaticWiredNetworkParam {
                                internet_ip: read_internet_ip,
                                netmask: read_netmask,
                                gateway: read_gateway,
                                dns: read_dns,
                            }
                        }
                    )
                )
            }
            else {
                db::users::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Gone().json(
                        HttpResponseCustom{
                            operation_status: "Failed".to_string(),
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

#[get("/private/api/settings/wirelessnetwork/status")]
pub async fn get_wlanpage(req: HttpRequest) -> Result<HttpResponse>{
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);

            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus {
                let (
                    read_router_ip, 
                    read_netmask, 
                    read_range_start, 
                    read_range_end, 
                    read_dns, 
                    read_default_lease, 
                    read_max_lease, 
                    read_timezone
                ) = db::systemdnetworkd::read_wlan_networkd();

                Ok(
                    HttpResponse::Ok().json(
                        WirelessNetworkParam{
                            router_ip: read_router_ip,
                            netmask: read_netmask,
                            range_start: read_range_start,
                            range_end: read_range_end,
                            dns: read_dns,
                            default_lease: read_default_lease,
                            max_lease: read_max_lease,
                            timezone: read_timezone,
                        }
                    )
                )
            }
            else {
                db::users::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Gone().json(
                        HttpResponseCustom{
                            operation_status: "Failed".to_string(),
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
