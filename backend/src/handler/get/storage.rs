use actix_web::{
    HttpResponse, 
    Result, 
    get,
    web,
    HttpRequest,
};
use crate::{
    db, 
    linux, 
    security, 
    tool,
    structs::{
        DriveDescription, 
        HttpResponseCustom, 
        PartUUID, 
        DriveItemExtended,
        ItemNamePath,
    }, 
};

#[get("/private/api/settings/storage/status")]
pub async fn get_storage_page(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let (_username, password) = db::query_logindata();
                let (_code, output, _error) = linux::get_all_partitions();
                let all_partitions: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();
                let mut local_content_storage = linux::get_partition_information("/kmp");
                local_content_storage.drive_partuuid = PartUUID {
                    drive_partuuid: "kmp".to_string(),
                };
                local_content_storage.drive_label = "Local Content Storage".to_string();
                let mut mounted_partitions_mount: Vec<String> = Vec::new();
                let mut not_mounted_partitions: Vec<&str> = Vec::new();
                let mut drives_description: Vec<DriveDescription> = vec![local_content_storage];
                let mut mounted_partitions_length: usize = 0;
                let mut not_mounted_partitions_length: usize = 0;
                let mut drives_description_length: usize = drives_description.len();
                let mut mount_operation_status: bool = true;
                for each_partition in all_partitions {
                    // let partition_full_path = format!("/dev/{}", each_partition);
                    let (_code, partition_filesystem_type, _error) = linux::get_partition_filesystem_type(&each_partition);
                    
                    if partition_filesystem_type != "swap" {
                        let is_mounted = db::query_existence_from_storage_table_by_path(each_partition);
                        match is_mounted {
                            true => {
                                let mount = db::query_mount_by_path_from_storage_table(each_partition);
                                mounted_partitions_mount.insert(mounted_partitions_length, mount);
                                mounted_partitions_length +=1;
                            },
                            false => {
                                not_mounted_partitions.insert(not_mounted_partitions_length, each_partition);
                                not_mounted_partitions_length +=1;
                            },
                        }
                    }
                }

                for each_partition in not_mounted_partitions {
                    let (code, output, _error) = linux::mount_ro_partition(&password, each_partition);
                    match code {
                        0 => {
                            mounted_partitions_mount.insert(mounted_partitions_length, output);
                            mounted_partitions_length +=1;
                        },
                        _ => {
                            mount_operation_status = false;
                            break;
                        },
                    }   
                }

                if mount_operation_status {
                    for each_mount in mounted_partitions_mount {
                        let current_drive_description = linux::get_partition_information(&each_mount);
                        drives_description.insert(drives_description_length, current_drive_description);
                        drives_description_length +=1;
                    }
                    Ok(
                        HttpResponse::Ok().json(
                            drives_description
                        )
                    )
                }
                else {
                    Ok(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom{
                                operation_status: "Failed".to_string(),
                                reason: "mount-Failed".to_string(),
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

#[get("/private/api/settings/storage/device/status/{drive_partuuid}")]
pub async fn get_storage_device_page(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            let (_username, password) = db::query_logindata();
            if passwordstatus {
                let drive_partuuid = req.match_info().get("drive_partuuid").unwrap();
                if drive_partuuid != "kmp" {
                    let path = db::query_mount_by_uuid_from_storage_table(&drive_partuuid);
                    let all_file = linux::query_file_in_partition(&password, &path);
                    Ok(
                        HttpResponse::Ok().json(
                            DriveItemExtended {
                                drive_label: "Local Content Storage".to_string(),
                                item_list: all_file,
                            }

                        )
                    )
                }
                else{
                    let all_file = linux::query_file_in_partition(&password, "/kmp/webadmin");
                    Ok(
                        HttpResponse::Ok().json(
                            DriveItemExtended {
                                drive_label: "Removeable Device".to_string(),
                                item_list: all_file,
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

#[get("/private/api/settings/storage/device/directory/status/{parent_directory}/{item_name}")]
pub async fn get_storage_device_directory_page(req: HttpRequest, item_struct: web::Query<ItemNamePath>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            let (_username, password) = db::query_logindata();
            if passwordstatus {
                
                // let parent_directory = req.match_info().get("parent_directory").unwrap();
                // let item_name = req.match_info().get("item_name").unwrap();
                let parent_directory = item_struct.parent_directory.as_str();
                let item_name = item_struct.item_name.as_str();

                if item_name.is_empty() {

                    // cd ..

                    if parent_directory == "/kmp/webadmin" || db::query_existence_from_storage_table_by_mount(parent_directory) {

                        let all_file = linux::query_file_in_partition(&password, parent_directory);
                        Ok(
                            HttpResponse::Ok().json(
                                DriveItemExtended {
                                    drive_label: "Local Content Storage".to_string(),
                                    item_list: all_file,
                                }
                            )
                        )
                    }
                    else {
                        let splited_parent_directory = parent_directory.split("/").collect::<Vec<&str>>();
                        let previous_directory = parent_directory.strip_suffix(&format!("/{}", splited_parent_directory[splited_parent_directory.len()-1])).unwrap();

                        let all_file = linux::query_file_in_partition(&password, &previous_directory);
                        Ok(
                            HttpResponse::Ok().json(
                                all_file
                            )
                        )
                    }
                }
                else {

                    // cd $forward_directory

                    let directory_path = format!("{}/{}", parent_directory, item_name);

                    let all_file = linux::query_file_in_partition(&password, &directory_path);
                    Ok(
                        HttpResponse::Ok().json(
                            DriveItemExtended {
                                drive_label: "Removeable Device".to_string(),
                                item_list: all_file,
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

#[get("/private/api/settings/storage/device/rwpermission/status/{drive_partuuid}")]
pub async fn get_storage_device_rw_permission(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            
            if passwordstatus {
                let drive_partuuid = req.match_info().get("drive_partuuid").unwrap();
                let mount = db::query_mount_by_uuid_from_storage_table(drive_partuuid);
                let is_mount_rw = linux::is_read_writeable(&mount);

                match is_mount_rw {
                    true => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Success".to_string(),
                                reason: "rw".to_string()
                            }
                        )
                    ),
                    false => Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "ro".to_string()
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

