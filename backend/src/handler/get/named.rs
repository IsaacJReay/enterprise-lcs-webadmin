use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    HttpRequest,
};

use crate::{
    db, 
    tool, 
    config,
    structs::{HttpResponseCustom},
    security
};

// use crate::{db, security, structs::{GetZoneRecords, HttpResponseCustom}, tool};

#[get("/private/api/settings/dns/status/{zone}")]
pub async fn get_dns_page(req: HttpRequest) -> Result<HttpResponse> { 
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);

            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let zone_is_internal = match req.match_info().get("zone").unwrap() {
                    "internal" => true,
                    _ => false
                };
                let zone_vec = config::named::read_zone_config_file(zone_is_internal, true);

                Ok(
                    HttpResponse::Ok().json(
                        zone_vec
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