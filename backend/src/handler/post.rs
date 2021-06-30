use std::io::Write;
use actix_files::NamedFile;
use actix_multipart::Multipart;
use futures::{
    StreamExt, 
    TryStreamExt,
};
use pam::{
    Authenticator, 
    PasswordConv,
};
use actix_web::{
    HttpResponse, 
    Result, 
    post, 
    web,
};
use crate::{
    file, 
    linux, 
    security,
    db,  
    tool,
    structs::*,
    handler::helper::{
        return_httpsresponse_from_config_var_named_external_zone,
        return_httpsresponse_from_config_named_conf_external_zone,
    }
};

#[post("/private/api/settings/export")]
pub async fn post_settings_export(backupparam: web::Json<BackupParam>) -> Result<NamedFile, HttpResponse> {

    let (_username, _password, olddate) = db::query_logindata();
    let password_status = tool::comparedate(olddate);
    let (code, _output, _error, filepath) = linux::tar_config(&backupparam.filename);
    let mut backup_name: String = String::new();
    let tar_status: bool;

    match code {
        0 => tar_status = true,
        _ => tar_status = false,
    }
    
    if tar_status {
        backup_name = security::encrypt_file(&filepath, &backupparam.password);
    }
    
    if password_status {
        Ok(NamedFile::open(backup_name).unwrap())
    }
    else {
        Err(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password_timeout".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/user/login")]
pub async fn post_pam_login(logindata: web::Json<LoginParam>) -> Result<HttpResponse> {
    
    // setup authenticator with system-auth
    let mut auth: Authenticator<PasswordConv> = Authenticator::with_password("system-auth")
        .unwrap();

    // Now, give username password to be authenticated 
    auth.get_handler()
        .set_credentials(&logindata.username, &logindata.password);

    // Now, Authenticate and Listen for feedback
    if  auth.authenticate()
            .is_ok() && 
        auth
            .open_session()
            .is_ok() {
        db::update_logindata(&logindata.username, &logindata.password);
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Success".to_string(),
                    reason: "".to_string(),
                }
            )
        )
    }
    else{
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "wrong_username_or_password".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/user/password")]
pub async fn post_reset_password(passwdparam: web::Json<PasswdParam>) -> Result<HttpResponse> {

    let (username, _password, olddate) = db::query_logindata();
    let passwordstatus: bool = tool::comparedate(olddate);

    if passwordstatus {
        let (code, _output, error) = linux::passwd(&username, &passwdparam.old_password, &passwdparam.new_password);
        if code == 0 {
            db::update_logindata(&username, &passwdparam.new_password);
            Ok(
                HttpResponse::Ok().json(
                    HttpResponseCustom {
                        operation_status: "Success".to_string(),
                        reason: "".to_string(),
                    }
                )
            )
        }
        else{
            Ok(
                HttpResponse::Ok().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: error,
                    }
                )
            )
        }
    }
    else{
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password_timeout".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/hostapd")]
pub async fn post_hostapd_settings(hostapdparam: web::Json<HostapdParam>) -> Result<HttpResponse> {

    let deserial_param: HostapdParam = HostapdParam {
        ssid: hostapdparam.ssid.clone(),
        hide_ssid: hostapdparam.hide_ssid,
        hw_mode: hostapdparam.hw_mode.clone(),
        channel: hostapdparam.channel.clone(),
        wpa: hostapdparam.wpa,
        passphrase: hostapdparam.passphrase.clone(),
        hw_n_mode: hostapdparam.hw_n_mode,
        qos: hostapdparam.qos,
    };

    let (write_hostapd_status,passwordstatus, move_hostapd_status, restart_hostapd_status) = file::config_hostapd(deserial_param);

    if write_hostapd_status {
        if passwordstatus {
            if move_hostapd_status {
                if restart_hostapd_status{
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Success".to_string(),
                                reason: "".to_string(),
                            }
                        )
                    )
                }
                else{
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "restart_service_error".to_string(),
                            }
                        )
                    )
                }
            }
            else {
                Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom {
                            operation_status: "Failed".to_string(),
                            reason: "move_file_error".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::Ok().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "password_timeout".to_string(),
                    }
                )
            )
        }
    }
    else{
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "write_file_error".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/wirelessnetwork")]
pub async fn post_wireless_network_settings(wirelessnetworkparam: web::Json<WirelessNetworkParam>) -> Result<HttpResponse> {
    
    let deserial_param: WirelessNetworkParam = WirelessNetworkParam {
        router_ip: wirelessnetworkparam.router_ip.clone(),
        netmask: wirelessnetworkparam.netmask.clone(),
        range_start: wirelessnetworkparam.range_start.clone(),
        range_end: wirelessnetworkparam.range_end.clone(),
        default_lease: wirelessnetworkparam.default_lease.clone(),
        max_lease: wirelessnetworkparam.max_lease.clone(),
        dns: wirelessnetworkparam.dns.clone(),
        timezone: wirelessnetworkparam.timezone.clone(),
    };

    let (
        write_default_named_status, 
        move_default_named_status
    ) = file::config_named();

    let (
        write_networkd_status, 
        write_named_status, 
        passwordstatus, 
        move_networkd_status, 
        move_named_status, 
        restart_networkd_status, 
        restart_named_service
    ) = file::config_systemd_networkd_wireless(deserial_param);

    if write_networkd_status && write_named_status && write_default_named_status{
        if passwordstatus {
            if move_networkd_status && move_named_status && move_default_named_status{
                if restart_networkd_status && restart_named_service {
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Success".to_string(),
                                reason: "".to_string(),
                            }
                        )
                    )
                }
                else{
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "restart_service_error".to_string(),
                            }
                        )
                    )
                }
            }
            else {
                Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom {
                            operation_status: "Failed".to_string(),
                            reason: "move_file_error".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::Ok().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "password_timeout".to_string(),
                    }
                )
            )
        }
    }
    else{
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "write_file_error".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/reset")]
pub async fn post_settings_reset() -> Result<HttpResponse> {
    
    let (username, password, olddate) = db::query_logindata();
    let passwordstatus: bool = tool::comparedate(olddate);
    let hostapdparam: HostapdParam = HostapdParam::default();
    let wirelessnetworkparam: WirelessNetworkParam = WirelessNetworkParam::default();
    let passwdparam: PasswdParam = PasswdParam {
        old_password: password.clone(),
        new_password: String::from("123"),
    };

    if passwordstatus{
        let (
            write_wireless_networkd_status, 
            write_named_status, 
            _passwordstatus, 
            move_wireless_networkd_status, 
            move_named_status, 
            restart_wireless_networkd_status, 
            restart_named_status
        ) = file::config_systemd_networkd_wireless(wirelessnetworkparam);

        let (
            write_wired_networkd_status, 
            _passwordstatus, 
            move_wired_networkd_status, 
            restart_wired_networkd_status
        ) = file::config_systemd_networkd_wired_dynamic();
    
        let write_networkd_status = write_wired_networkd_status && write_wireless_networkd_status;
        let move_networkd_status = move_wired_networkd_status && move_wireless_networkd_status;
        let restart_networkd_status = restart_wired_networkd_status && restart_wireless_networkd_status;

        let (
            write_hostapd_status,
            _passwordstatus, 
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
                            HttpResponse::Ok().json(
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
                        HttpResponse::Ok().json(
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
                    HttpResponse::Ok().json(
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
                HttpResponse::Ok().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "write_file_error".to_string(),
                    }
                )
            )
        }
    }
    else{
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password_timeout".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/import")]
pub async fn post_settings_import(restoreparam: web::Json<RestoreParam>, mut payload: Multipart) -> Result<HttpResponse> {
    
    let (_username, password, olddate) = db::query_logindata();
    let filepath: String = String::new();
    let untar_status: bool;
    let password_status: bool = tool::comparedate(olddate);
    let mv_etc_status: bool;
    let restart_service_status: bool;

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
        if password_status {
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
                        HttpResponse::Ok().json(
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
                    HttpResponse::Ok().json(
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
                HttpResponse::Ok().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "password_timeout".to_string(),
                    }
                )
            )
        }
    }
    else {
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "untar_file_error".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/setting/wirednetwork/static")]
pub async fn post_static_wired_network(staticwirednetworkparam: web::Json<StaticWiredNetworkParam>) -> Result<HttpResponse> {
    
    let (_username, _password, olddate) = db::query_logindata();
    let password_status: bool = tool::comparedate(olddate);

    if password_status{
        let deserialparam = StaticWiredNetworkParam {
            internet_ip: staticwirednetworkparam.internet_ip.clone(),
            netmask: staticwirednetworkparam.netmask.clone(),
            gateway: staticwirednetworkparam.gateway.clone(),
            dns: staticwirednetworkparam.dns.clone(),
        };

        let (
            write_networkd_status, 
            passwordstatus, 
            move_networkd_status, 
            restart_networkd_status
        ) = file::config_systemd_networkd_wired_static(deserialparam);

        if write_networkd_status{
            if passwordstatus {
                if move_networkd_status {
                    if restart_networkd_status {
                        db::update_static_wan_networkd(
                            &staticwirednetworkparam.internet_ip, 
                            &staticwirednetworkparam.netmask, 
                            &staticwirednetworkparam.gateway, 
                            &staticwirednetworkparam.dns
                        );
                        Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom {
                                    operation_status: "Success".to_string(),
                                    reason: "".to_string(),
                                }
                            )
                        )
                    }
                    else{
                        Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: "restart_service_error".to_string(),
                                }
                            )
                        )
                    }
                }
                else {
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "move_file_error".to_string(),
                            }
                        )
                    )
                }
            }
            else{
                Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom {
                            operation_status: "Failed".to_string(),
                            reason: "password_timeout".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::Ok().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "write_file_error".to_string(),
                    }
                )
            )
        }
    }
    else {
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password-timeout".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/setting/wirednetwork/dynamic")]
pub async fn post_dynamic_wired_network() -> Result<HttpResponse> {

    let (_username, _password, olddate) = db::query_logindata();
    let password_status: bool = tool::comparedate(olddate);

    if password_status{

        let (
            write_networkd_status, 
            passwordstatus, 
            move_networkd_status, 
            restart_networkd_status
        ) = file::config_systemd_networkd_wired_dynamic();

        if write_networkd_status{
            if passwordstatus {
                if move_networkd_status {
                    if restart_networkd_status {
                        Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom {
                                    operation_status: "Success".to_string(),
                                    reason: "".to_string(),
                                }
                            )
                        )
                    }
                    else{
                        Ok(
                            HttpResponse::Ok().json(
                                HttpResponseCustom {
                                    operation_status: "Failed".to_string(),
                                    reason: "restart_service_error".to_string(),
                                }
                            )
                        )
                    }
                }
                else {
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "move_file_error".to_string(),
                            }
                        )
                    )
                }
            }
            else{
                Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom {
                            operation_status: "Failed".to_string(),
                            reason: "password_timeout".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::Ok().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "write_file_error".to_string(),
                    }
                )
            )
        }
    }
    else {
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password-timeout".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/dns/domain_name/creation")]
pub async fn post_create_domain_name(domain_name_struct: web::Json<CreateDomainName>) -> Result<HttpResponse> {

    let (_username, _password, olddate) = db::query_logindata();
    let password_status: bool = tool::comparedate(olddate);

    if password_status{

        let domain_name = domain_name_struct.domain_name.clone();
        db::insert_domain_name_into_dnszones(domain_name.as_str());
        return_httpsresponse_from_config_named_conf_external_zone()
    }
    else {
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password-timeout".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/dns/zone_record/creation")]
pub async fn post_add_zone_record(zone_record_struct: web::Json<PartialZoneRecords>) -> Result<HttpResponse> {

    let (_username, _password, olddate) = db::query_logindata();
    let password_status: bool = tool::comparedate(olddate);

    if password_status{

        db::insert_into_zonerecords(zone_record_struct.clone());
        return_httpsresponse_from_config_var_named_external_zone(zone_record_struct.foreign_key.as_str())
    }
    else {
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password-timeout".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/time/timezone")]
pub async fn post_set_timezone(timezone_struct: web::Json<Timezone>) -> Result<HttpResponse> {

    let (_username, password, olddate) = db::query_logindata();
    let password_status: bool = tool::comparedate(olddate);

    if password_status{
        let (code, _output, error) = linux::set_timezone(&password, &timezone_struct.timezone);
        match code {
            0 => Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom {
                            operation_status: "Success".to_string(),
                            reason: "".to_string(),
                        }
                    )
                ),
            _ => Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom {
                            operation_status: "Failed".to_string(),
                            reason: format!("{}", error),
                        }
                    )
                ),
        }
    }
    else {
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password-timeout".to_string(),
                }
            )
        )
    }
}

#[post("/private/api/settings/time/timedate")]
pub async fn post_set_time(time_struct: web::Json<TimeDate>) -> Result<HttpResponse> {

    let (_username, password, olddate) = db::query_logindata();
    let password_status: bool = tool::comparedate(olddate);

    let timedate = format!("{} {}", time_struct.date, time_struct.time);

    if password_status{
        let (code, _output, error) = linux::set_time(&password, &timedate);
        match code {
            0 => Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom {
                            operation_status: "Success".to_string(),
                            reason: "".to_string(),
                        }
                    )
                ),
            _ => Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom {
                            operation_status: "Failed".to_string(),
                            reason: format!("{}", error),
                        }
                    )
                ),
        }
    }
    else {
        Ok(
            HttpResponse::Ok().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "password-timeout".to_string(),
                }
            )
        )
    }
}
