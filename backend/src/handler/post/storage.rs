use actix_web::{
    post,
    web,
    HttpRequest,
    HttpResponse,
    Result,
};
use crate::{db, linux, security, structs::{MakeDirectoryArgs, DeleteArgs, MoveOrCopyArgs, HttpResponseCustom, PartUUID}, tool};

#[post("/private/api/settings/storage/device/rwpermission/request")]
pub async fn post_storage_device_rw_permission(req: HttpRequest, uuid_struct: web::Json<PartUUID>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let path = db::query_path_by_uuid_from_storage_table(&uuid_struct.drive_partuuid);
                let (code, output, error) = linux::mount_rw_partition(&password, &path, &uuid_struct.drive_partuuid);
                match code {
                    0 => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Success".to_string(),
                                reason: output,
                            }
                        )
                    ),
                    _ => Ok(
                        HttpResponse::InternalServerError().json(
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

#[post("/private/api/settings/storage/device/copy_or_move")]
pub async fn post_storage_device_copy_or_move(req: HttpRequest, args_vec: web::Json<MoveOrCopyArgs>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {

                let source_is_external_prefix = match args_vec.source_uuid.as_str() {
                    "kmp" => false,
                    _ => true,
                };
                let destination_is_external_prefix = match args_vec.destination_uuid.as_str() {
                    "kmp" => false,
                    _ => true,
                };

                let source_prefix =  match args_vec.source_uuid.as_str() {
                    "kmp" => "/kmp".to_string(),
                    _ => db::query_mount_by_uuid_from_storage_table(args_vec.source_uuid.as_str()),
                };

                let destination_prefix =  match args_vec.destination_uuid.as_str() {
                    "kmp" => "/kmp".to_string(),
                    _ => db::query_mount_by_uuid_from_storage_table(args_vec.destination_uuid.as_str()),
                };

                let source_string = args_vec.source_files
                    .iter()
                    .map( |s| format!("{}/{}", source_prefix, s))
                    .collect::<Vec<String>>()
                    .join(" ");

                let destination_string = args_vec.destination_files
                    .iter()
                    .map(|s| format!("{}/{}", destination_prefix, s))
                    .collect::<Vec<String>>()
                    .join(" ");
                

                if args_vec.operation == "copy" {
                    let (code, output, error) = linux::copy_filedir(
                        &password, 
                        &source_string, 
                        &destination_string, 
                        source_is_external_prefix, 
                        &args_vec.source_uuid, 
                        destination_is_external_prefix, 
                        &args_vec.destination_uuid
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
                    let (code, output, error) = linux::move_filedir(
                        &password, 
                        &source_string, 
                        &destination_string, 
                        source_is_external_prefix, 
                        &args_vec.source_uuid, 
                        destination_is_external_prefix, 
                        &args_vec.destination_uuid
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

#[post("/private/api/settings/storage/device/deletion")]
pub async fn post_storage_device_remove_filedir(req: HttpRequest, args_vec: web::Json<DeleteArgs>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {

                let items_prefix = match args_vec.drive_partuuid.as_str() {
                    "kmp" => "/kmp".to_string(),
                    _ => db::query_mount_by_uuid_from_storage_table(&args_vec.drive_partuuid)
                };
                
                let items_string = args_vec.selected_filedir
                    .iter()
                    .map(|s| format!("{}/{}", items_prefix, s))
                    .collect::<Vec<String>>()
                    .join(" ");

                let (code, output, error) = linux::remove_filedir_root(&password, &items_string);

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

#[post("/private/api/settings/storage/device/directory/creation")]
pub async fn post_storage_device_directory_creation(req: HttpRequest, directory_info: web::Json<MakeDirectoryArgs>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                // let item_prefix = match directory_info.parent_directory.as_str() {
                //     "/kmp/webadmin" => "/kmp/webadmin".to_string(),
                //     _ => db::query_mount_by_uuid_from_storage_table(&directory_info.parent_directory),
                // };

                let drive_is_external_prefix = match directory_info.drive_partuuid.as_str(){
                    "kmp" => false,
                    _ => true,
                };

                // let dir_location  = format!("{}/{}/{}", item_prefix, directory_info.parent_directory, directory_info.directory_name);

                let dir_location = match directory_info.drive_partuuid.as_str() {
                    "kmp" => format!("/kmp/webadmin/{}/{}", directory_info.parent_directory, directory_info.directory_name),
                    _ => format!("{}/{}/{}", db::query_mount_by_uuid_from_storage_table(&directory_info.drive_partuuid), directory_info.parent_directory, directory_info.directory_name)
                };

                let (code, output, error) = linux::make_dir(&password, &dir_location, drive_is_external_prefix, &directory_info.drive_partuuid);


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

#[post("/private/api/settings/storage/device/unmount")]
pub async fn post_storage_device_unmount(req: HttpRequest, uuid_struct: web::Json<PartUUID>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let full_dev_path = format!("/dev/{}", db::query_path_by_uuid_from_storage_table(&uuid_struct.drive_partuuid));
                println!("{}", &full_dev_path);

                let (code, output, error) = linux::unmount_partition(&password, &full_dev_path);

                match code {
                    0 => {
                        db::delete_from_storage_table(&uuid_struct.drive_partuuid);
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