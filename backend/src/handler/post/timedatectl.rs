use actix_web::{
    web,
    post,
    Result,
    HttpResponse,
    HttpRequest,
};
use crate::{
    db,
    security,
    tool,
    linux,
    structs::{
        Timezone,
        HttpResponseCustom,
        TimeDate,
    },
};

#[post("/private/api/settings/time/timezone")]
pub async fn post_set_timezone(req: HttpRequest, timezone_struct: web::Json<Timezone>) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let password_status: bool = tool::comparedate(olddate);

            if password_status{
                let (code, _output, error) = linux::set_timezone(&password, &timezone_struct.timezone);
                match code {
                    0 => Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom {
                                    operation_status: "Success".to_string(),
                                    reason: "".to_string(),
                                }
                            )
                        ),
                    _ => Ok(
                            HttpResponse::InternalServerError().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: format!("{}", error),
                                }
                            )
                        ),
                }
            }
            else {
                db::delete_from_token_table(auth);
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

#[post("/private/api/settings/time/timedate")]
pub async fn post_set_time(req: HttpRequest, time_struct: web::Json<TimeDate>) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let password_status: bool = tool::comparedate(olddate);

            let timedate = format!("{} {}", time_struct.date, time_struct.time);

            if password_status{
                let (code, _output, error) = linux::set_time(&password, &timedate);
                match code {
                    0 => Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom {
                                    operation_status: "Success".to_string(),
                                    reason: "".to_string(),
                                }
                            )
                        ),
                    _ => Ok(
                            HttpResponse::InternalServerError().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: format!("{}", error),
                                }
                            )
                        ),
                }
            }
            else {
                db::delete_from_token_table(auth);
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
