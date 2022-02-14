use actix_web::{
    post,
    web,
    HttpRequest,
    HttpResponse,
    Result,
};
use crate::{db, linux, security, structs::{MakeDirectoryArgs, MoveOrCopyArgs, HttpResponseCustom, PartUUID}, tool};

#[post("/private/api/settings/storage/device/copy_or_move")]
pub async fn post_storage_device_copy_or_move(req: HttpRequest, args_vec: web::Json<MoveOrCopyArgs>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {

                let source_prefix =  match args_vec.source_uuid.as_str() {
                    "kmp" => "/kmp/webadmin".to_string(),
                    _ => db::storage::query_mount_by_uuid_from_storage_table(args_vec.source_uuid.as_str()),
                };

                let destination_prefix =  match args_vec.destination_uuid.as_str() {
                    "kmp" => "/kmp/webadmin".to_string(),
                    _ => db::storage::query_mount_by_uuid_from_storage_table(args_vec.destination_uuid.as_str()),
                };

                let source_string = args_vec.source_items
                    .iter()
                    .map( |s| format!("{}/{}", source_prefix, s))
                    .collect::<Vec<String>>()
                    .join(" ");

                let destination_string = format!("{}/{}", destination_prefix, args_vec.items_destination);
                

                if args_vec.operation == "copy" {
                    let (code, output, error) = linux::storage::copy_or_move(
                        true,
                        &source_string, 
                        &destination_string,
                    );

                    match code {
                        0 => Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom{
                                    operation_status: "Success".to_string(),
                                    reason: output.to_string(),
                                }
                            )
                        ),
                        _ => Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom{
                                    operation_status: "Failed".to_string(),
                                    reason: error,
                                }
                            )
                        )
                    }
                    
                }
                else if args_vec.operation == "move" {
                    let (code, output, error) = linux::storage::copy_or_move(
                        false,
                        &source_string, 
                        &destination_string   
                    );

                    match code {
                        0 => Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom{
                                    operation_status: "Success".to_string(),
                                    reason: output,
                                }
                            )
                        ),
                        _ => Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom{
                                    operation_status: "Failed".to_string(),
                                    reason: error,
                                }
                            )
                        )
                    }
                }
                else {
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom{
                                operation_status: "Failed".to_string(),
                                reason: "operation-not-supported".to_string(),
                            }
                        )
                    )
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

#[post("/private/api/settings/storage/device/directory/creation")]
pub async fn post_storage_device_directory_creation(req: HttpRequest, directory_info: web::Json<MakeDirectoryArgs>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {

                let dir_location = match directory_info.drive_partuuid.as_str() {
                    "kmp" => format!("/kmp/webadmin/{}/{}", directory_info.parent_directory, directory_info.directory_name),
                    _ => format!("{}/{}/{}", db::storage::query_mount_by_uuid_from_storage_table(&directory_info.drive_partuuid), directory_info.parent_directory, directory_info.directory_name)
                };

                let (code, output, error) = linux::storage::make_dir(&dir_location);


                match code {
                    0 => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom{
                                operation_status: "Success".to_string(),
                                reason: output,
                            }
                        )
                    ),
                    _ => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom{
                                operation_status: "Failed".to_string(),
                                reason: error,
                            }
                        )
                    )
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

#[post("/private/api/settings/storage/device/unmount")]
pub async fn post_storage_device_unmount(req: HttpRequest, uuid_struct: web::Json<PartUUID>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::users::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let full_dev_path = format!("/dev/{}", db::storage::query_path_by_uuid_from_storage_table(&uuid_struct.drive_partuuid));

                let (code, output, error) = linux::storage::unmount_partition(&password, &full_dev_path);

                match code {
                    0 => {
                        db::storage::delete_from_storage_table(&uuid_struct.drive_partuuid);
                        Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom{
                                    operation_status: "Success".to_string(),
                                    reason: output,
                                }
                            )
                        )
                    },
                    _ => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom{
                                operation_status: "Failed".to_string(),
                                reason: error,
                            }
                        )
                    )
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