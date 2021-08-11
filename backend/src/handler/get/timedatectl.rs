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
        NTPStatus, 
        TimeDate, 
        TimeDateZone, 
        TimeDateZoneNTP, 
        Timezone, 
    }, 
};

#[get("/private/api/settings/time/status")]
pub async fn get_timedatepage(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let (_code, output, _error) = linux::query_date_for_display();
                let time_vec: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();
                let current_date = time_vec[0];
                let current_time = time_vec[1];
                let (_code, current_timezone, _error) = linux::query_timezone();
                let (_code, current_ntp_status, _error) = linux::query_ntp_status();
                let status: bool = if current_ntp_status == "active" {
                    true
                }
                else {
                    false
                };

                Ok(
                    HttpResponse::Ok().json(
                        TimeDateZoneNTP{
                            ntp_status: NTPStatus {
                                ntp_status: status,
                            },
                            timedatezone: TimeDateZone{
                                timedate: TimeDate{
                                    time: current_time.to_string(),
                                    date: current_date.to_string(),
                                },
                                timezone: Timezone {
                                    timezone: current_timezone.to_string(),
                                }
                            }
                        }
                    )
                )
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
