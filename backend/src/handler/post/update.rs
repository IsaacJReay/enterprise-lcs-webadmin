use actix_web::{
    web,
    post,
    Result,
    HttpResponse,
    HttpRequest,
};
use fork::{daemon, Fork};
use crate::{
    db,
    security,
    tool,
    config::{
        write_file,
        update::update_content_server,
    },
    structs::{
        HttpResponseCustom,
        SystemUpdateRequest
    },
};

#[post("/private/api/settings/update/update")]
pub async fn post_update_content_server(req: HttpRequest, update_request_struct: web::Json<SystemUpdateRequest>) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::users::query_logindata();
            let password_status: bool = tool::comparedate(olddate);

            if password_status{
                match std::path::Path::new("/tmp/update_db.lock").exists() {
                    true => Ok(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "Another update is in progress".to_string(),
                            }
                        )
                    ),
                    false => {
                        write_file(" ".as_bytes(), "/tmp/update_db.lock");
                        if let Ok(Fork::Child) = daemon(false, false) {
                            update_content_server(&password, &update_request_struct.id, update_request_struct.sys_update);
                        }
                        Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom {
                                    operation_status: "Success".to_string(),
                                    reason: "Another update is in progress".to_string(),
                                }
                            )
                        )
                    }
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
