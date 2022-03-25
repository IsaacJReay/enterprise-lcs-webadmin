use actix_web::{
    post,
    HttpRequest,
    HttpResponse,
    web,
    Result,
};
use crate::{
    handler,
    config,
    structs::{
        DnsZonesInfo,
        HttpResponseCustom,
    }, 
};

#[post("/private/api/settings/dns/new/{zone}")]
pub async fn post_handle_new_domain_name_and_record(req: HttpRequest, args_struct: web::Json<DnsZonesInfo>) -> Result<HttpResponse> { 

    let (_username, password) = handler::handle_validate_token_response(&req)?;

    match &args_struct.domain_name.contains(" ") {
        true => Ok(()),
        false => Err(
            HttpResponse::NotAcceptable().json(
                HttpResponseCustom{
                    operation_status: "Failed".to_string(),
                    reason: "No Space is Allowed".to_string(),
                }
            )
        )
    }?;

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
