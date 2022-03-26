use actix_web::{
    HttpResponse, 
    HttpRequest,
    Result, 
    put,
    error,
    http
};
use crate::{
    handler,
    config,
};

#[put("/private/api/settings/dns/domain_name/rename/{zone}/{old_domain_name}/{new_domain_name}")]
pub async fn put_rename_domain_name(req: HttpRequest) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;
    let zone_is_internal = match req.match_info().get("zone").unwrap() {
        "internal" => true,
        _ => false
    };
    let old_domain_name = req.match_info().get("old_domain_name").unwrap();
    let new_domain_name = req.match_info().get("new_domain_name").unwrap();
    match config::named::rename_domain_name(&password, &old_domain_name, &new_domain_name, zone_is_internal) {
        Ok(()) => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        Err(err) => Err(error::ErrorInternalServerError(err))
    }

}
