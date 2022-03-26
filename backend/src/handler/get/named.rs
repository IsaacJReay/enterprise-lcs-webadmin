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
    let zone_vec = config::named::read_zone_config_file(zone_is_internal, true);

    Ok(HttpResponse::Ok().json(zone_vec))
}