use actix_web::{
    HttpResponse, 
    HttpRequest,
    Result, 
    put,
};
use crate::{
    db,
    security,
    tool,
    config,
    structs::HttpResponseCustom,
};

#[put("/private/api/settings/dns/domain_name/rename/{zone}/{old_domain_name}/{new_domain_name}")]
pub async fn put_rename_domain_name(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            let (_username, password) = db::users::query_logindata();
            if passwordstatus {
                let zone_is_internal = match req.match_info().get("zone").unwrap() {
                    "internal" => true,
                    _ => false
                };
                let old_domain_name = req.match_info().get("old_domain_name").unwrap();
                let new_domain_name = req.match_info().get("new_domain_name").unwrap();
                match config::named::rename_domain_name(&password, &old_domain_name, &new_domain_name, zone_is_internal) {
                    Ok(()) => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom{
                                operation_status: "Success".to_string(),
                                reason: "".to_string(),
                            }
                        )
                    ),
                    Err(err) => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom{
                                operation_status: "Failed".to_string(),
                                reason: err,
                            }
                        )
                    )
                }
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


