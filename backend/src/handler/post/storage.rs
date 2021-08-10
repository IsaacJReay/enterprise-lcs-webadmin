use actix_web::{
    post,
    web,
    HttpRequest,
    HttpResponse,
    Result,
};
use crate::{
    db,
    security,
    tool,
    structs::{
        PartUUID,
        HttpResponseCustom,
    }
};

#[post("/private/api/settings/storage/device/rwpermission/request")]
pub async fn post_storage_device_rw_permission(req: HttpRequest, uuid_struct: web::Json<PartUUID>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                Ok(
                    HttpResponse::Ok().json(
                        "Test"
                    )
                )
            }
            else {
                db::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Gone().json(
                        HttpResponseCustom{
                            operation_status: "failed".to_string(),
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