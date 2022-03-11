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
    config,
    structs::{
        DnsZonesInfo,
        HttpResponseCustom,
    }, 
};

#[post("/private/api/settings/dns/new/{zone}")]
pub async fn post_handle_new_domain_name_and_record(req: HttpRequest, args_struct: web::Json<DnsZonesInfo>) -> Result<HttpResponse> { 
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::users::query_logindata();
            let password_status: bool = tool::comparedate(olddate);

            if password_status{
                
                if !&args_struct.domain_name.contains(" ") {
                    let zone_is_internal = match req.match_info().get("zone").unwrap() {
                        "internal" => true,
                        _ => false
                    };
                    match config::named::handle_new_domain_name_and_record(&password, args_struct.to_owned(), zone_is_internal) {
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
                    Ok(
                        HttpResponse::NotAcceptable().json(
                            HttpResponseCustom{
                                operation_status: "Failed".to_string(),
                                reason: "No Space is Allowed".to_string(),
                            }
                        )
                    )
                }
                
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
