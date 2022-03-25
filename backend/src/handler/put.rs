use actix_web::{
    HttpResponse, 
    HttpRequest,
    Result, 
    put,
};
use crate::{
    db,
    config,
    structs::HttpResponseCustom,
};

#[put("/private/api/settings/dns/domain_name/rename/{zone}/{old_domain_name}/{new_domain_name}")]
pub async fn put_rename_domain_name(req: HttpRequest) -> Result<HttpResponse> {

    let (_username, password) = match db::users::validate_token(&req){
        Ok((username, password)) => Ok((username, password)),
        Err((code, message)) => match code {
            401 
            => Err(
                    HttpResponse::Gone().json(
                        HttpResponseCustom{
                            operation_status: "Failed".to_string(),
                            reason: message,
                        }
                    )
                ),
            _ 
            => Err(
                    HttpResponse::Unauthorized().json(
                        HttpResponseCustom{
                            operation_status: "Failed".to_string(),
                            reason: message,
                        }
                    )
                )
        }
    }?;

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
