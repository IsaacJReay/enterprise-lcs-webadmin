use actix_web::{
    HttpResponse, 
    Result, 
    get,
    HttpRequest,
};
use crate::{config, db, linux, security, structs::{DriveDescription, HttpResponseCustom, PartUUID}, tool};

#[get("/private/api/settings/storage/status")]
pub async fn get_storage_page(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let (_username, password) = db::users::query_logindata();
                let all_partitions = linux::storage::get_all_partitions();
                let mut local_content_storage = linux::storage::get_partition_information("/kmp");
                local_content_storage.drive_partuuid = PartUUID {
                    drive_partuuid: "kmp".to_string(),
                };
                local_content_storage.drive_label = "Local Content Storage".to_string();
                let mut mounted_partitions_mount: Vec<String> = Vec::new();
                let mut not_mounted_partitions: Vec<String> = Vec::new();
                let mut drives_description: Vec<DriveDescription> = vec![local_content_storage];
                
                let mut drives_description_length: usize = drives_description.len();
                let mut mount_operation_status: bool = true;
                for each_partition in all_partitions {
                    let (_code, partition_filesystem_type, _error) = linux::storage::get_partition_filesystem_type(&each_partition);

                    if  partition_filesystem_type == "exfat" && partition_filesystem_type == "vfat" && partition_filesystem_type == "ntfs" {
                        let is_mounted = db::storage::query_existence_from_storage_table_by_path(&each_partition);
                        match is_mounted {
                            true => mounted_partitions_mount.push(db::storage::query_mount_by_path_from_storage_table(&each_partition)),
                            false => not_mounted_partitions.push(each_partition),
                        }
                    }
                }

                for each_partition in not_mounted_partitions {
                    let (code, output, _error) = linux::storage::mount_partition(&password, &each_partition);
                    match code {
                        0 => mounted_partitions_mount.push(output),
                        _ => {
                            mount_operation_status = false;
                            break;
                        },
                    }   
                }

                if mount_operation_status {
                    for each_mount in mounted_partitions_mount {
                        let current_drive_description = linux::storage::get_partition_information(&each_mount);
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

#[get("/private/api/settings/storage/device/status/{drive_partuuid}")]
pub async fn get_storage_device_page_test(req: HttpRequest) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::users::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            // let (_username, _password) = db::query_logindata();
            if passwordstatus {
                let drive_partuuid = req.match_info().get("drive_partuuid").unwrap();
                if drive_partuuid != "kmp" {
                    let path = db::storage::query_mount_by_uuid_from_storage_table(&drive_partuuid);
                    let top = config::generate_file_system_struct(&path, "Removeable Device");
                    Ok(
                        HttpResponse::Ok()
                            .json(top)
                    )
                }
                else{
                    let  top = config::generate_file_system_struct("/kmp/webadmin", "Local Content Storage");                  
                    Ok(
                        HttpResponse::Ok()
                            .json(
                                top
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