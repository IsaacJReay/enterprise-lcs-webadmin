use crate::{config, handler, structs::{DnsRecords, EditDnsRecords}};
use actix_web::{error, http, put, web, HttpRequest, HttpResponse, Result};

#[put("/private/api/settings/dns/domain_name/rename/{zone}/{old_domain_name}")]
pub async fn put_rename_domain_name(
    req: HttpRequest,
    new_domain_name: web::Json<String>,
) -> Result<HttpResponse> {
    let (_username, password) = handler::handle_validate_token_response(&req)?;
    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false,
    };
    let old_domain_name = req.match_info().get("old_domain_name").unwrap();
    match config::named::rename_domain_name(
        &password,
        &old_domain_name,
        &new_domain_name.into_inner(),
        zone_is_internal,
    ) {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}

#[put("/private/api/settings/dns/edit/{zone}/{domain_name}")]
pub async fn put_edit_dns_records(
    req: HttpRequest,
    records_edit_pair: web::Json<EditDnsRecords>
) -> Result<HttpResponse> {
    let (_username, password) = handler::handle_validate_token_response(&req)?;
    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false,
    };
    let domain_name = req.match_info().get("domain_name").unwrap();
    let records_edit_pair = records_edit_pair.into_inner();

    match config::named::edit_dns_record(
        &password,
        domain_name,
        records_edit_pair.old_record,
        records_edit_pair.new_record,
        zone_is_internal,
    ) {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}

#[put("/private/api/settings/dns/sort/{zone}/{domain_name}")]
pub async fn put_sort_dns_records(
    req: HttpRequest,
    vec_records: web::Json<Vec<DnsRecords>>,
) -> Result<HttpResponse> {
    let (_username, password) = handler::handle_validate_token_response(&req)?;
    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false,
    };
    let domain_name = req.match_info().get("domain_name").unwrap();
    match config::named::sort_dns_records(
        &password,
        domain_name,
        vec_records.into_inner(),
        zone_is_internal,
    ) {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(err) => Err(error::ErrorInternalServerError(err)),
    }
}
