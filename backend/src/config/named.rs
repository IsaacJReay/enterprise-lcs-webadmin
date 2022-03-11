use crate::{ 
    linux,
    config,
    structs::{
        DnsZonesInfo,
        DnsRecords
    }, 
};

pub fn read_forward_dns_server() -> String {
    let config = config::read_file("/etc/named.conf.options");

    config
        .lines()
        .filter(|each_line| each_line.contains("forwarders") && each_line.contains("{") && each_line.contains("};"))
        .next()
        .unwrap()
        .split_whitespace()
        .filter(|each_item| *each_item != "forwarders" && *each_item != "{" && *each_item != "};")
        .map(|each_item| each_item.replace(";", ""))
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn read_zone_config_file(zone_is_internal: bool, include_records: bool) -> Vec<DnsZonesInfo> {

    let file_data = config::read_file(match zone_is_internal {
        true => "/etc/named.conf.internal.zones",
        false => "/etc/named.conf.external.zones",
    });

    let mut zone_vec: Vec<DnsZonesInfo> = Vec::new();
    file_data.lines().for_each(
        |each_line| {
            if each_line.contains("zone") && each_line.contains("IN {") {
                let splited_each_line = each_line.split_whitespace().collect::<Vec<&str>>();
                let (current_domain_name, curent_status) = match each_line.starts_with("#") {
                    true => (splited_each_line[2].replacen("\"", "", 2), false),
                    false => (splited_each_line[1].replacen("\"", "", 2), true)
                };
                let current_zone_record = match include_records {
                    true => Some(read_zone_record_file(zone_is_internal, &current_domain_name)),
                    false => None
                };
                zone_vec.push(
                    DnsZonesInfo {
                        domain_name: current_domain_name,
                        status: curent_status,
                        zone_record: current_zone_record
                    }
                );              
            }
        }
    );
    zone_vec
}

fn read_zone_record_file(zone_is_internal: bool, domain_name: &str) -> Vec<DnsRecords>{

    let file_data = config::read_file(&(format!("/var/named/{}",domain_name)+match zone_is_internal {
        true => ".internal.zone",
        false => ".external.zone",
    }));

    let mut vec_record: Vec<DnsRecords> = Vec::new();

    file_data
        .lines()
        .skip(8)
        .filter(|each_line| each_line.split_whitespace().count() > 3 )
        .for_each(|each_line| {
            let splited_each_line = each_line.split_whitespace().map(|each_str| each_str.to_string()).collect::<Vec<String>>();
            let current_subdomain_name = splited_each_line[0].clone();
            let (current_dns_type, current_address)= match splited_each_line[2].as_ref() {
                "MX" => (splited_each_line[2].clone()+" "+&splited_each_line[3], splited_each_line[4].clone()),
                "CAA" => (splited_each_line[2].clone(), each_line.split_whitespace().skip(3).map(|each_str| each_str.to_string()).collect::<Vec<String>>().join(" ")),
                _ => (splited_each_line[2].clone(), splited_each_line[3].clone())
            };
            vec_record.push(
                DnsRecords {
                    subdomain_name: current_subdomain_name,
                    dns_type: current_dns_type,
                    address: current_address
                }
            )

        }
    );

    vec_record
}

pub fn handle_new_domain_name_and_record(password:&str, args_struct: DnsZonesInfo, zone_is_internal: bool) -> Result<(), String>{
    let zone_location = match zone_is_internal {
        true => "internal",
        false => "external"
    };
    let zone_record_name: String = format!("{}.{}.zone", args_struct.domain_name, zone_location);
    let zone_config_name: String = format!("named.conf.{}.zones", zone_location);

    let mut vec_all_zone_config: Vec<DnsZonesInfo> = read_zone_config_file(zone_is_internal, true);
    let mut vec_new_zone_record: Vec<DnsRecords> = Vec::new();

    if let Some(zone) = vec_all_zone_config.clone().into_iter().filter(|each_zone| each_zone.domain_name == args_struct.domain_name).next() {
        let index = vec_all_zone_config.iter().position(|each_zone| each_zone.domain_name == zone.domain_name).unwrap();
        if let Some(records) = zone.zone_record {
            records.into_iter().for_each(|each_record| vec_new_zone_record.push(each_record));
        }
        vec_all_zone_config.remove(index);
        linux::storage::remove_filedir_root(password, &("/var/named/".to_owned()+zone_record_name.as_ref()));
    }
    if let Some(zone) = args_struct.zone_record {
        zone.into_iter().for_each(|each_zone| vec_new_zone_record.push(each_zone));
    }

    let (vec_new_zone_record, vec_new_zone_record_clone): (Option<Vec<DnsRecords>>,Option<Vec<DnsRecords>>) = match vec_new_zone_record.len() {
        0 => (None, None),
        _ => (Some(vec_new_zone_record.clone()), Some(vec_new_zone_record))
    };

    vec_all_zone_config.push(
        DnsZonesInfo{
            domain_name: args_struct.domain_name.to_owned(),
            status: args_struct.status,
            zone_record: vec_new_zone_record
        }
    );

    let mut zone_config = String::new();
    let zone_record = config::templates::generate_records_for_zone(&args_struct.domain_name,vec_new_zone_record_clone);

    vec_all_zone_config 
        .iter()
        .for_each(|each_zone| zone_config.push_str(config::templates::generate_zone_config(&each_zone.domain_name, each_zone.status, zone_is_internal).as_ref()));

    config::write_file(zone_config.as_bytes(), zone_config_name.as_ref());
    config::write_file(zone_record.as_bytes(), zone_record_name.as_ref());
    
    linux::storage::move_filedir_root(password, zone_config_name.as_ref(), "/etc");
    linux::storage::move_filedir_root(password, zone_record_name.as_ref(), "/var/named/");

    linux::chown_chmod(password, "root", "named", "770", &("/var/named/".to_owned()+zone_record_name.as_ref()));
    linux::chown_chmod(password, "root", "root", "775", &("/etc/".to_owned()+zone_config_name.as_ref()));

    let (code, output, _error) = linux::restartservice(password, "named");
    match code {
        0 => Ok(()),
        _ => Err(output)
    }
}

pub fn delete_domain_name(password: &str, domain_name: &str, zone_is_internal: bool) -> Result<(), String> {
    let zone_location = match zone_is_internal {
        true => "internal",
        false => "external"
    };
    let zone_record_name: String = format!("{}.{}.zone", domain_name, zone_location);
    let zone_config_name: String = format!("named.conf.{}.zones", zone_location);

    let mut vec_all_zone_config :Vec<DnsZonesInfo> = read_zone_config_file(zone_is_internal, true);
    vec_all_zone_config.remove(vec_all_zone_config.iter().position(|each_zone| each_zone.domain_name == domain_name).unwrap());
    linux::storage::remove_filedir_root(password, &("/var/named/".to_owned()+zone_record_name.as_ref()));
    let mut zone_config = String::new();
    vec_all_zone_config 
        .iter()
        .for_each(|each_zone| zone_config.push_str(config::templates::generate_zone_config(&each_zone.domain_name, each_zone.status, zone_is_internal).as_ref()));

    config::write_file(zone_config.as_bytes(), zone_config_name.as_ref());
    linux::storage::move_filedir_root(password, zone_config_name.as_ref(), "/etc");
    linux::chown_chmod(password, "root", "root", "775", &("/etc/".to_owned()+zone_config_name.as_ref()));

    let (code, output, _error) = linux::restartservice(password, "named");
    match code {
        0 => Ok(()),
        _ => Err(output)
    }
}

pub fn delete_dns_records(password: &str, domain_name: &str, subdomain_name: &str, zone_is_internal: bool) -> Result<(), String> {
    let zone_location = match zone_is_internal {
        true => "internal",
        false => "external"
    };
    let zone_record_name: String = format!("{}.{}.zone", domain_name, zone_location);
    let vec_all_zone_config :Vec<DnsZonesInfo> = read_zone_config_file(zone_is_internal, true);
    
    let mut vec_selected_zone_record = 
        vec_all_zone_config
            .iter()
            .filter(|each_zone| each_zone.domain_name == domain_name)
            .next()
            .unwrap()
            .to_owned()
            .zone_record
            .unwrap();
    
    vec_selected_zone_record.remove(vec_selected_zone_record.iter().position(|each_record| each_record.subdomain_name == subdomain_name).unwrap());   
    
    let zone_record = config::templates::generate_records_for_zone(domain_name,Some(vec_selected_zone_record));

    config::write_file(zone_record.as_bytes(), zone_record_name.as_ref());
    linux::storage::move_filedir_root(password, zone_record_name.as_ref(), "/var/named/");
    linux::chown_chmod(password, "root", "named", "770", &("/var/named/".to_owned()+zone_record_name.as_ref()));

    let (code, output, _error) = linux::restartservice(password, "named");
    match code {
        0 => Ok(()),
        _ => Err(output)
    }
}

pub fn rename_domain_name(password: &str, old_domain_name: &str, new_domain_name: &str, zone_is_internal: bool) -> Result<(), String>{
    
    let vec_all_zone_config: Vec<DnsZonesInfo> = read_zone_config_file(zone_is_internal, true);
    let mut selected_zone_config = vec_all_zone_config.iter().filter(|each_zone| each_zone.domain_name == old_domain_name).next().unwrap().to_owned();
    selected_zone_config.domain_name = new_domain_name.to_string();
    delete_domain_name(password, old_domain_name, zone_is_internal)?;
    handle_new_domain_name_and_record(password, selected_zone_config, zone_is_internal)?;
    Ok(())

}