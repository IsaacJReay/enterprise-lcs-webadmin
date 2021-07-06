use actix_web::{
    HttpResponse,
    HttpRequest, 
    Result, 
    delete,
    web,
};
use crate::{
    security,
    tool,
    db,
    handler::helper::{
        return_httpsresponse_from_config_var_named_external_zone,
        return_httpsresponse_from_config_named_conf_external_zone,
    },
    structs::{
        DeleteRecord,
        DnsId,
        HttpResponseCustom,
    }
};

#[delete("/private/api/settings/dns/zone_record/deletion")]
pub async fn delete_delete_zone_record(req: HttpRequest, delete_record_struct: web::Json<DeleteRecord>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let foreign_key = &delete_record_struct.foreign_key.foreign_key.to_owned();
                let id = &delete_record_struct.id.id.to_owned();

                db::delete_from_zonerecords_by_id(
                    id, 
                    &foreign_key,
                );

                return_httpsresponse_from_config_var_named_external_zone(foreign_key)
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

#[delete("/private/api/settings/dns/domain_name/deletion")]
pub async fn delete_delete_domain_name(req: HttpRequest,dns_id_struct: web::Json<DnsId>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
            db::delete_from_dnszones_by_id(dns_id_struct.id.to_owned().as_str());

            return_httpsresponse_from_config_named_conf_external_zone()
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


