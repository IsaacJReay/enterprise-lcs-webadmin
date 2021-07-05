use sqlite::State;
use wildmatch::WildMatch;
use ipnetwork::Ipv4Network;
use crate::{ 
    tool,
    security, 
    structs::{
        DnsZones,
        ZoneRecords,
        PartialZoneRecords,
        DnsId,
    }, 
};
use std::{
    convert::TryInto, 
    path::Path,
    fs::{
        File,
        remove_file,
    }, 
    io::{
        self, 
        BufRead, 
        Lines, 
        BufReader,
    }, 
};

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

pub fn read_hostapd() -> (String, bool, u8, String, u8, String, bool, bool){
    let mut ssid: String = String::new();
    let mut hide_ssid: bool = 0 != 0;
    let mut channel: u8 = 0;
    let mut wpa: u8 = 0;
    let mut hw_mode: String = String::new();
    let mut passphrase: String = String::new();
    let mut hw_n_mode: bool = 0 != 0;
    let mut qos: bool = 0 != 0;

    if let Ok(file) = read_lines("/etc/hostapd/hostapd.conf") {
        // Consumes the iterator, returns an (Optional) String
        for lines in file {
            if let Ok(line) = lines {
                if WildMatch::new("ssid=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    ssid = format!("{}",line_vec[1]);
                }
                else if WildMatch::new("ignore_broadcast_ssid=*").matches(&line) {
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    hide_ssid = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap() != 0;
                }
                else if WildMatch::new("wpa=*").matches(&line) {
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    wpa = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap();
                }
                else if WildMatch::new("hw_mode=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    hw_mode = format!("{}",line_vec[1]);
                }
                else if WildMatch::new("channel=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    channel = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap();
                }
                else if WildMatch::new("wpa_passphrase=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    passphrase = format!("{}",line_vec[1]);
                }
                else if WildMatch::new("ieee80211n=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    hw_n_mode = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap() != 0;
                }
                else if WildMatch::new("wmm_enabled=*").matches(&line){
                    let split = &line.split("=");
                    let line_vec = split.clone().collect::<Vec<&str>>();
                    qos = format!("{}",line_vec[1]).to_owned().as_str().parse::<u8>().unwrap() != 0;
                }
            }
        }
    }
    (ssid, hide_ssid, wpa, hw_mode, channel,passphrase, hw_n_mode, qos)
}

pub fn read_named() -> String{

    let mut raw: String = String::new();
    let mut dns: String = String::new();

    if let Ok(file) = read_lines("/etc/named.conf.options") {
        // Consumes the iterator, returns an (Optional) String
        for lines in file {
            if let Ok(line) = lines {
                if WildMatch::new("*forwarders {*").matches(&line){
                    raw = line.replace("forwarders {", "").replace("}", "").replace(";", "");
                }
            }
        }
    }

    let splitedraw = raw.split_whitespace();
    let collectedraw: Vec<&str> = splitedraw.collect();
    for ip in collectedraw{
        dns = format!("{} {}", ip, dns)
    }
    dns
}

pub fn read_resolvconf() -> String {

    let mut resolv_dns: String = String::new();

    if let Ok(file) = read_lines("/etc/resolv.conf") {
        // Consumes the iterator, returns an (Optional) String
        for lines in file {
            if let Ok(line) = lines {
                if WildMatch::new("nameserver *").matches(&line){
                    let dns_vec = line.split_whitespace().collect::<Vec<&str>>();
                    resolv_dns = format!("{} {}", resolv_dns, dns_vec[1]);
                }
            }
        }
    }
    resolv_dns
}

