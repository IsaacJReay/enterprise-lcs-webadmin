use crate::{handler, linux, structs::TimeDateZoneNTP};
use actix_web::{get, HttpRequest, HttpResponse, Result};

#[get("/private/api/settings/time/status")]
pub async fn get_timedatepage(req: HttpRequest) -> Result<HttpResponse> {
    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let (_code, output, _error) = linux::systemsettings::query_date_for_display();
    let time_vec: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();
    let current_date = time_vec[0];
    let current_time = time_vec[1];
    let (_code, current_timezone, _error) = linux::systemsettings::query_timezone();
    let (_code, current_ntp_status, _error) = linux::systemsettings::query_ntp_status();
    let status = match current_ntp_status.as_ref() {
        "active" => true,
        _ => false,
    };

    Ok(HttpResponse::Ok().json(TimeDateZoneNTP {
        ntp_status: status,
        time: current_time.to_string(),
        date: current_date.to_string(),
        timezone: current_timezone.to_string(),
    }))
}
