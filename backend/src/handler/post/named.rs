use actix_web::{
    post,
    HttpRequest,
    HttpResponse,
    web,
    Result,
};
use crate::{
    db,
    security,
    tool,
    handler::helper::{
        return_httpsresponse_from_config_named_conf_external_zone,
        return_httpsresponse_from_config_var_named_external_zone,
    },
    structs::{
        PartialZoneRecords,
        CreateDomainName,
        HttpResponseCustom,
    },
};

#[post("/private/api/settings/dns/domain_name/creation")]
pub async fn post_create_domain_name(req: HttpRequest, domain_name_struct: web::Json<CreateDomainName>) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, _password) = db::query_logindata();
            let password_status: bool = tool::comparedate(olddate);

            if password_status{

                let domain_name = domain_name_struct.domain_name.clone();
                db::insert_domain_name_into_dnszones(domain_name.as_str());
                return_httpsresponse_from_config_named_conf_external_zone()
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

#[post("/private/api/settings/dns/zone_record/creation")]
pub async fn post_add_zone_record(req: HttpRequest, zone_record_struct: web::Json<PartialZoneRecords>) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, _password) = db::query_logindata();
            let password_status: bool = tool::comparedate(olddate);

            if password_status{

                db::insert_into_zonerecords(zone_record_struct.clone());
                return_httpsresponse_from_config_var_named_external_zone(zone_record_struct.foreign_key.as_str())
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
