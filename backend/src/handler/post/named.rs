use crate::{config, handler, structs::DnsZonesInfo};
use actix_web::{error, http, post, web, HttpRequest, HttpResponse, Result};

#[post("/private/api/settings/dns/new/{zone}")]
pub async fn post_handle_new_domain_name_and_record(
    req: HttpRequest,
    args_struct: web::Json<DnsZonesInfo>,
) -> Result<HttpResponse> {
    let (_username, password) = handler::handle_validate_token_response(&req)?;

    match &args_struct.domain_name.contains(char::is_whitespace) {
        true => Err(error::ErrorNotAcceptable("unacceptable_space")),
        false => Ok(()),
    }?;

    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false,
    };
    match config::named::handle_new_domain_name_and_record(
        &password,
        args_struct.to_owned(),
        zone_is_internal,
    ) {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}
