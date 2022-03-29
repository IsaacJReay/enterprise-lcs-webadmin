pub mod templates;
pub mod update;
pub mod named;

use walkdir::WalkDir;
use flate2::read::GzDecoder;
use reqwest::{
    StatusCode, header::{
        CONTENT_LENGTH, 
        RANGE
    }
};
use toml::value::Value;
use tar::Archive;
use std::{    
    str::FromStr,
    io::{
        BufReader,
        BufWriter,
        prelude::*
    },
    fs::{
        OpenOptions,
        File, 
        // metadata
    }
};
use crate::{
    CHUNK_SIZE, 
    linux,
    structs::{
        PartialRangeIter,
        HostapdParam, 
        StaticWiredNetworkParam, 
        WirelessNetworkParam,
        DirectoryInfo,
        ItemMetaData,
        PathPartition,
        ContentServerUpdate,
    }, 
};

pub fn insert_update_information_to_toml(status: (bool, bool), new_update_id: &str, new_update: &Value, is_sys_update: bool) {
    
    let output_file_location = match status.0 {
        true => "/tmp/update_db.toml.downloading",
        false => match status.1 {
            true => "/tmp/update_db.toml.installing",
            false => "/kmp/update_db.toml"
        }
    };

    let mut config = toml::from_str::<ContentServerUpdate>(&read_file(output_file_location)).unwrap();

    let map = match is_sys_update {
        true => config.sys_update.as_mut().unwrap(),
        false => config.patch_update.as_mut().unwrap()
    };

    map.insert(new_update_id.to_string(), new_update.to_owned());
    
    write_file(toml::to_string(&config).unwrap().as_bytes(), output_file_location);
}

pub fn remove_update_information_from_toml(status: (bool, bool), update_id: &str, is_sys_update: bool) {
    
    let output_file_location = match status.0 {
        true => "/tmp/update_db.toml.downloading",
        false => match status.1 {
            true => "/tmp/update_db.toml.installing",
            false => "/kmp/update_db.toml"
        }
    };

    let mut config = toml::from_str::<ContentServerUpdate>(&read_file(output_file_location)).unwrap();
    
    let map = match is_sys_update {
        true => config.sys_update.as_mut().unwrap(),
        false => config.patch_update.as_mut().unwrap()
    };

    map.remove(update_id).unwrap();

}

pub fn read_file(source_file: &str) -> String {

    match OpenOptions::new()
        .read(true)
        .open(source_file) {
            Ok(file_read) => {
                let mut read_buffer = BufReader::new(&file_read);
                let mut contents = String::new();
                read_buffer.read_to_string(&mut contents).unwrap();
                contents
            },
            Err(_error) => {
                String::new()
            }
        }
}

pub fn write_file(config: &[u8], destination_file: &str) {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(destination_file)
        .unwrap();
    let mut write_buffer = BufWriter::new(&file);
    write_buffer.write_all(config).unwrap();
}

pub fn continue_file(source_file: &str) -> File {

    OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(source_file)
        .unwrap()
}

pub fn untar_file(filename: &str, extract_location: &str) -> bool {
    let file = File::open(filename).unwrap();
    let mut archive = Archive::new(GzDecoder::new(file));
    match archive.unpack(extract_location) {
        Ok(_t) => true,
        Err(_err) => false
    }
}

fn download_file(download_link: &str, mut output_file: File) -> bool {
    
    let client = reqwest::blocking::Client::new();

    match client.head(download_link).send() {
        Ok(response) => {
            match response.headers().get(CONTENT_LENGTH) {
                Some(length) => {
                    match u64::from_str(length.to_str().unwrap()) {
                        Ok(length) => {
                            let startingpoint = output_file.metadata().unwrap().len();
                            match PartialRangeIter::new(startingpoint, length - 1, CHUNK_SIZE) {
                                Ok(whole_range) => {
                                    let mut success: bool = true;
                                    for range in whole_range {    
                                        match client.get(download_link).header(RANGE, &range).send(){
                                            Ok(mut response) => {
                                                let status = response.status();
                                                match !(status == StatusCode::OK || status == StatusCode::PARTIAL_CONTENT) {
                                                    true => std::io::copy(&mut response, &mut output_file).unwrap(),
                                                    false => {success = false; 0}
                                                }
                                            },
                                            Err(_) => {success = false; 0}
                                        };
                                    }
                                    match success {
                                        true => true,
                                        false => false,
                                    }
                                },
                                Err(_) => false,
                            }
                        },
                        Err(_) => false,
                    }
                },
                None => false,
            }
        },
        Err(_t) => false,
    }
}

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

