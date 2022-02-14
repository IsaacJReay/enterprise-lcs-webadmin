mod templates;

use walkdir::WalkDir;
use std::{
    io::prelude::*,
    fs::{
        File, 
        metadata
    }
};
use crate::{
    db, 
    linux,
    structs::{
        HostapdParam, 
        StaticWiredNetworkParam, 
        WirelessNetworkParam,
        PartialZoneRecords,
        ZoneRecords,
        DirectoryInfo,
        ItemMetaData,
        PathPartition,
    }, 
};

pub fn generate_file_system_struct(linux_path: &str, drive_label: &str) -> DirectoryInfo {
    let root_path = WalkDir::new(linux_path);
    let root_path_metadata = std::fs::metadata(linux_path).unwrap();
    let root_path_length = PathPartition::new(linux_path).parts.len();
    let mut main_directory_info = DirectoryInfo::new(
        linux_path, 
        Some(
            ItemMetaData::new(
                root_path_metadata
            )
        )
    );

    for path in root_path {
        let entry_path = path.as_ref().unwrap().path();
        
        let current_metadata = ItemMetaData::new(
            match entry_path.metadata() {
                Ok(result) => result,
                Err(_err) => entry_path.symlink_metadata().unwrap()
            }
        );
        let path_str = entry_path.clone().to_str().unwrap();
        let path = PathPartition::new(path_str);
        
        build_tree(
            &mut main_directory_info, 
            &path.parts, 
            Some(
                current_metadata
            ), 
            root_path_length
        );
    }
    main_directory_info.name = drive_label.to_string();
    main_directory_info
}

pub fn build_tree(current_node: &mut DirectoryInfo, current_parts_list: &Vec<String>, current_metadata: Option<ItemMetaData>, current_parts_depth: usize) {
    if current_parts_depth < current_parts_list.len() {
        let current_item_name = &current_parts_list[current_parts_depth];

        let mut new_node = match current_item_name.starts_with(".") {
            true => current_node,
            false => match current_node.find_child(&current_item_name) {
                Some(stored_directory_info) => stored_directory_info,
                None => current_node.add_child(
                    DirectoryInfo::new(
                        &current_item_name, 
                        current_metadata.clone()
                    )
                ),
            },
        };

        build_tree(
            &mut new_node, 
            current_parts_list, 
            current_metadata.clone(), 
            current_parts_depth + 1
        );
    }
}

pub fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}

pub fn createfile(filename: &str, content: &[u8]) -> std::io::Result<()> {

    let mut file = File::create(filename)?;
    file.write_all(content)?;
    Ok(())

}

pub fn config_hostapd(hostapdparam: HostapdParam) -> (bool, bool, bool){

    let write_hostapd_status: bool;
    let move_hostapd_status: bool;
    let restart_hostapd_status: bool;
    let (_username, password) = db::users::query_logindata();

    let conf: String = templates::gen_hostapd_conf(&hostapdparam.ssid, hostapdparam.hide_ssid, &hostapdparam.hw_mode, &hostapdparam.channel, hostapdparam.wpa, &hostapdparam.passphrase, hostapdparam.hw_n_mode, hostapdparam.qos);

    let result = createfile("hostapd.conf", &conf.as_bytes());
    match result {
        Ok(()) => write_hostapd_status = true,
        Err(_err) => write_hostapd_status = false,
    }

    let (code,_output,_error) = linux::storage::move_filedir_root(&password, "hostapd.conf", "/etc/hostapd");
    match &code {
        0 => move_hostapd_status = true,
        _ => move_hostapd_status = false,
    }

    let (code,_output,_error) = linux::restartservice(&password, "hostapd");
    match &code {
        0 => restart_hostapd_status = true,
        _ => restart_hostapd_status = false,
    }

    (write_hostapd_status, move_hostapd_status, restart_hostapd_status)
}

