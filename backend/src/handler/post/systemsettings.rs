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
    handler,
    security,
    linux,
    config,
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

    let (_username, _password) = handler::handle_validate_token_response(&req)?;

    let (code, _output, _error, filepath) = linux::systemsettings::tar_config(&backupparam.filename);
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
                    operation_status: "Failed".to_string(),
                    reason: format!("tar-{}-failed", backup_name),
                }
            )
        )
    }
}

#[post("/private/api/settings/import")]
pub async fn post_settings_import(req: HttpRequest, restoreparam: web::Json<RestoreParam>, mut payload: Multipart) -> Result<HttpResponse> {
    
    let (_username, password) = handler::handle_validate_token_response(&req)?;

    let filepath: String = String::new();
    let untar_status: bool;
    let mv_etc_status: bool;
    let restart_service_status: bool;

    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_type = field.content_disposition().unwrap();
        let filename = content_type.get_filename().unwrap();
        let filepath = format!("/tmp/{}", sanitize_filename::sanitize(&filename));

        // config::create is blocking operation, use threadpool
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

    let (code, _output, _error, untar_location) = linux::systemsettings::untar_config(&tar_file);

    match code {
        0 => untar_status = true,
        _ => untar_status = false,
    }

    let (code, _output, _error) = linux::storage::move_filedir_root(&password, format!("{}/*", untar_location).as_str(), "/etc/");
    match code {
        0 => mv_etc_status = true,
        _ => mv_etc_status = false,
    }

    let (code, _output, _error) = linux::restartservice(&password, "hostapd named systemd-networkd");

    match code {
        0 => restart_service_status = true,
        _ => restart_service_status = false,
    }

    match untar_status && mv_etc_status && restart_service_status {
        true => Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Success".to_string(),
                    reason: "".to_string(),
                }
            )
        ),
        false => Ok(
            HttpResponse::InternalServerError().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "file_ops_error".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/reset")]
pub async fn post_settings_reset(req: HttpRequest) -> Result<HttpResponse> {

    let (username, password) = handler::handle_validate_token_response(&req)?;
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
    ) = config::config_systemd_networkd_wireless(password.as_ref(), wirelessnetworkparam);

    let (
        write_wired_networkd_status, 
        move_wired_networkd_status, 
        restart_wired_networkd_status
    ) = config::config_systemd_networkd_wired_dynamic(password.as_ref());

    let write_networkd_status = write_wired_networkd_status && write_wireless_networkd_status;
    let move_networkd_status = move_wired_networkd_status && move_wireless_networkd_status;
    let restart_networkd_status = restart_wired_networkd_status && restart_wireless_networkd_status;

    let (
        write_hostapd_status,
        move_hostapd_status, 
        restart_hostapd_status
    ) = config::config_hostapd(password.as_ref(), hostapdparam);

    let (
        error_code, 
        _passwd_output, 
        _error_output
    ) = linux::passwd(
        username.as_str(),
            passwdparam.old_password.as_str(),
            passwdparam.new_password.as_str(),
        );

    match 
        error_code != 0 &&
        write_networkd_status && 
        write_hostapd_status && 
        write_named_status && 
        move_networkd_status && 
        move_hostapd_status && 
        move_named_status && 
        restart_networkd_status && 
        restart_hostapd_status && 
        restart_named_status 
    {
        true => Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Success".to_string(),
                    reason: "".to_string(),
                }
            )
        ),
        false => Ok(
            HttpResponse::InternalServerError().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "file_ops_error".to_string(),
                }
            )
        )
    }
}