pub fn createfile(filename: &str, content: &[u8]) -> std::io::Result<()> {

    let mut file = File::create(filename)?;
    file.write_all(content)?;
    Ok(())

}

pub fn config_hostapd(password: &str, hostapdparam: HostapdParam) -> Result<(), String>{

    let conf: String = templates::gen_hostapd_conf(&hostapdparam.ssid, hostapdparam.hide_ssid, &hostapdparam.hw_mode, &hostapdparam.channel, hostapdparam.wpa, &hostapdparam.passphrase, hostapdparam.hw_n_mode, hostapdparam.qos);
    createfile("hostapd.conf", &conf.as_bytes()).unwrap();
    linux::storage::move_filedir_root(&password, "hostapd.conf", "/etc/hostapd");
    match linux::restartservice(&password, "hostapd").0 {
        0 => Ok(()),
        _ => Err("hostapd_error")
    }?;

    Ok(())
}

pub fn config_systemd_networkd_wireless(password: &str, wlanparam: WirelessNetworkParam) -> Result<(), String>{

    let networkd_conf: String = templates::gen_systemd_networkd_wireless(
        &wlanparam.router_ip, 
        &wlanparam.netmask, 
        &wlanparam.range_start, 
        &wlanparam.range_end, 
        &wlanparam.dns, 
        &wlanparam.default_lease, 
        &wlanparam.max_lease, 
        &wlanparam.timezone
    );
    
    let named_conf_acl: String = templates::gen_named_conf_acl(
        &wlanparam.router_ip, 
        &wlanparam.netmask
    );

    let named_conf_options: String = templates::gen_named_conf_options(&wlanparam.dns);


    // Match Create File statuses

    createfile("named.conf.acl", &named_conf_acl.as_bytes()).unwrap();
    createfile("named.conf.options", &named_conf_options.as_bytes()).unwrap();
    createfile("20-wireless.network", &networkd_conf.as_bytes()).unwrap();
    linux::storage::move_filedir_root(&password, "20-wireless.network", "/etc/systemd/network/");
    linux::storage::move_filedir_root(&password, "named.conf.acl named.conf.options", "/etc/");
    match linux::restartservice(&password, "systemd-networkd").0 {
        0 => Ok(()),
        _ => Err("systemd-networkd_error".to_string())
    }?;
    match linux::restartservice(&password, "named").0 {
        0 => Ok(()),
        _ => Err("named_error".to_string())
    }?;

    Ok(())

}

pub fn config_systemd_networkd_wired_static(password: &str, staticwirednetworkparam: StaticWiredNetworkParam) -> Result<(), String> {
    
    let networkd_conf = templates::gen_systemd_networkd_wired_static(
        &staticwirednetworkparam.internet_ip, 
        &staticwirednetworkparam.netmask, 
        &staticwirednetworkparam.gateway, 
        &staticwirednetworkparam.dns
    );

    createfile("20-wired.network", networkd_conf.as_bytes()).unwrap();
    linux::storage::move_filedir_root(&password, "20-wired.network", "/etc/systemd/network/");
    match linux::restartservice(&password, "systemd-networkd").0 {
        0 => Ok(()),
        _ => Err("systemd-networkd_error")
    }?;

    Ok(())

}

pub fn config_systemd_networkd_wired_dynamic(password: &str) -> Result<(), String> {

    let networkd_conf = templates::gen_systemd_networkd_wired_dynamic();
    createfile("20-wired.network", networkd_conf.as_bytes()).unwrap();
    linux::storage::move_filedir_root(&password, "20-wired.network", "/etc/systemd/network/");
    match linux::restartservice(&password, "systemd-networkd").0 {
        0 => Ok(()),
        _ => Err("systemd-networkd_error"),
    }?;

    Ok(())
}
