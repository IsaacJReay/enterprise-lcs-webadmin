use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    HttpRequest,
};
use crate::{
    config,
    db, 
    security, 
    tool,
    structs::{
        HttpResponseCustom,
        StatusPageResult, 
    }, 
};

#[get("/private/api/token/validation")]
pub async fn get_token_validated(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth) {
            let olddate = security::extract_token(auth);
            let (_username, _password) = db::users::query_logindata();


            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus{
                Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom{
                            operation_status: "Success".to_string(),
                            reason: "token-valid".to_string(),
                        }
                    )
                )
            }
            else{
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
                    HttpResponseCustom{
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
                HttpResponseCustom{
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}

#[get("/private/api/settings/status")]
pub async fn get_statuspage(req: HttpRequest) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);

            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus{

                let (
                    ssid, 
                    _hide_ssid, 
                    _wpa,
                    hw_mode, 
                    channel, 
                    _passphrase, 
                    hw_n_mode, 
                    qos
                ) = db::hostapd::read_hostapd();
                let (
                    eth0_macaddr, 
                    eth0_ipaddr, 
                    eth0_subnetmask, 
                    eth0_gateway
                ) = db::systemdnetworkd::read_eth0();
                let (
                    wlan0_macaddr, 
                    wlan0_ipaddr, 
                    wlan0_subnetmask
                ) = db::systemdnetworkd::read_wlan0();
                let wlan0_dns = config::named::read_forward_dns_server();

                Ok(
                    HttpResponse::Ok().json(
                        StatusPageResult {
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
                            wlan_qos: qos
                        }
                    )
                )
            }
            else{
                db::users::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Gone().json(
                        HttpResponseCustom {
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
