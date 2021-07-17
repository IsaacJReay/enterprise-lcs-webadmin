use std::io::Write;
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    post,
    Result,
};
use futures::{
    StreamExt, 
    TryStreamExt,
};
use crate::{
    db,
    tool,
    security,
    linux,
    file,
    structs::{
        BackupParam,
        HttpResponseCustom,
        RestoreParam,
        HostapdParam,
        WirelessNetworkParam,
        PasswdParam,
    },
};
use actix_multipart::Multipart;
use actix_files::NamedFile;

#[post("/private/api/settings/export")]
pub async fn post_settings_export(req: HttpRequest, backupparam: web::Json<BackupParam>) -> Result<NamedFile, HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, _password) = db::query_logindata();
            let password_status: bool = tool::comparedate(olddate);
            if password_status {
            
                let (code, _output, _error, filepath) = linux::tar_config(&backupparam.filename);
                let mut backup_name: String = String::new();
                let tar_status: bool;

                match code {
                    0 => tar_status = true,
                    _ => tar_status = false,
                }
                
                if tar_status {
                    backup_name = security::encrypt_file(&filepath, &backupparam.password);
                    Ok(NamedFile::open(backup_name).unwrap())
                }
                else {
                    Err(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom{
                                operation_status: "failed".to_string(),
                                reason: format!("tar-{}-failed", backup_name),
                            }
                        )
                    )
                }
            }
            else {
                db::delete_from_token_table(auth);
                Err(
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
            Err(
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
        Err(
            HttpResponse::Unauthorized().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/import")]
pub async fn post_settings_import(req: HttpRequest, restoreparam: web::Json<RestoreParam>, mut payload: Multipart) -> Result<HttpResponse> {
    
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (_username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            let filepath: String = String::new();
            let untar_status: bool;
            let mv_etc_status: bool;
            let restart_service_status: bool;

            if passwordstatus{

            // iterate over multipart stream
                while let Ok(Some(mut field)) = payload.try_next().await {
                    let content_type = field.content_disposition().unwrap();
                    let filename = content_type.get_filename().unwrap();
                    let filepath = format!("/tmp/{}", sanitize_filename::sanitize(&filename));

                    // File::create is blocking operation, use threadpool
                    let mut f = web::block(|| std::fs::File::create(filepath))
                        .await
                        .unwrap();

                    // Field in turn is stream of *Bytes* object
                    while let Some(chunk) = field.next().await {
                        let data = chunk.unwrap();
                        // filesystem operations are blocking, we have to use threadpool
                        f = web::block(move || f.write_all(&data).map(|_| f)).await.unwrap();
                    }
                }

                let tar_file = security::decrypt_file(&filepath, &restoreparam.password);

                let (code, _output, _error, untar_location) = linux::untar_config(&tar_file);

                match code {
                    0 => untar_status = true,
                    _ => untar_status = false,
                }

                let (code, _output, _error) = linux::mvfile(&password, format!("{}/*", untar_location).as_str(), "/etc/");
                match code {
                    0 => mv_etc_status = true,
                    _ => mv_etc_status = false,
                }

                let (code, _output, _error) = linux::restartservice(&password, "hostapd named systemd-networkd");

                match code {
                    0 => restart_service_status = true,
                    _ => restart_service_status = false,
                }
                if untar_status {
                    if mv_etc_status {
                        if restart_service_status {
                            Ok(
                                HttpResponse::Ok().json(
                                    HttpResponseCustom {
                                        operation_status: "Success".to_string(),
                                        reason: "".to_string(),
                                    }
                                )
                            )
                        }
                        else {
                            Ok(
                                HttpResponse::InternalServerError().json(
                                    HttpResponseCustom {
                                        operation_status: "Failed".to_string(),
                                        reason: "restart_service".to_string(),
                                    }
                                )
                            )
                        }
                    }
                    else {
                        Ok(
                            HttpResponse::InternalServerError().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: "move_file_error".to_string(),
                                }
                            )
                        )
                    }
                }
                else {
                    Ok(
                        HttpResponse::UnsupportedMediaType().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "untar_file_error".to_string(),
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

#[post("/private/api/settings/reset")]
pub async fn post_settings_reset(req: HttpRequest) -> Result<HttpResponse> {
    
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let (username, password) = db::query_logindata();
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus{
            
                let hostapdparam: HostapdParam = HostapdParam::default();
                let wirelessnetworkparam: WirelessNetworkParam = WirelessNetworkParam::default();
                let passwdparam: PasswdParam = PasswdParam {
                    old_password: password.clone(),
                    new_password: String::from("123"),
                };

                let (
                    write_wireless_networkd_status, 
                    write_named_status, 
                    move_wireless_networkd_status, 
                    move_named_status, 
                    restart_wireless_networkd_status, 
                    restart_named_status
                ) = file::config_systemd_networkd_wireless(wirelessnetworkparam);

                let (
                    write_wired_networkd_status, 
                    move_wired_networkd_status, 
                    restart_wired_networkd_status
                ) = file::config_systemd_networkd_wired_dynamic();
            
                let write_networkd_status = write_wired_networkd_status && write_wireless_networkd_status;
                let move_networkd_status = move_wired_networkd_status && move_wireless_networkd_status;
                let restart_networkd_status = restart_wired_networkd_status && restart_wireless_networkd_status;

                let (
                    write_hostapd_status,
                    move_hostapd_status, 
                    restart_hostapd_status
                ) = file::config_hostapd(hostapdparam);

                let (
                    error_code, 
                    _passwd_output, 
                    _error_output
                ) = linux::passwd(
                    username.as_str(),
                        passwdparam.old_password.as_str(),
                        passwdparam.new_password.as_str(),
                    );

                if write_networkd_status && write_hostapd_status && write_named_status {
                    if move_networkd_status && move_hostapd_status && move_named_status {
                        if restart_networkd_status && restart_hostapd_status && restart_named_status {
                            match error_code {
                                0 => {
                                        db::create_tables();
                                        Ok(
                                        HttpResponse::Ok().json(
                                            HttpResponseCustom {
                                                operation_status: "Success".to_string(),
                                                reason: "".to_string(),
                                            }
                                        )
                                    )
                                },
                                _ => Ok(
                                    HttpResponse::InternalServerError().json(
                                        HttpResponseCustom {
                                            operation_status: "Failed".to_string(),
                                            reason: "reset_password_error".to_string(),
                                        }
                                    )
                                ),
                            }
                        }
                        else{
                            Ok(
                                HttpResponse::InternalServerError().json(
                                    HttpResponseCustom {
                                        operation_status: "Failed".to_string(),
                                        reason: "restart_service_error".to_string(),
                                    }
                                )
                            )
                        }
                    }
                    else{
                        Ok(
                            HttpResponse::InternalServerError().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: "move_file_error".to_string(),
                                }
                            )
                        )
                    }
                }
                else {
                    Ok(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "write_file_error".to_string(),
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
