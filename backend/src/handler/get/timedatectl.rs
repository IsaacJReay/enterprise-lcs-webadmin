use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    HttpRequest,
};
use crate::{
    db, 
    linux, 
    security, 
    tool,
    structs::{
        HttpResponseCustom, 
        TimeDateZoneNTP, 
    }, 
};

#[get("/private/api/settings/time/status")]
pub async fn get_timedatepage(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let (_code, output, _error) = linux::systemsettings::query_date_for_display();
                let time_vec: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();
                let current_date = time_vec[0];
                let current_time = time_vec[1];
                let (_code, current_timezone, _error) = linux::systemsettings::query_timezone();
                let (_code, current_ntp_status, _error) = linux::systemsettings::query_ntp_status();
                let status: bool = if current_ntp_status == "active" {
                    true
                }
                else {
                    false
                };

                Ok(
                    HttpResponse::Ok().json(
                        TimeDateZoneNTP{
                            ntp_status: status,
                            time: current_time.to_string(),
                            date: current_date.to_string(),
                            timezone: current_timezone.to_string(),
                        }
                    )
                )
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
