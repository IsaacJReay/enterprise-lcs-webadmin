use actix_web::{
    HttpResponse, 
    HttpRequest,
    Result, 
    put,
    web,
};
use crate::{
    db,
    security,
    tool,
    handler::helper::return_httpsresponse_from_config_named_conf_external_zone,
    structs::{
        UpdateStatus,
        RenameDomain,
        HttpResponseCustom,
    },
};

#[put("/private/api/settings/dns/status/update")]
pub async fn put_update_dns_status(req: HttpRequest, update_status_struct: web::Json<UpdateStatus>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let id = update_status_struct.id.clone();
                let status = update_status_struct.status;

                db::named::insert_status_into_dnszone_by_id(id.as_str(), status);

                return_httpsresponse_from_config_named_conf_external_zone()
            }
            else {
                db::users::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Ok().json(
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

#[put("/private/api/settings/dns/domain_name/update")]
pub async fn put_rename_domain_name(req: HttpRequest, rename_domain_struct: web::Json<RenameDomain>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let foreign_key: String = rename_domain_struct.foreign_key.foreign_key.clone();
                let new_domain_name: String = rename_domain_struct.new_domain_name.clone();
                db::named::update_domain_name_by_foreign_key(&foreign_key, &new_domain_name);

                return_httpsresponse_from_config_named_conf_external_zone()
            }
            else {
                db::users::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Ok().json(
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

