pub mod delete;
pub mod get;
pub mod post;
pub mod put;

pub fn handle_validate_token_response(req: &actix_web::HttpRequest) -> Result<(String, String), actix_web::HttpResponse>{
    match crate::db::users::validate_token(&req){
        Ok((username, password)) => Ok((username, password)),
        Err((code, message)) => match code {
            401 
            => Err(
                    actix_web::HttpResponse::Gone().json(
                        crate::structs::HttpResponseCustom{
                            operation_status: "Failed".to_string(),
                            reason: message,
                        }
                    )
                ),
            _ => 
            Err(
                actix_web::HttpResponse::Unauthorized().json(
                    crate::structs::HttpResponseCustom{
                            operation_status: "Failed".to_string(),
                            reason: message,
                        }
                    )
                )
        }
    }
}