pub fn read_wlan_networkd() -> (String, String, String, String, String, String, String, String) {

    let mut full_ip_address: String = String::new();
    let mut pool_offset: String = String::new();
    let mut pool_size: String = String::new();
    let mut dns: String = String::new();
    let mut default_lease: String = String::new();
    let mut max_lease: String = String::new();
    let mut timezone: String = String::new();

    if let Ok(file) = read_lines("/etc/systemd/network/20-wireless.network") {
        // Consumes the iterator, returns an (Optional) String
        for lines in file {
            if let Ok(line) = lines {
                if WildMatch::new("Address=*").matches(&line){
                    let vec = line.split("=").collect::<Vec<&str>>();
                    full_ip_address = vec[1].to_owned();
                }
                else if WildMatch::new("PoolOffset=*").matches(&line){
                    let vec = line.split("=").collect::<Vec<&str>>();
                    pool_offset = vec[1].to_owned();
                }
                else if WildMatch::new("PoolSize=*").matches(&line){
                    let vec = line.split("=").collect::<Vec<&str>>();
                    pool_size = vec[1].to_owned();
                }
                else if WildMatch::new("DNS=*").matches(&line){
                    let vec = line.split("=").collect::<Vec<&str>>();
                    dns = vec[1].to_owned();
                }
                else if WildMatch::new("DefaultLeaseTimeSec=*").matches(&line){
                    let vec = line.split("=").collect::<Vec<&str>>();
                    default_lease = vec[1].to_owned();
                }
                else if WildMatch::new("MaxLeaseTimeSec=*").matches(&line){
                    let vec = line.split("=").collect::<Vec<&str>>();
                    max_lease = vec[1].to_owned();
                }
                else if WildMatch::new("Timezone=*").matches(&line){
                    let vec = line.split("=").collect::<Vec<&str>>();
                    timezone = vec[1].to_owned();
                }
            }
        }
    }
    let splited_full_ip_address: Vec<&str> = full_ip_address.split("/").collect::<Vec<&str>>();
    let router_ip = splited_full_ip_address[0].to_string();
    let prefix_length = splited_full_ip_address[1].to_string();

    let struct_ipv4network = Ipv4Network::new(
        router_ip.parse().unwrap(), 
        prefix_length.parse::<u8>().unwrap()
    )   
        .unwrap();

    let netmask = struct_ipv4network.mask().to_string();

    let network_address = tool::to_binary(struct_ipv4network.network().to_string());

    let binary_range_start = network_address + pool_offset.parse::<usize>().unwrap();
    let binary_range_end = binary_range_start + pool_size.parse::<usize>().unwrap();

    let range_start = tool::from_binary(format!("{:b}", binary_range_start));
    let range_end = tool::from_binary(format!("{:b}", binary_range_end));

    (router_ip, netmask, range_start, range_end, dns, default_lease, max_lease, timezone)

}

pub fn read_wan_networkd() -> (bool, String, String, String, String){

    let mut dhcp_status: bool = true;
    let mut full_ip_address: String = String::new();
    let mut gateway: String = String::new();
    let mut dns: String = String::new();
    let internet_ip: String;
    let netmask: String;

    if let Ok(file) = read_lines("/etc/systemd/network/20-wired.network") {
        // Consumes the iterator, returns an (Optional) String
        for lines in file {
            if let Ok(line) = lines {
                if WildMatch::new("DHCP=*").matches(&line){
                    let split = line.split("=").collect::<Vec<&str>>();
                    match split[1] {
                        "yes" => dhcp_status = true,
                        _ => dhcp_status = false,
                    }
                    
                }  
            }
        }
    }
    if dhcp_status == false {
        if let Ok(file) = read_lines("/etc/systemd/network/20-wired.network") {
            // Consumes the iterator, returns an (Optional) String
            for lines in file {
                if let Ok(line) = lines {
                    if WildMatch::new("Address=*").matches(&line){
                        let vec = line.split("=").collect::<Vec<&str>>();
                        full_ip_address = vec[1].to_owned();
                    }
                    else if WildMatch::new("Gateway=*").matches(&line){
                        let vec = line.split("=").collect::<Vec<&str>>();
                        gateway = vec[1].to_owned();
                    }
                    else if WildMatch::new("DNS=*").matches(&line){
                        let vec = line.split("=").collect::<Vec<&str>>();
                        dns = vec[1].to_owned();
                    }
                }
            }
        }
        let splited_ip = full_ip_address.split("/").collect::<Vec<&str>>();
        internet_ip = splited_ip[0].to_string();
        let prefix = splited_ip[1].parse::<u8>().unwrap();

        let ip_struct = Ipv4Network::new(internet_ip.parse().unwrap(), prefix).unwrap();

        netmask = ip_struct.mask().to_string();
    }
    else {
        let (_macaddr_output, ipaddr_output, eth0_subnetmask, gateway_output) = read_eth0();
        let resolve_dns = read_resolvconf();
        internet_ip = ipaddr_output;
        netmask = eth0_subnetmask;
        gateway = gateway_output;
        dns = resolve_dns;
    }    

    (dhcp_status, internet_ip, netmask, gateway, dns)
}

