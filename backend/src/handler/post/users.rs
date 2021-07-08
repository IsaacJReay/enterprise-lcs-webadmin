use actix_web::{
    web,
    post,
    Result,
    HttpRequest,
    HttpResponse,
};
use pam::{
    Authenticator,
    PasswordConv,
};
use crate::{
    db,
    tool,
    security,
    linux,
    structs::{
        LoginParam,
        HttpResponseCustom,
        PasswdParam,
    },

};


#[post("/private/api/user/login")]
pub async fn post_pam_login(logindata: web::Json<LoginParam>) -> Result<HttpResponse> {
    
    // setup authenticator with system-auth
    let mut auth: Authenticator<PasswordConv> = Authenticator::with_password("system-auth")
        .unwrap();

    // Now, give username password to be authenticated 
    auth.get_handler()
        .set_credentials(&logindata.username, &logindata.password);

    // Now, Authenticate and Listen for feedback
    if  auth.authenticate()
            .is_ok() && 
        auth
            .open_session()
            .is_ok() {
        db::update_logindata(&logindata.username, &logindata.password);
        let token = security::generate_token(&logindata.username, &logindata.password);
        db::insert_into_token_table(&token);
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Success".to_string(),
                    reason: token,
                }
            )
        )
    }
    else{
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "wrong_username_or_password".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/user/password")]
pub async fn post_reset_password(req: HttpRequest, passwdparam: web::Json<PasswdParam>) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (username, _password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus {
                let (code, _output, error) = linux::passwd(&username, &passwdparam.old_password, &passwdparam.new_password);
                if code == 0 {
                    db::update_logindata(&username, &passwdparam.new_password);
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Success".to_string(),
                                reason: "".to_string(),
                            }
                        )
                    )
                }
                else{
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: error,
                            }
                        )
                    )
                }
            }
            else {
                db::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Ok().json(
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
                HttpResponse::Ok().json(
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
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}
