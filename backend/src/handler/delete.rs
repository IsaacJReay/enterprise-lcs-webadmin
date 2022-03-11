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
    linux,
    config,
    structs::{
        HttpResponseCustom,
        DeleteArgs,
    }
};

#[delete("/private/api/settings/dns/delete/{zone}/{domain_name}")]
pub async fn delete_delete_domain_name(req: HttpRequest) -> Result<HttpResponse> {
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
                let domain_name = req.match_info().get("domain_name").unwrap();
                match config::named::delete_domain_name(&password, domain_name, zone_is_internal) {
                    Ok(()) => Ok(
                        HttpResponse::Gone().json(
                            HttpResponseCustom{
                                operation_status: "Success".to_string(),
                                reason: "".to_string(),
                            }
                        )
                    ),
                    Err(err) => Ok(
                        HttpResponse::Gone().json(
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

#[delete("/private/api/settings/dns/delete/{zone}/{domain_name}/{subdomain_name}")]
pub async fn delete_delete_zone_record(req: HttpRequest) -> Result<HttpResponse> {
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
                let subdomain_name = req.match_info().get("subdomain_name").unwrap();
                let domain_name = req.match_info().get("domain_name").unwrap();
                match config::named::delete_dns_records(&password, domain_name, subdomain_name, zone_is_internal) {
                    Ok(()) => Ok(
                        HttpResponse::Gone().json(
                            HttpResponseCustom{
                                operation_status: "Success".to_string(),
                                reason: "".to_string(),
                            }
                        )
                    ),
                    Err(err) => Ok(
                        HttpResponse::Gone().json(
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

#[delete("/private/api/settings/storage/device/deletion")]
pub async fn post_storage_device_remove_filedir(req: HttpRequest, args_vec: web::Json<DeleteArgs>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::users::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {

                let items_prefix = match args_vec.drive_partuuid.as_str() {
                    "kmp" => "/kmp/webadmin".to_string(),
                    _ => db::storage::query_mount_by_uuid_from_storage_table(&args_vec.drive_partuuid)
                };
                
                let items_string = args_vec.selected_filedir
                    .iter()
                    .map(|s| format!("{}/{}", items_prefix, s))
                    .collect::<Vec<String>>()
                    .join(" ");

                println!("{}", items_string);

                let (code, output, error) = linux::storage::remove_filedir_root(&password, &items_string);

                match code {
                    0 => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom{
                                operation_status: "Success".to_string(),
                                reason: output,
                            }
                        )
                    ),
                    _ => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom{
                                operation_status: "Failed".to_string(),
                                reason: error,
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