pub fn read_eth0() -> (String, String, String, String){
    let options = run_script::ScriptOptions::new();
    let eth0_macaddress = r#"ip address show eth0 | grep link/ether | awk -F' ' '{printf $2}'"#;   
    let eth0_ipaddress = r#"ip address | grep eth0 |grep inet | awk -F' ' '{printf $2}' |awk -F '/' '{printf $1}'"#;   
    let eth0_prefixaddr = r#"ip address | grep eth0 |grep inet | awk -F' ' '{printf $2}'"#;
    let eth0_gateway = r#"ip route | awk 'NR==1' | awk -F ' ' '{printf $3}'"#;
    let (_code, macaddr_output, _error) = run_script::run_script!(
        &format!("{}", eth0_macaddress),
        &vec![],
        &options
    ).unwrap();

    let (_code, ipaddr_output, _error) = run_script::run_script!(
        &format!("{}", eth0_ipaddress),
        &vec![],
        &options
    ).unwrap();

    let (_code, prefix_output, _error) = run_script::run_script!(
        &format!("{}", eth0_prefixaddr),
        &vec![],
        &options
    ).unwrap();

    let (_code, gateway_output, _error) = run_script::run_script!(
        &format!("{}", eth0_gateway),
        &vec![],
        &options
    ).unwrap();


    let subnet: Ipv4Network = prefix_output.parse().unwrap();
    let eth0_subnetmask: String = subnet.mask().to_string();

    (macaddr_output, ipaddr_output, eth0_subnetmask, gateway_output)
}

pub fn read_wlan0() -> (String, String, String){
    let options = run_script::ScriptOptions::new();
    let wlan0_macaddress = r#"ip address show wlan0 | grep link/ether | awk -F' ' '{printf $2}'"#;   
    let wlan0_ipaddress = r#"ip address | grep wlan0 |grep inet | awk -F' ' '{printf $2}' |awk -F '/' '{printf $1}'"#;   
    let wlan0_prefixaddr = r#"ip address | grep wlan0 |grep inet | awk -F' ' '{printf $2}'"#;
    
    let (_code, macaddr_output, _error) = run_script::run_script!(
        &format!("{}", wlan0_macaddress),
        &vec![],
        &options
    ).unwrap();

    let (_code, ipaddr_output, _error) = run_script::run_script!(
        &format!("{}", wlan0_ipaddress),
        &vec![],
        &options
    ).unwrap();

    let (_code, prefix_output, _error) = run_script::run_script!(
        &format!("{}", wlan0_prefixaddr),
        &vec![],
        &options
    ).unwrap();

    let subnet: Ipv4Network = prefix_output.parse().unwrap();
    let wlan0_subnetmask = subnet.mask().to_string();

    (macaddr_output, ipaddr_output, wlan0_subnetmask)
}

