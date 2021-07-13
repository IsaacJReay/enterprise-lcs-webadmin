use actix_web::{
    post,
    web,
    HttpRequest,
    HttpResponse,
    Result,
};
use crate::{
    tool,
    security,
    db,
    file,
    structs::{
        HostapdParam,
        HttpResponseCustom,
    },
};

#[post("/private/api/settings/hostapd")]
pub async fn post_hostapd_settings(req: HttpRequest, hostapdparam: web::Json<HostapdParam>) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
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
                ) = file::config_hostapd(deserial_param);

                if  write_hostapd_status {
                    if move_hostapd_status {
                        if restart_hostapd_status{
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
                                HttpResponse::Ok().json(
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
                            HttpResponse::Ok().json(
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
                        HttpResponse::Ok().json(
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
                    HttpResponse::Ok().json(
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
                HttpResponse::Ok().json(
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
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}