pub fn config_systemd_networkd_wireless(wirelessnetworkparam: WirelessNetworkParam) -> (bool, bool, bool, bool, bool, bool){

    // Create Status variables
    let (_username, password) = db::users::query_logindata();

    let write_networkd_status: bool;
    let write_acl_status: bool;
    let write_options_status: bool;
    let write_named_status: bool;

    let move_networkd_status: bool;
    let move_acl_status: bool;
    let move_options_status: bool;
    let move_named_status: bool;

    let restart_networkd_status: bool;
    let restart_named_status: bool;


    // Create File Variables

    let networkd_conf: String = templates::gen_systemd_networkd_wireless(
        &wirelessnetworkparam.router_ip, 
        &wirelessnetworkparam.netmask, 
        &wirelessnetworkparam.range_start, 
        &wirelessnetworkparam.range_end, 
        &wirelessnetworkparam.dns, 
        &wirelessnetworkparam.default_lease, 
        &wirelessnetworkparam.max_lease, 
        &wirelessnetworkparam.timezone
    );
    
    let named_conf_acl: String = templates::gen_named_conf_acl(
        &wirelessnetworkparam.router_ip, 
        &wirelessnetworkparam.netmask
    );

    let named_conf_options: String = templates::gen_named_conf_options(&wirelessnetworkparam.dns);


    // Match Create File statuses
    {
        let result = createfile("named.conf.acl", &named_conf_acl.as_bytes());
        match result {
            Ok(()) => write_acl_status = true,
            Err(_err) => write_acl_status = false,        
        }

        let result = createfile("named.conf.options", &named_conf_options.as_bytes());
        match result {
            Ok(()) => write_options_status = true,
            Err(_err) => write_options_status = false,        
        }

        let result = createfile("20-wireless.network", &networkd_conf.as_bytes());
        match result {
            Ok(()) => write_networkd_status = true,
            Err(_err) => write_networkd_status = false,
        }

        // Match Move File Statuses

        let (code,_output,_error) = linux::storage::move_filedir_root(&password, "20-wireless.network", "/etc/systemd/network/");
        match &code {
            0 => move_networkd_status = true,
            _ => move_networkd_status = false,
        }

        let (code,_output,_error) = linux::storage::move_filedir_root(&password, "named.conf.acl", "/etc/");
        match &code {
            0 => move_acl_status = true,
            _ => move_acl_status = false,
        }

        let (code,_output,_error) = linux::storage::move_filedir_root(&password, "named.conf.options", "/etc/");
        match &code {
            0 => move_options_status = true,
            _ => move_options_status = false,
        }


        //Match Restart Service Status

        let (code,_output,_error) = linux::restartservice(&password, "systemd-networkd");
        match &code {
            0 => restart_networkd_status = true,
            _ => restart_networkd_status = false,
        }

        let (code,_output,_error) = linux::restartservice(&password, "named");
        match &code {
            0 => restart_named_status = true,
            _ => restart_named_status = false,
        }
    }
    
    write_named_status = write_acl_status && write_options_status;
    move_named_status = move_acl_status && move_options_status;

    (
    write_networkd_status, 
    write_named_status,  
    move_networkd_status, 
    move_named_status, 
    restart_networkd_status, 
    restart_named_status
    )
}

