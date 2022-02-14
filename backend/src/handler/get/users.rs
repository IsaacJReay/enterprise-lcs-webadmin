use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    HttpRequest,
};
use crate::{
    db, 
    security, 
    tool,
    structs::{
        HttpResponseCustom, 
        UserName
    }, 
};

#[get("/private/api/user/query")]
pub async fn get_logindata(req: HttpRequest) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth) {
            let olddate = security::extract_token(auth);
            let (current_username, _password) = db::users::query_logindata();


            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus{
                Ok(
                    HttpResponse::Ok().json(
                        UserName{
                            username: current_username
                        }
                    )
                )
            }
            else{
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
                    HttpResponseCustom{
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
                HttpResponseCustom{
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}