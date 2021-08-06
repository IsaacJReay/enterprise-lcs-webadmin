use actix_web::{
    HttpResponse, 
    Result, 
    get, 
    web,
    HttpRequest,
};
use crate::{db, linux, security, structs::{DriveDescription, ForeignKey, HostapdParam, HttpResponseCustom, ItemNamePath, NTPStatus, PartUUID, StaticWiredNetworkParam, StatusPageResult, TimeDate, TimeDateZone, TimeDateZoneNTP, Timezone, WanPageResult, WirelessNetworkParam, UserName}, tool};

#[get("/private/api/token/validation")]
pub async fn get_token_validated(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth) {
            let olddate = security::extract_token(auth);
            let (_username, _password) = db::query_logindata();


            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus{
                Ok(
                    HttpResponse::Ok().json(
                        HttpResponseCustom{
                            operation_status: "Success".to_string(),
                            reason: "token-valid".to_string(),
                        }
                    )
                )
            }
            else{
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
                    HttpResponseCustom{
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
                HttpResponseCustom{
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}

#[get("/private/api/user/query")]
pub async fn get_logindata(req: HttpRequest) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth) {
            let olddate = security::extract_token(auth);
            let (current_username, _password) = db::query_logindata();


            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus{
                Ok(
                    HttpResponse::Ok().json(
                        UserName{
                            username: current_username
                        }
                    )
                )
            }
            else{
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
                    HttpResponseCustom{
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
                HttpResponseCustom{
                    operation_status: "Failed".to_string(),
                    reason: "missing-token".to_string(),
                }
            )
        )
    }
}

#[get("/private/api/settings/status")]
pub async fn get_statuspage(req: HttpRequest) -> Result<HttpResponse> {

    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);

            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus{

                let (
                    ssid, 
                    _hide_ssid, 
                    _wpa,
                    hw_mode, 
                    channel, 
                    _passphrase, 
                    hw_n_mode, 
                    qos
                ) = db::read_hostapd();
                let (
                    eth0_macaddr, 
                    eth0_ipaddr, 
                    eth0_subnetmask, 
                    eth0_gateway
                ) = db::read_eth0();
                let (
                    wlan0_macaddr, 
                    wlan0_ipaddr, 
                    wlan0_subnetmask
                ) = db::read_wlan0();
                let wlan0_dns = db::read_named();

                Ok(
                    HttpResponse::Ok().json(
                        StatusPageResult {
                            wan_macaddress: eth0_macaddr,
                            wan_ip: eth0_ipaddr,
                            wan_netmask: eth0_subnetmask,
                            wan_gateway: eth0_gateway,
                            wlan_macaddress: wlan0_macaddr,
                            wlan_ip: wlan0_ipaddr,
                            wlan_netmask: wlan0_subnetmask,
                            wlan_dns: wlan0_dns,
                            wlan_ssid: ssid,
                            wlan_hw_mode: hw_mode,
                            wlan_channel: channel,
                            wlan_hw_n_mode: hw_n_mode,
                            wlan_qos: qos
                        }
                    )
                )
            }
            else{
                db::delete_from_token_table(auth);
                Ok(
                    HttpResponse::Gone().json(
                        HttpResponseCustom {
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

#[get("/private/api/settings/hostapd/status")]
pub async fn get_wifipage(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus {
                let (
                    read_ssid, 
                    read_hide_ssid, 
                    read_wpa,
                    read_hw_mode, 
                    read_channel, 
                    read_passphrase, 
                    read_hw_n_mode, 
                    read_qos
                ) = db::read_hostapd();

                Ok(
                    HttpResponse::Ok().json(
                        HostapdParam {
                            ssid: read_ssid,
                            hide_ssid: read_hide_ssid,
                            wpa: read_wpa,
                            hw_mode: read_hw_mode,
                            channel: read_channel,
                            passphrase: read_passphrase,
                            hw_n_mode: read_hw_n_mode,
                            qos: read_qos,
                        }
                    )
                )
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

#[get("/private/api/settings/wirednetwork/status")]
pub async fn get_wanpage(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);

            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus {
                let (
                    dhcp_status, 
                    read_internet_ip, 
                    read_netmask, 
                    read_gateway, 
                    read_dns
                ) = db::read_wan_networkd();
                Ok(
                    HttpResponse::Ok().json(
                        WanPageResult{
                            dhcp: dhcp_status,
                            wired_network_param: StaticWiredNetworkParam {
                                internet_ip: read_internet_ip,
                                netmask: read_netmask,
                                gateway: read_gateway,
                                dns: read_dns,
                            }
                        }
                    )
                )
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

#[get("/private/api/settings/wirelessnetwork/status")]
pub async fn get_wlanpage(req: HttpRequest) -> Result<HttpResponse>{
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);

            let passwordstatus: bool = tool::comparedate(olddate);

            if passwordstatus {
                let (
                    read_router_ip, 
                    read_netmask, 
                    read_range_start, 
                    read_range_end, 
                    read_dns, 
                    read_default_lease, 
                    read_max_lease, 
                    read_timezone
                ) = db::read_wlan_networkd();

                Ok(
                    HttpResponse::Ok().json(
                        WirelessNetworkParam{
                            router_ip: read_router_ip,
                            netmask: read_netmask,
                            range_start: read_range_start,
                            range_end: read_range_end,
                            dns: read_dns,
                            default_lease: read_default_lease,
                            max_lease: read_max_lease,
                            timezone: read_timezone,
                        }
                    )
                )
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

#[get("/private/api/settings/dns/domain_name/status")]
pub async fn get_domain_name_page(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);

            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let zone_vec = db::read_dnszones();

                Ok(
                    HttpResponse::Ok().json(
                        zone_vec
                    )
                )
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

#[get("/private/api/settings/dns/zone_records/status")]
pub async fn get_zone_record_page(req: HttpRequest, foreign_key: web::Json<ForeignKey>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let record_vec = db::read_zonerecords_by_foreign_key(&foreign_key.foreign_key);

                Ok(
                    HttpResponse::Ok().json(
                        record_vec
                    )
                )
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

#[get("/private/api/settings/time/status")]
pub async fn get_timedatepage(req: HttpRequest) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            if passwordstatus {
                let (_code, output, _error) = linux::query_date_for_display();
                let time_vec: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();
                let current_date = time_vec[0];
                let current_time = time_vec[1];
                let (_code, current_timezone, _error) = linux::query_timezone();
                let (_code, current_ntp_status, _error) = linux::query_ntp_status();
                let status: bool = if current_ntp_status == "active" {
                    true
                }
                else {
                    false
                };

                Ok(
                    HttpResponse::Ok().json(
                        TimeDateZoneNTP{
                            ntp_status: NTPStatus {
                                ntp_status: status,
                            },
                            timedatezone: TimeDateZone{
                                timedate: TimeDate{
                                    time: current_time.to_string(),
                                    date: current_date.to_string(),
                                },
                                timezone: Timezone {
                                    timezone: current_timezone.to_string(),
                                }
                            }
                        }
                    )
                )
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
                let (_code, output, _error) = linux::get_partitions();
                let all_partitions: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();
                let mut local_content_storage = linux::get_partition_information("/kmp");
                local_content_storage.drive_partuuid = PartUUID {
                    drive_partuuid: "kmp".to_string(),
                };
                local_content_storage.drive_label = "Local Content Storage".to_string();
                let mut mounted_partitions_mount: Vec<String> = Vec::new();
                let mut unmount_partitions: Vec<&str> = Vec::new();
                let mut drives_description: Vec<DriveDescription> = vec![local_content_storage];
                let mut mounted_partitions_length: usize = 0;
                let mut unmount_partitions_length: usize = 0;
                let mut drives_description_length: usize = drives_description.len();
                let mut mount_operation_status: bool = true;
                for each_partition in all_partitions {
                    let is_mounted = db::query_existence_from_storage_table_by_path(each_partition);
                    match is_mounted{
                        true => {
                            let mount = db::query_mount_by_path_from_storage_table(each_partition);
                            mounted_partitions_mount.insert(mounted_partitions_length, mount);
                            mounted_partitions_length +=1;
                        },
                        false => {
                            unmount_partitions.insert(unmount_partitions_length, each_partition);
                            unmount_partitions_length +=1;
                        },
                    }
                }

                for each_partition in unmount_partitions {
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

#[get("/private/api/settings/storage/device/status")]
pub async fn get_storage_device_page(req: HttpRequest, uuid_struct: web::Json<PartUUID>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            let (_username, password) = db::query_logindata();
            if passwordstatus {
                if &uuid_struct.drive_partuuid != "kmp" {
                    let path = db::query_mount_by_uuid_from_storage_table(&uuid_struct.drive_partuuid);
                    let all_file = linux::query_file_in_partition(&password, &path);
                    Ok(
                        HttpResponse::Ok().json(
                            all_file
                        )
                    )
                }
                else{
                    let all_file = linux::query_file_in_partition(&password, "/kmp");
                    Ok(
                        HttpResponse::Ok().json(
                            all_file
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

#[get("/private/api/settings/storage/device/directory/status")]
pub async fn get_storage_device_directory_page(req: HttpRequest, item_info_struct: web::Json<ItemNamePath>) -> Result<HttpResponse> {
    let auth_is_empty = req.headers().get("AUTHORIZATION").is_none();

    if !auth_is_empty{
        let auth = req.headers().get("AUTHORIZATION").unwrap().to_str().unwrap();
        if db::query_token(auth){
            let olddate = security::extract_token(auth);
            let passwordstatus: bool = tool::comparedate(olddate);
            let (_username, password) = db::query_logindata();
            if passwordstatus {

                if item_info_struct.item_name.is_empty() {

                    // cd ..
                    // println!("{}", db::query_existence_from_storage_table_by_mount(&item_info_struct.parent_directory));

                    if item_info_struct.parent_directory == "/kmp" || db::query_existence_from_storage_table_by_mount(&item_info_struct.parent_directory) {

                        let all_file = linux::query_file_in_partition(&password, &item_info_struct.parent_directory);
                        Ok(
                            HttpResponse::Ok().json(
                                all_file
                            )
                        )
                    }
                    else {
                        let splited_parent_directory = item_info_struct.parent_directory.split("/").collect::<Vec<&str>>();
                        let previous_directory = item_info_struct.parent_directory.strip_suffix(&format!("/{}", splited_parent_directory[splited_parent_directory.len()-1])).unwrap();

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

                    let directory_path = format!("{}/{}", item_info_struct.parent_directory, item_info_struct.item_name);

                    let all_file = linux::query_file_in_partition(&password, &directory_path);
                    Ok(
                        HttpResponse::Ok().json(
                            all_file
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