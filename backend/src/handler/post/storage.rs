use actix_web::{
    post,
    web,
    HttpRequest,
    HttpResponse,
    Result,
};
use crate::{db, linux, security, structs::{CopyOrMoveArgs, DeleteFileDir, HttpResponseCustom, ItemNamePath, PartUUID}, tool};

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
                            operation_status: "failed".to_string(),
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

#[post("/private/api/settings/storage/device/copy")]
pub async fn post_storage_device_copy_or_move(req: HttpRequest, args_vec: web::Json<CopyOrMoveArgs>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {

                let mut source_string: String = String::new();
                // let mut destination_string: String = String::new();
                let source_is_external_prefix = args_vec.source[0].parent_directory.starts_with("/tmp/");
                let destination_is_external_prefix = args_vec.destination.parent_directory.starts_with("/tmp/");
                let source_uuid: String;
                let destination_uuid: String;

                if source_is_external_prefix {
                    let splited_mount = args_vec.source[0].parent_directory.split("/").collect::<Vec<&str>>();
                    source_uuid = splited_mount[2].to_string();
                }
                else {
                    source_uuid = String::new();
                }

                if destination_is_external_prefix {
                    let splited_mount = args_vec.destination.parent_directory.split("/").collect::<Vec<&str>>();
                    destination_uuid = splited_mount[2].to_string();
                }
                else {
                    destination_uuid = String::new();
                }
                
                for each_items in &args_vec.source {
                    let fullpath = format!("{}/{}", each_items.parent_directory, each_items.item_name);
                    source_string = format!("{} {}", source_string, fullpath);
                }

                let destination_string = format!("{}/{}", args_vec.destination.parent_directory, args_vec.destination.item_name);

                if args_vec.operation == "copy" {
                    let (code, output, error) = linux::copy_filedir(
                        &password, 
                        &source_string, 
                        &destination_string, 
                        source_is_external_prefix, 
                        &source_uuid, 
                        destination_is_external_prefix, 
                        &destination_uuid
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
                        &source_uuid, 
                        destination_is_external_prefix, 
                        &destination_uuid
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
                            operation_status: "failed".to_string(),
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
pub async fn post_storage_device_remove_filedir(req: HttpRequest, args_vec: web::Json<DeleteFileDir>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let mut items_string: String = String::new();

                for each_items in &args_vec.selected_filedir {
                    let full_path = format!("{}/{}", each_items.parent_directory, each_items.item_name);
                    items_string = format!("{} {}", items_string, full_path);
                }

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
                            operation_status: "failed".to_string(),
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
pub async fn post_storage_device_directory_creation(req: HttpRequest, item_info: web::Json<ItemNamePath>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let drive_uuid: String;
                let drive_is_external_prefix = item_info.parent_directory.starts_with("/tmp/");
                if drive_is_external_prefix {
                    let splited_path = item_info.parent_directory.split("/").collect::<Vec<&str>>();

                    drive_uuid = splited_path[2].to_string();
                }
                else  {
                    drive_uuid = String::new();
                }

                let dir_location  = format!("{}/{}", item_info.parent_directory, item_info.item_name);

                let (code, output, error) = linux::make_dir(&password, &dir_location, drive_is_external_prefix, &drive_uuid);


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
                            operation_status: "failed".to_string(),
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

                let (code, output, error) = linux::unmount_partition(&password, &uuid_struct.drive_partuuid);

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
                            operation_status: "failed".to_string(),
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