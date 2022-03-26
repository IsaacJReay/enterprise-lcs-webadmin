use actix_web::{
    web,
    post,
    Result,
    HttpResponse,
    HttpRequest,
    error,
    http
};
use crate::{
    handler,
    linux,
    structs::{
        Timezone,
        TimeDate,
    },
};

#[post("/private/api/settings/time/timezone")]
pub async fn post_set_timezone(req: HttpRequest, timezone_struct: web::Json<Timezone>) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let (code, _output, error) = linux::systemsettings::set_timezone(&password, &timezone_struct.timezone);
    match code {
        0 => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        _ => Err(error::ErrorInternalServerError(error))
    }
}

#[post("/private/api/settings/time/timedate")]
pub async fn post_set_time(req: HttpRequest, time_struct: web::Json<TimeDate>) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let (code, _output, error) = linux::systemsettings::set_time(&password, &time_struct.date, &time_struct.time);
    match code {
        0 => Ok(HttpResponse::new(http::StatusCode::from_u16(200).unwrap())),
        _ => Err(error::ErrorInternalServerError(error))
    }
}