pub fn create_tables() {
    let result = remove_file("/tmp/lcs.db");
    let mut error: String = String::new();
    match result {
        Ok(()) => (),
        Err(err) => {
            error = err.to_string(); 
        },
    }
    
    if &error == "Operation not permitted (os error 1)"{
        eprintln!("{}", &error);
    }
    else {
        let connection = sqlite::open("/tmp/lcs.db").unwrap();
        connection
            .execute(
r#"
CREATE TABLE dnszones (id TXT, domain_name TXT, status TXT);
CREATE TABLE logindata (variable TXT, value TXT);
CREATE TABLE zonerecords(id TXT, subdomain_name TXT, type TXT, address TXT, foreign_key TXT);
CREATE TABLE tokentable(token TXT);

INSERT INTO logindata VALUES ('username', 'NULL');
INSERT INTO logindata VALUES ('password', 'NULL');
"#,)
            .unwrap();
    }
}

pub fn insert_into_token_table(token: &str){
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    connection
        .execute(
            format!("INSERT INTO tokentable VALUES ('{}');", token)
        )
            .unwrap()
}

pub fn delete_from_token_table(token: &str) {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let auth_split_whitespace_vec = token.split_ascii_whitespace().collect::<Vec<&str>>();

    connection
        .execute(
            format!("DELETE FROM tokentable WHERE token='{}';", auth_split_whitespace_vec[1])
        )
            .unwrap()
}

pub fn query_token(token: &str) -> bool {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let auth_split_whitespace_vec = token.split_ascii_whitespace().collect::<Vec<&str>>();


    let mut check_empty_statement = connection
        .prepare(
            format!("SELECT EXISTS(SELECT token FROM tokentable WHERE token='{}' LIMIT 1);", auth_split_whitespace_vec[1])
        )
            .unwrap();

    check_empty_statement.next().unwrap();
    let output: u64 = check_empty_statement.read::<i64>(0).unwrap().try_into().unwrap();

    output!=0
    
    
}

pub fn update_logindata(username: &str, password: &str){
    
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let encrypted_password = security::encrypt(password.to_string(), security::padding_convert("Koompi-Onelab"));
    
    connection
        .execute(
format!("
UPDATE logindata SET value = '{}' WHERE variable = 'username';
UPDATE logindata SET value = '{}' WHERE variable = 'password';
", &username, &encrypted_password)
        )
        .unwrap()
}

pub fn update_static_wan_networkd(internet_ip: &str, netmask: &str, gateway: &str, dns: &str) {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    connection
        .execute(format!("
UPDATE wan_networkd SET value = '{}' WHERE variable = 'connection_type';
UPDATE wan_networkd SET value = '{}' WHERE variable = 'address';
UPDATE wan_networkd SET value = '{}' WHERE variable = 'netmask';
UPDATE wan_networkd SET value = '{}' WHERE variable = 'gateway';
UPDATE wan_networkd SET value = '{}' WHERE variable = 'dns';
        ", true, internet_ip, netmask, gateway, dns)
    )
        .unwrap()

}

pub fn query_logindata() -> (String, String){
    
    let mut username: String = String::new();
    let mut password: String = String::new();
    
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let mut statement = connection
        .prepare("SELECT value FROM logindata")
        .unwrap();

    let mut increment: u8 = 0;

    while let State::Row = statement.next().unwrap() {
        if increment == 0 {
            username = statement.read::<String>(0).unwrap();
        }
        else if increment == 1 {
            password = statement.read::<String>(0).unwrap();
        }
        increment = increment + 1;
    };

    let decrypted_password = security::decrypt(password, security::padding_convert("Koompi-Onelab"));

    (username, decrypted_password)
}

pub fn read_dnszones() -> Vec<DnsZones> {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut zones_vec: Vec<DnsZones> = Vec::new();
    let mut zones_vec_size: usize = 0;
    let mut check_empty_statement = connection
        .prepare("SELECT COUNT(*) FROM dnszones;")
        .unwrap();
    
    check_empty_statement.next().unwrap();
    let line_count: u64 = check_empty_statement.read::<i64>(0).unwrap().try_into().unwrap();

    if line_count != 0 {
        let mut read_statement = connection
            .prepare("SELECT id,domain_name,status FROM dnszones;")
            .unwrap();
        
        while let State::Row = read_statement.next().unwrap() {
            let current_id: String = read_statement.read::<String>(0).unwrap();
            let current_domain_name: String = read_statement.read::<String>(1).unwrap();
            let current_status: bool = read_statement.read::<String>(2).unwrap().parse().unwrap();
            
            zones_vec.insert(
                zones_vec_size, 
                DnsZones {
                    id: current_id,
                    domain_name: current_domain_name,
                    status: current_status,
                }
            );
            zones_vec_size += 1;
        }
    }
    zones_vec 
}

pub fn read_zonerecords_by_foreign_key(foreign_key: &str) -> Vec<ZoneRecords> {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut records_vec: Vec<ZoneRecords> = Vec::new();
    let mut records_vec_size: usize = 0;
    let mut read_statement = connection
        .prepare(format!("SELECT * FROM zonerecords WHERE foreign_key = '{}';", foreign_key))
        .unwrap();
    while let State::Row = read_statement.next().unwrap() {
        let current_id: String = read_statement.read::<String>(0).unwrap();
        let current_subdomain_name: String = read_statement.read::<String>(1).unwrap();
        let current_type: String = read_statement.read::<String>(2).unwrap();
        let current_address: String = read_statement.read::<String>(3).unwrap();
        let current_foriegn_key: String = read_statement.read::<String>(4).unwrap();
        
        records_vec.insert(
            records_vec_size,
            ZoneRecords{
                id_zonerecords: DnsId{
                    id: current_id,
                },
                partial_zonerecords: PartialZoneRecords{
                    subdomain_name: current_subdomain_name,
                    dns_type: current_type,
                    address: current_address,
                    foreign_key: current_foriegn_key
                },
            }
        );
        records_vec_size += 1;
    }
    records_vec
    
}

pub fn populate_dnszones() {
    let mut active_zones: Vec<String> = Vec::new();
    let mut inactive_zones: Vec<String> = Vec::new();
    let mut active_zones_id = 0;
    let mut increment = 1;
    let mut inactive_zones_id = 0;
    if let Ok(file) = read_lines("/etc/named.conf.external.zones") {
        for lines in file {
            if let Ok(line) = lines {
                if WildMatch::new("zone*").matches(&line){
                    let line_vec = &line.split_whitespace().clone().collect::<Vec<&str>>();
                    active_zones.insert(active_zones_id, line_vec[1].to_string().replace("\"", ""));
                    active_zones_id += 1;
                }
                else if WildMatch::new(r#"# zone*"#).matches(&line){
                    // println!("ab");
                    let line_vec = &line.split_whitespace().clone().collect::<Vec<&str>>();
                    inactive_zones.insert(inactive_zones_id, line_vec[2].to_string().replace("\"", ""));
                    inactive_zones_id += 1;                    
                }
            }
        }
    }
    for domain_name in active_zones {
        insert_domain_name_into_dnszones(&domain_name);
        insert_status_into_dnszone_by_id(increment.to_string().as_str(), true);
        increment += 1;
    }
    for domain_name in inactive_zones {
        insert_domain_name_into_dnszones(&domain_name);
        insert_status_into_dnszone_by_id(increment.to_string().as_str(), false);
        increment += 1;
    }    
}

pub fn populate_zonerecords(){
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut vec_domain_name: Vec<String> = Vec::new();
    let mut vec_len: usize = 0;
    let mut find_domain_name = connection
        .prepare("SELECT domain_name FROM dnszones")
        .unwrap();

    while let State::Row = find_domain_name.next().unwrap() {
        vec_domain_name.insert(vec_len, find_domain_name.read::<String>(0).unwrap());
        vec_len += 1;
    }
    
    if vec_len != 0 {
        for increment in 0..vec_len {
            if let Ok(file) = read_lines(format!("/var/named/{}.external.zone", vec_domain_name[increment])) {
                let mut loop_pusher: usize = 0;
                for lines in file {
                    if loop_pusher > 7 {
                        if let Ok(line) = lines {
                            let line_vec = &line.split_whitespace().clone().collect::<Vec<&str>>();
                            if line_vec.len() > 3 {
                                let zone_struct: PartialZoneRecords = PartialZoneRecords{
                                    subdomain_name: line_vec[0].to_string(),
                                    dns_type: line_vec[2].to_string(),
                                    address: line_vec[3].to_string(),
                                    foreign_key: (increment+1).to_string(),
                                };
                                insert_into_zonerecords(zone_struct);
                            }
                        }
                    }
                    else {
                        loop_pusher += 1;
                    }    
                }
            }   
        }
    }
}

pub fn insert_domain_name_into_dnszones(domain_name: &str) {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut latest_id: u8 = 0;
    let mut statement = connection
        .prepare("SELECT id FROM dnszones")
        .unwrap();

    while let State::Row = statement.next().unwrap() {
        let current_id: u8 = statement.read::<i64>(0).unwrap().try_into().unwrap();
        if current_id > latest_id {
            latest_id = current_id;
        }
    }
    latest_id = latest_id + 1;

    connection
            .execute(format!(
                r#"
                INSERT INTO dnszones VALUES ('{}', '{}', 'false');
                "#,latest_id, domain_name),
            )
            .unwrap();
}

pub fn insert_status_into_dnszone_by_id(id: &str, status: bool) {
    
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    connection.execute(
        format!(
            r#"UPDATE dnszones SET status = '{}' WHERE id = '{}';"#, status, id
        )
    )
        .unwrap()
}

pub fn insert_into_zonerecords(zone_struct: PartialZoneRecords) {
    let subdomain_name = zone_struct.subdomain_name;
    let dns_type = zone_struct.dns_type;
    let address = zone_struct.address;
    let foreign_key = zone_struct.foreign_key;
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut latest_id: u64 = 0;
    let mut statement = connection
        .prepare("SELECT id,foreign_key FROM zonerecords")
        .unwrap();

    while let State::Row = statement.next().unwrap() {
        let current_id: u64 = statement.read::<i64>(0).unwrap().try_into().unwrap();
        let current_key: String = statement.read::<String>(1).unwrap();
        if current_id > latest_id  && foreign_key == current_key {
            latest_id = current_id;
        }
    }
    latest_id = latest_id + 1;

    connection
            .execute(format!(
                r#"
                INSERT INTO zonerecords VALUES ('{}', '{}', '{}', '{}', '{}');
                "#,latest_id, subdomain_name, dns_type, address, foreign_key),
            )
            .unwrap();
}

pub fn delete_from_dnszones_by_id(id: &str){
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut dnszones_latest_id: u64 = 0;
    let u64_dnszones_id = id.parse::<u64>().unwrap();
    let mut statement = connection
        .prepare("SELECT id FROM dnszones")
        .unwrap();

    while let State::Row = statement.next().unwrap() {
        dnszones_latest_id = statement.read::<i64>(0).unwrap().try_into().unwrap();
    }

    let row_affected: u64 = dnszones_latest_id - u64_dnszones_id;

    connection
            .execute(format!(
                r#"
                DELETE FROM dnszones WHERE id='{}';
                DELETE FROM zonerecords WHERE foreign_key='{}';
                "#,id,id)
            )
            .unwrap();

    if row_affected > 0 {
        for new_id in u64_dnszones_id..dnszones_latest_id {
            connection
                .execute(format!(
                    r#"
                    UPDATE dnszones SET id = '{}' WHERE id = '{}';
                    UPDATE zonerecords SET foreign_key = '{}' WHERE foreign_key = '{}';
                    "#, new_id, new_id+1, new_id, new_id+1),
                )
                .unwrap();
        }
    }
}

pub fn delete_from_zonerecords_by_id(id: &str, foreign_key: &str) {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut latest_id: u64 = 0;
    let u64_id = id.parse::<u64>().unwrap();
    let mut statement = connection
        .prepare("SELECT id,foreign_key FROM zonerecords")
        .unwrap();

    while let State::Row = statement.next().unwrap() {
        let current_key = statement.read::<String>(1).unwrap();
        if foreign_key == current_key {
            latest_id = statement.read::<i64>(0).unwrap().try_into().unwrap();
        }
    }

    let row_affected: u64 = latest_id - u64_id;

    connection
            .execute(format!(
                r#"
                DELETE FROM zonerecords WHERE id='{}' AND foreign_key='{}'
                "#,id, foreign_key)
            )
            .unwrap();

    if row_affected > 0 {
        for new_id in u64_id..latest_id {
            connection
            .execute(format!(
                r#"
                UPDATE zonerecords SET id = '{}' WHERE id = '{}' AND foreign_key='{}';
                "#, new_id, new_id+1, foreign_key),
            )
            .unwrap();
        }
    }
}

pub fn update_domain_name_by_foreign_key(foreign_key: &str, domain_name: &str) {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let _statement = connection
        .prepare(format!("UPDATE dnszones SET domain_name = '{}' WHERE id = '{}'", domain_name, foreign_key))
        .unwrap();
}

