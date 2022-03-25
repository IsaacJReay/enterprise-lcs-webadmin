use actix_web::{
    HttpResponse,
    HttpRequest, 
    Result, 
    delete,
    web,
};
use crate::{
    handler,
    linux,
    config,
    structs::{
        HttpResponseCustom,
        DeleteArgs,
    }
};

#[delete("/private/api/settings/dns/delete/{zone}/{domain_name}")]
pub async fn delete_delete_domain_name(req: HttpRequest) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false
    };
    let domain_name = req.match_info().get("domain_name").unwrap();
    match config::named::delete_domain_name(&password, domain_name, zone_is_internal) {
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

#[delete("/private/api/settings/dns/delete/{zone}/{domain_name}/{subdomain_name}")]
pub async fn delete_delete_zone_record(req: HttpRequest) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;
    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false
    };
    let subdomain_name = req.match_info().get("subdomain_name").unwrap();
    let domain_name = req.match_info().get("domain_name").unwrap();
    match config::named::delete_dns_records(&password, domain_name, subdomain_name, zone_is_internal) {
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

#[delete("/private/api/settings/storage/device/deletion")]
pub async fn post_storage_device_remove_filedir(req: HttpRequest, args_vec: web::Json<DeleteArgs>) -> Result<HttpResponse> {
    
    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let items_prefix = match args_vec.drive_partuuid.as_str() {
        "kmp" => "/kmp/webadmin".to_string(),
        _ => crate::db::storage::query_from_storage_table(None, Some(&args_vec.drive_partuuid)).1
    };
    
    let items_string = args_vec.selected_filedir
        .iter()
        .map(|s| format!("{}/{}", items_prefix, s))
        .collect::<Vec<String>>()
        .join(" ");

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
