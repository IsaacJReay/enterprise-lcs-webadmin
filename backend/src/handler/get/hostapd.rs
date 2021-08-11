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
        HostapdParam, 
        HttpResponseCustom, 
    }, 
};

#[get("/private/api/settings/hostapd/status")]
pub async fn get_wifipage(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus {
                let (
                    read_ssid, 
                    read_hide_ssid, 
                    read_wpa,
                    read_hw_mode, 
                    read_channel, 
                    read_passphrase, 
                    read_hw_n_mode, 
                    read_qos
                ) = db::read_hostapd();

                Ok(
                    HttpResponse::Ok().json(
                        HostapdParam {
                            ssid: read_ssid,
                            hide_ssid: read_hide_ssid,
                            wpa: read_wpa,
                            hw_mode: read_hw_mode,
                            channel: read_channel,
                            passphrase: read_passphrase,
                            hw_n_mode: read_hw_n_mode,
                            qos: read_qos,
                        }
                    )
                )
            }
            else {
                db::delete_from_token_table(auth);
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
