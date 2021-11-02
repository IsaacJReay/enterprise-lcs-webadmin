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
    config,
    structs::{
        DriveDescription, 
        HttpResponseCustom, 
        PartUUID, 
        ItemListExtended,
        Metadata,
        Dir,
        Path,
    }, 
};
use walkdir::WalkDir;

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
                    let all_file = linux::query_all_file_in_partition(&password, &path);
                    Ok(
                        HttpResponse::Ok().json(
                            ItemListExtended {
                                drive_label: "Removeable Device".to_string(),
                                item_list: all_file,
                            }

                        )
                    )
                }
                else{
                    let all_file = linux::query_all_file_in_partition(&password, "/kmp/webadmin");
                    Ok(
                        HttpResponse::Ok().json(
                            ItemListExtended {
                                drive_label: "Local Content Storage".to_string(),
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

#[get("/private/api/settings/storage/device/status/{drive_partuuid}")]
pub async fn get_storage_device_page_test(req: HttpRequest) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            // let (_username, _password) = db::query_logindata();
            if passwordstatus {
                let drive_partuuid = req.match_info().get("drive_partuuid").unwrap();
                if drive_partuuid != "kmp" {
                    let path = db::query_mount_by_uuid_from_storage_table(&drive_partuuid);
                    let root_path = WalkDir::new(&path);

                    let mut top = Dir::new(&path, None);
                    for path in root_path {
                        let entry_path = path.as_ref().unwrap().path();
                        let metadata = Metadata::new(entry_path.metadata().unwrap());
                        let path_str = entry_path.clone().to_str().unwrap();
                        let path = Path::new(path_str);
                        config::build_tree(&mut top, &path.parts, Some(metadata), 0);
                    }
                    Ok(
                        HttpResponse::Ok()
                            .json(serde_json::to_string_pretty(&top).unwrap()                             )
                    )
                }
                else{
                    let root_path = WalkDir::new("/kmp/webadmin");

                    let mut top = Dir::new("/kmp/webadmin", None);
                    for path in root_path {
                        let entry_path = path.as_ref().unwrap().path();
                        let metadata = Metadata::new(entry_path.metadata().unwrap());
                        let path_str = entry_path.clone().to_str().unwrap();
                        let path = Path::new(path_str);
                        config::build_tree(&mut top, &path.parts, Some(metadata), 0);
                    }
                    Ok(
                        HttpResponse::Ok()
                            .json(serde_json::to_string_pretty(&top).unwrap()                             )
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

    
    // std::fs::write("data.json", serde_json::to_string_pretty(&top).unwrap()).unwrap();
    // Ok(
    //     HttpResponse::Ok().json(
    //         serde_json::to_string_pretty(&top).unwrap()
    //     )
    // )


}