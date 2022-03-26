use actix_web::{
    post,
    web,
    HttpRequest,
    HttpResponse,
    Result,
    http,
    body,
    error
};
use crate::{
    db, 
    linux, 
    handler,
    structs::{
        MakeDirectoryArgs, 
        MoveOrCopyArgs, 
        PartUUID
    }
};

#[post("/private/api/settings/storage/device/copy_or_move")]
pub async fn post_storage_device_copy_or_move(req: HttpRequest, args_vec: web::Json<MoveOrCopyArgs>) -> Result<HttpResponse> {

    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let source_prefix =  match args_vec.source_uuid.as_str() {
        "kmp" => "/kmp/webadmin".to_string(),
        _ => db::storage::query_from_storage_table(None, Some(args_vec.source_uuid.as_str())).1,
    };

    let destination_prefix =  match args_vec.destination_uuid.as_str() {
        "kmp" => "/kmp/webadmin".to_string(),
        _ => db::storage::query_from_storage_table(None, Some(args_vec.destination_uuid.as_str())).1,
    };

    let source_string = args_vec.source_items
        .iter()
        .map( |s| format!("{}/{}", source_prefix, s))
        .collect::<Vec<String>>()
        .join(" ");

    let destination_string = format!("{}/{}", destination_prefix, args_vec.items_destination);
    
    let (code, output, error) = linux::storage::copy_or_move(
        match args_vec.operation.as_str() {
            "copy" =>  true,
            _ => false
        },
        &source_string, 
        &destination_string,
    );

    match code {
        0 => Ok(HttpResponse::with_body(http::StatusCode::from_u16(200).unwrap(), body::BoxBody::new(output))),
        _ => Err(error::ErrorUnauthorized(error))    
    }   
}

#[post("/private/api/settings/storage/device/directory/creation")]
pub async fn post_storage_device_directory_creation(req: HttpRequest, directory_info: web::Json<MakeDirectoryArgs>) -> Result<HttpResponse> {
    
    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let dir_location = match directory_info.drive_partuuid.as_str() {
        "kmp" => format!("/kmp/webadmin/{}/{}", directory_info.parent_directory, directory_info.directory_name),
        _ => format!("{}/{}/{}", db::storage::query_from_storage_table(None, Some(&directory_info.drive_partuuid)).1, directory_info.parent_directory, directory_info.directory_name)
    };

    let (code, output, error) = linux::storage::make_dir(&dir_location);

    match code {
        0 => Ok(HttpResponse::with_body(http::StatusCode::from_u16(200).unwrap(), body::BoxBody::new(output))),
        _ => Err(error::ErrorUnauthorized(error))    
    }
}

#[post("/private/api/settings/storage/device/unmount")]
pub async fn post_storage_device_unmount(req: HttpRequest, uuid_struct: web::Json<PartUUID>) -> Result<HttpResponse> {

    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let full_dev_path = format!("/dev/{}", db::storage::query_from_storage_table(None, Some(&uuid_struct.drive_partuuid)).0);
    let (code, output, error) = linux::storage::unmount_partition(&password, &full_dev_path);

    match code {
        0 => {
            db::storage::delete_from_storage_table(&uuid_struct.drive_partuuid);
            Ok(HttpResponse::with_body(http::StatusCode::from_u16(200).unwrap(), body::BoxBody::new(output)))
        },
        _ => Err(error::ErrorUnauthorized(error))        
    }
}