pub fn config_named() -> (bool, bool) {
    
    let (_username, password) = db::users::query_logindata();
    let named_conf: String = templates::gen_named_conf();
    let named_conf_zones: String = templates::gen_named_conf_internal_zones();
    let named_conf_logging: String = templates::gen_named_conf_logging();

    let write_conf_status: bool;
    let write_zones_status: bool;
    let write_logging_status: bool;
    let write_named_status: bool;

    let move_conf_status: bool;
    let move_zones_status: bool;
    let move_logging_status: bool;
    let move_named_status: bool;

    let result = createfile("named.conf", &named_conf.as_bytes());
    match result {
        Ok(()) => write_conf_status = true,
        Err(_e) => write_conf_status = false,
    }

    let result = createfile("named.conf.logging", &named_conf_logging.as_bytes());
    match result {
        Ok(()) => write_logging_status = true,
        Err(_e) => write_logging_status = false,
    }
    let result = createfile("named.conf.internal.zones", &named_conf_zones.as_bytes());
    match result {
        Ok(()) => write_zones_status = true,
        Err(_e) => write_zones_status = false,
    }

    let (code, _output, _error) = linux::storage::move_filedir_root(&password, "named.conf", "/etc/");
    match &code {
        0 => move_conf_status = true,
        _ => move_conf_status = false,
    }

    let (code,_output,_error) = linux::storage::move_filedir_root(&password, "named.conf.logging", "/etc/");
    match &code {
        0 => move_logging_status = true,
        _ => move_logging_status = false,
    }
    
    let (code,_output,_error) = linux::storage::move_filedir_root(&password, "named.conf.internal.zones", "/etc/");
    match &code {
        0 => move_zones_status = true,
        _ => move_zones_status = false,
    }

    write_named_status = write_logging_status && write_conf_status && write_zones_status;
    move_named_status = move_conf_status && move_logging_status && move_zones_status;

    (write_named_status, move_named_status)

}

pub fn config_systemd_networkd_wired_static(staticwirednetworkparam: StaticWiredNetworkParam) -> (bool, bool, bool) {
    let (_username, password) = db::users::query_logindata();
    let move_networkd_status: bool;
    let restart_networkd_status: bool;
    let write_networkd_status: bool;

    let networkd_conf = templates::gen_systemd_networkd_wired_static(
        &staticwirednetworkparam.internet_ip, 
        &staticwirednetworkparam.netmask, 
        &staticwirednetworkparam.gateway, 
        &staticwirednetworkparam.dns
    );

    let result = createfile("20-wired.network", networkd_conf.as_bytes());

    match result {
        Ok(()) => write_networkd_status = true,
        Err(_err) => write_networkd_status = false,
    }

    let (code, _output, _error) = linux::storage::move_filedir_root(&password, "20-wired.network", "/etc/systemd/network/");

    match code {
        0 => move_networkd_status = true,
        _ => move_networkd_status = false,
    }

    let (code, _output, _error) = linux::restartservice(&password, "systemd-networkd");

    match code {
        0 => restart_networkd_status = true,
        _ => restart_networkd_status = false,
    }

    (write_networkd_status, move_networkd_status, restart_networkd_status)

}

pub fn config_systemd_networkd_wired_dynamic() -> (bool, bool, bool) {
    let (_username, password) = db::users::query_logindata();
    let move_networkd_status: bool;
    let restart_networkd_status: bool;
    let write_networkd_status: bool;

    let networkd_conf = templates::gen_systemd_networkd_wired_dynamic();

    let result = createfile("20-wired.network", networkd_conf.as_bytes());
    match result {
        Ok(()) => write_networkd_status = true,
        Err(_err) => write_networkd_status = false,
    }

    let (code, _output, _error) = linux::storage::move_filedir_root(&password, "20-wired.network", "/etc/systemd/network/");

    match code {
        0 => move_networkd_status = true,
        _ => move_networkd_status = false,
    }

    let (code, _output, _error) = linux::restartservice(&password, "systemd-networkd");
    
    match code {
        0 => restart_networkd_status = true,
        _ => restart_networkd_status = false,
    }

    (write_networkd_status, move_networkd_status, restart_networkd_status)
}

