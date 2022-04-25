use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    HttpRequest,
};

use crate::{
    config,
    handler,
};

#[get("/private/api/settings/dns/status/{zone}")]
pub async fn get_dns_page(req: HttpRequest) -> Result<HttpResponse> { 

    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false
    };
    let zone_vec = config::named::read_zone_config_file(zone_is_internal, false);

    Ok(HttpResponse::Ok().json(zone_vec))
}

#[get("/private/api/settings/dns/status/{zone}/{domain_name}")]
pub async fn get_dns_page_domain_name(req: HttpRequest) -> Result<HttpResponse> {
    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false,
    };
    let domain_name = req.match_info().get("domain_name").unwrap();
    let subdomain = config::named::read_zone_record_file(zone_is_internal, domain_name);
    let mut zone_vec = config::named::read_zone_config_file(zone_is_internal, false)
        .into_iter()
        .filter(|each_item| each_item.domain_name == domain_name)
        .next()
        .unwrap();
    zone_vec.zone_record = match &subdomain.is_empty() {
        true => None,
        false => Some(subdomain)
    };
    Ok(HttpResponse::Ok().json(zone_vec))
}