pub fn config_name_conf_external_zones() -> (bool, bool, bool, bool) {
    let (_username, password) = db::users::query_logindata();
    let zone_vec  = db::named::read_dnszones();
    // println!("Inside {:#?}", zone_vec);
    let mut record_vec: Vec<ZoneRecords>;
    let mut write_var_zone_status: bool = true;
    let mut move_var_zone_status: bool = true;
    let cleanup_exzone_status: bool;
    let cleanup_var_named_status: bool;
    let write_exzone_status: bool;
    let move_exzone_status: bool;
    let restart_named_status: bool;


    let conf: String = templates::gen_named_conf_external_zones();

    let (code, _output, _error) = linux::storage::remove_filedir_root(&password, "/etc/named.conf.external.zones");
    match code {
        0 => cleanup_exzone_status = true,
        _ => cleanup_exzone_status = false,
    }

    let (code, _output, _error) = linux::storage::remove_filedir_root(&password, "/var/named/*.external.zone");
    match code {
        0 => cleanup_var_named_status = true,
        _ => cleanup_var_named_status = false,
    }

    let result = createfile("named.conf.external.zones", conf.as_bytes());
    match result {
        Ok(()) => write_exzone_status = true,
        Err(_err) => write_exzone_status = false,
    }

    for increments in 0..zone_vec.len(){
        let filename: String = zone_vec[increments].domain_name.to_owned() + ".external.zone";
        // println!("{}", filename);
    
        record_vec = db::named::read_zonerecords_by_foreign_key(&zone_vec[increments].id.to_owned());
        let mut partial_record_vec: Vec<PartialZoneRecords> = Vec::new();

        for partial_increments in 0..record_vec.len(){
            partial_record_vec.insert(
                partial_increments, 
                PartialZoneRecords{
                    subdomain_name: record_vec[increments].partial_zonerecords.subdomain_name.to_owned(),
                    dns_type: record_vec[increments].partial_zonerecords.dns_type.to_owned(),
                    address: record_vec[increments].partial_zonerecords.address.to_owned(),
                    foreign_key: record_vec[increments].partial_zonerecords.foreign_key.to_owned(),
                },
            );
        }

        let current_conf: String = templates::gen_var_named_one_zone(partial_record_vec);
        let result = createfile(&filename, current_conf.as_bytes());
        match result {
            Ok(()) => (),
            Err(_err) => write_var_zone_status = false,
        }
    };

    let (code, _ouput, _error) = linux::storage::move_filedir_root(&password, "named.conf.external.zones", "/etc/");
    match code {
        0 => move_exzone_status = true,
        _ => move_exzone_status = false,
    }

    for increments in 0..zone_vec.len(){
        let filename: String = zone_vec[increments].domain_name.to_owned() + ".external.zone";
        let (code, _ouput, _error) = linux::storage::move_filedir_root(&password, &filename, "/var/named/");
        match code {
            0 => (),
            _ => move_var_zone_status = false,
        }
    };

    let (code, _output, _error) = linux::restartservice(&password, "named");
    match code {
        0 => restart_named_status = true,
        _ => restart_named_status = false,
    }

    let write_named_status: bool = write_exzone_status && write_var_zone_status;
    let move_named_status: bool = move_exzone_status && move_var_zone_status;
    let cleanup_named_status: bool = cleanup_exzone_status && cleanup_var_named_status;
    

    (cleanup_named_status, write_named_status, move_named_status, restart_named_status)
}

pub fn config_var_named_external_zones(zone_vec: Vec<PartialZoneRecords>) -> (bool, bool, bool){
    let (_username, password) = db::users::query_logindata();

    let dns_vec = db::named::read_dnszones();
    let mut filename: String = String::new();

    for increments in 0..dns_vec.len(){
        if dns_vec[increments].id == zone_vec[0].foreign_key{
            filename = dns_vec[increments].domain_name.to_owned() + ".external.zone";
        }
    }

    let write_var_zone_status: bool;
    let move_var_zone_status: bool;
    let restart_named_status: bool;

    let conf: String = templates::gen_var_named_one_zone(zone_vec);
    let result = createfile(&filename, conf.as_bytes());
    match result {
        Ok(()) => write_var_zone_status = true,
        Err(_err) => write_var_zone_status = false,
    }

    let (code, _output, _error) = linux::storage::move_filedir_root(&password, &&filename, "/var/named");
    match code {
        0 => move_var_zone_status = true,
        _ => move_var_zone_status = false,
    }

    let (code, _output, _error) = linux::restartservice(&password, "named");
    match code {
        0 => restart_named_status = true,
        _ => restart_named_status = false,
    }
    (write_var_zone_status, move_var_zone_status, restart_named_status)
}