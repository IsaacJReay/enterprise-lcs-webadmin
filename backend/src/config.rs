use std::{
    io::prelude::*,
    fs::{
        File, 
        metadata
    }
};
use ipnetwork::Ipv4Network;
use crate::{
    db, 
    structs::{
        HostapdParam, 
        StaticWiredNetworkParam, 
        WirelessNetworkParam,
        PartialZoneRecords,
        ZoneRecords,
    }, 
    linux, 
    tool,
};

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
    let (_username, password) = db::query_logindata();

    let conf: String = gen_hostapd_conf(&hostapdparam.ssid, hostapdparam.hide_ssid, &hostapdparam.hw_mode, &hostapdparam.channel, hostapdparam.wpa, &hostapdparam.passphrase, hostapdparam.hw_n_mode, hostapdparam.qos);

    let result = createfile("hostapd.conf", &conf.as_bytes());
    match result {
        Ok(()) => write_hostapd_status = true,
        Err(_err) => write_hostapd_status = false,
    }

    let (code,_output,_error) = linux::move_filedir_root(&password, "hostapd.conf", "/etc/hostapd");
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
    let (_username, password) = db::query_logindata();

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

    let networkd_conf: String = gen_systemd_networkd_wireless(
        &wirelessnetworkparam.router_ip, 
        &wirelessnetworkparam.netmask, 
        &wirelessnetworkparam.range_start, 
        &wirelessnetworkparam.range_end, 
        &wirelessnetworkparam.dns, 
        &wirelessnetworkparam.default_lease, 
        &wirelessnetworkparam.max_lease, 
        &wirelessnetworkparam.timezone
    );
    
    let named_conf_acl: String = gen_named_conf_acl(
        &wirelessnetworkparam.router_ip, 
        &wirelessnetworkparam.netmask
    );

    let named_conf_options: String = gen_named_conf_options(&wirelessnetworkparam.dns);


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

        let (code,_output,_error) = linux::move_filedir_root(&password, "20-wireless.network", "/etc/systemd/network/");
        match &code {
            0 => move_networkd_status = true,
            _ => move_networkd_status = false,
        }

        let (code,_output,_error) = linux::move_filedir_root(&password, "named.conf.acl", "/etc/");
        match &code {
            0 => move_acl_status = true,
            _ => move_acl_status = false,
        }

        let (code,_output,_error) = linux::move_filedir_root(&password, "named.conf.options", "/etc/");
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
    
    let (_username, password) = db::query_logindata();
    let named_conf: String = gen_named_conf();
    let named_conf_zones: String = gen_named_conf_internal_zones();
    let named_conf_logging: String = gen_named_conf_logging();

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

    let (code, _output, _error) = linux::move_filedir_root(&password, "named.conf", "/etc/");
    match &code {
        0 => move_conf_status = true,
        _ => move_conf_status = false,
    }

    let (code,_output,_error) = linux::move_filedir_root(&password, "named.conf.logging", "/etc/");
    match &code {
        0 => move_logging_status = true,
        _ => move_logging_status = false,
    }
    
    let (code,_output,_error) = linux::move_filedir_root(&password, "named.conf.internal.zones", "/etc/");
    match &code {
        0 => move_zones_status = true,
        _ => move_zones_status = false,
    }

    write_named_status = write_logging_status && write_conf_status && write_zones_status;
    move_named_status = move_conf_status && move_logging_status && move_zones_status;

    (write_named_status, move_named_status)

}

pub fn config_systemd_networkd_wired_static(staticwirednetworkparam: StaticWiredNetworkParam) -> (bool, bool, bool) {
    let (_username, password) = db::query_logindata();
    let move_networkd_status: bool;
    let restart_networkd_status: bool;
    let write_networkd_status: bool;

    let networkd_conf = gen_systemd_networkd_wired_static(
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

    let (code, _output, _error) = linux::move_filedir_root(&password, "20-wired.network", "/etc/systemd/network/");

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
    let (_username, password) = db::query_logindata();
    let move_networkd_status: bool;
    let restart_networkd_status: bool;
    let write_networkd_status: bool;

    let networkd_conf = gen_systemd_networkd_wired_dynamic();

    let result = createfile("20-wired.network", networkd_conf.as_bytes());
    match result {
        Ok(()) => write_networkd_status = true,
        Err(_err) => write_networkd_status = false,
    }

    let (code, _output, _error) = linux::move_filedir_root(&password, "20-wired.network", "/etc/systemd/network/");

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
    let (_username, password) = db::query_logindata();
    let zone_vec  = db::read_dnszones();
    // println!("Inside {:#?}", zone_vec);
    let mut record_vec: Vec<ZoneRecords>;
    let mut write_var_zone_status: bool = true;
    let mut move_var_zone_status: bool = true;
    let cleanup_exzone_status: bool;
    let cleanup_var_named_status: bool;
    let write_exzone_status: bool;
    let move_exzone_status: bool;
    let restart_named_status: bool;


    let conf: String = gen_named_conf_external_zones();

    let (code, _output, _error) = linux::remove_filedir_root(&password, "/etc/named.conf.external.zones");
    match code {
        0 => cleanup_exzone_status = true,
        _ => cleanup_exzone_status = false,
    }

    let (code, _output, _error) = linux::remove_filedir_root(&password, "/var/named/*.external.zone");
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
    
        record_vec = db::read_zonerecords_by_foreign_key(&zone_vec[increments].id.to_owned());
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

        let current_conf: String = gen_var_named_one_zone(partial_record_vec);
        let result = createfile(&filename, current_conf.as_bytes());
        match result {
            Ok(()) => (),
            Err(_err) => write_var_zone_status = false,
        }
    };

    let (code, _ouput, _error) = linux::move_filedir_root(&password, "named.conf.external.zones", "/etc/");
    match code {
        0 => move_exzone_status = true,
        _ => move_exzone_status = false,
    }

    for increments in 0..zone_vec.len(){
        let filename: String = zone_vec[increments].domain_name.to_owned() + ".external.zone";
        let (code, _ouput, _error) = linux::move_filedir_root(&password, &filename, "/var/named/");
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
    let (_username, password) = db::query_logindata();

    let dns_vec = db::read_dnszones();
    let mut filename: String = String::new();

    for increments in 0..dns_vec.len(){
        if dns_vec[increments].id == zone_vec[0].foreign_key{
            filename = dns_vec[increments].domain_name.to_owned() + ".external.zone";
        }
    }

    let write_var_zone_status: bool;
    let move_var_zone_status: bool;
    let restart_named_status: bool;

    let conf: String = gen_var_named_one_zone(zone_vec);
    let result = createfile(&filename, conf.as_bytes());
    match result {
        Ok(()) => write_var_zone_status = true,
        Err(_err) => write_var_zone_status = false,
    }

    let (code, _output, _error) = linux::move_filedir_root(&password, &&filename, "/var/named");
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

fn gen_hostapd_conf(ssid: &str, hide_ssid: bool, hw_mode: &str, channel: &u8, wpa: u8, passphrase: &str, hw_n_mode: bool, qos: bool) -> String {
    format!("interface=wlan0
# SSID to be used in IEEE 802.11 management frames
ssid={}
# Driver interface type (hostap/wired/none/nl80211/bsd)
driver=nl80211
# Country code (ISO/IEC 3166-1)
#country_code=US

# Operation mode (a = IEEE 802.11a (5 GHz), b = IEEE 802.11b (2.4 GHz)
hw_mode={}
# Channel number
channel={}
# Maximum number of stations allowed
#max_num_sta=5

# Bit field: bit0 = WPA, bit1 = WPA2
wpa={}
# Bit field: 1=wpa, 2=wep, 3=both
auth_algs=1

# Set of accepted cipher suites; disabling insecure TKIP
wpa_pairwise=CCMP
# Set of accepted key management algorithms
wpa_key_mgmt=WPA-PSK
wpa_passphrase={}

# hostapd event logger configuration
logger_stdout=-1
logger_stdout_level=2

ignore_broadcast_ssid={}
macaddr_acl=0

# Uncomment and modify the following section if your device supports 802.11n
## Enable 802.11n support
ieee80211n={}
## QoS support
wmm_enabled={}
## Use iw list to show device capabilities and modify ht_capab accordingly
#ht_capab=[HT40+][SHORT-GI-40][TX-STBC][RX-STBC1][DSSS_CCK-40]", 
    ssid, 
    hw_mode, 
    channel, 
    wpa, 
    passphrase, 
    hide_ssid as u8, 
    hw_n_mode as u8, 
    qos as u8
    )
}

fn gen_systemd_networkd_wireless(router_ip: &str, netmask: &str, range_start: &str, range_end: &str, dns: &str, default_lease: &str, max_lease: &str, timezone: &str) -> String{

    let struct_gateway_address = Ipv4Network::with_netmask(
        router_ip.parse().unwrap(),
        netmask.parse().unwrap()
        )
            .unwrap();
    let gateway_address = struct_gateway_address.to_string();
    let network_ip = struct_gateway_address.network().to_string();

    let pool_offset = tool::to_binary(range_start.to_string()) - tool::to_binary(network_ip.to_string());
    let pool_size = tool::to_binary(range_end.to_string()) - tool::to_binary(range_start.to_string());
    let router_dns = router_ip.to_owned() + " " + dns;

    format!("[Match]
Name=wlan0

[Network]
IPMasquerade=both
Address={}
DHCPServer=yes

[DHCPServer]
PoolOffset={}
PoolSize={}
DNS={}
DefaultLeaseTimeSec={}
MaxLeaseTimeSec={}
Timezone={}", gateway_address, pool_offset, pool_size, router_dns, default_lease, max_lease, timezone)
}

fn gen_systemd_networkd_wired_static(internet_ip: &str, netmask: &str, gateway: &str, dns: &str) -> String {
    let internet_address = Ipv4Network::with_netmask(
        internet_ip.parse().unwrap(),
        netmask.parse().unwrap()
        )
            .unwrap()
            .to_string();
    format!(
"
[Match]
Name=eth0

[Network]
DHCP=no
Address={}
Gateway={}
DNS={}
IPv6PrivacyExtensions=yes", internet_address, gateway, dns)
}

fn gen_systemd_networkd_wired_dynamic() -> String {
    r#"
[Match]
Name=eth0

[Network]
DHCP=yes
IPv6PrivacyExtensions=yes

[DHCP]
RouteMetric=1024
"#.to_string()
}

fn gen_named_conf() -> String {
    r#"
include "/etc/named.conf.options";
include "/etc/named.conf.internal.zones";
include "/etc/named.conf.external.zones";
include "/etc/named.conf.acl";
include "/etc/named.conf.logging";"#.to_string()
}

fn gen_named_conf_acl(router_ip: &str, netmask: &str) -> String {

    let gateway_address = Ipv4Network::with_netmask(
        router_ip.parse().unwrap(),
        netmask.parse().unwrap()
        )
            .unwrap()
            .network()
            .to_string();
    format!(r#"acl local-networks {{
    127.0.0.0/8;
    {};
}};"#, gateway_address)
}

fn gen_named_conf_options(dns: &str)-> String {

    let mut splited_dns: Vec<&str> = dns.split_whitespace().collect::<Vec<&str>>();
    splited_dns.reverse();

    let mut joint_dns: String = String::new();

    for each_ip in splited_dns {
        joint_dns = each_ip.to_owned() + "; " +  joint_dns.as_str();
    }

    format!(r#"options {{
    directory "/var/named";
    pid-file "/run/named/named.pid";
    session-keyfile "/run/named/session.key";

    allow-query       {{ local-networks; }};
    allow-recursion   {{ local-networks; }};
    allow-query-cache {{ local-networks; }};
    allow-transfer    {{ local-networks; }};
    allow-update      {{ local-networks; }};

    version none;
    hostname none;
    server-id none;

    auth-nxdomain yes;
    datasize default;
    empty-zones-enable no;
    dnssec-validation yes;

    forwarders {{ {} }};
}};"#, joint_dns)
}

fn gen_named_conf_internal_zones() -> String {
    r#"zone "localhost" IN {
    type master;
    file "localhost.zone";
};

zone "0.0.127.in-addr.arpa" IN {
    type master;
    file "127.0.0.zone";
};

zone "1.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.0.ip6.arpa" {
    type master;
    file "localhost.ip6.zone";
};
zone "koompi.com" IN {
    type master;
    file "koompi.zone";
    allow-update { none; };
    notify no;
};"#.to_string()
}

fn gen_named_conf_logging() -> String {
    r#"logging {{
    channel xfer-log {{
        file "/var/log/named.log";
            print-category yes;
            print-severity yes;
            severity info;
        }};
        category xfer-in {{ xfer-log; }};
        category xfer-out {{ xfer-log; }};
        category notify {{ xfer-log; }};
}};"#.to_string()
}

fn gen_named_conf_external_zones() -> String {

    let zones_vec = db::read_dnszones();

    let mut actual_conf: String = String::new();

    for increments in 0..zones_vec.len(){
        if zones_vec[increments].status != false {
            let current_zone_info = format!(
r#"zone "{}" IN {{
    type master;
    file "{}.external.zone";
    allow-update {{ none; }};
    notify no;
}};
"#, zones_vec[increments].domain_name, zones_vec[increments].domain_name);
            actual_conf = format!("{}\n{}", actual_conf, current_zone_info);
        }
        else {
            let current_zone_info = format!(
r#"# zone "{}" IN {{
#     type master;
#     file "{}.external.zone";
#     allow-update {{ none; }};
#     notify no;
# }};
"#, zones_vec[increments].domain_name, zones_vec[increments].domain_name);
            actual_conf = format!("{}\n{}", actual_conf, current_zone_info);
        }
    }
    actual_conf
}

fn gen_var_named_one_zone(zone_vec: Vec<PartialZoneRecords>) -> String {

    let actual_conf: String;
    if zone_vec.len() != 0{
        let mut ns_declaration: String = String::new();
        let mut subdomain_declaration: String = String::new();
        let dns_vec = db::read_dnszones();
        // println!("{:#?}", zone_vec);

        let index_dns_vec: usize = zone_vec[0].foreign_key.parse::<usize>().unwrap();
        let domain_name: String = dns_vec[index_dns_vec-1].domain_name.to_owned();

        for increments in 0..zone_vec.len(){
            let subdomain_name: String = zone_vec[increments].subdomain_name.to_owned();
            let dns_type: String = zone_vec[increments].dns_type.to_owned();
            let address: String = zone_vec[increments].address.to_owned();
            if zone_vec[increments].dns_type == "A" {
                let current_ns = format!("                IN      NS      {}", zone_vec[increments].subdomain_name);
                ns_declaration = format!("{}\n{}", ns_declaration, current_ns);
                let current_subdomain = format!("{}              IN      {}       {}", subdomain_name, dns_type,address);
                subdomain_declaration = format!("{}\n{}", subdomain_declaration, current_subdomain)
            }
            else {
                let current_subdomain = format!("{}              IN      {}       {}", subdomain_name, dns_type,address);
                subdomain_declaration = format!("{}\n{}", subdomain_declaration, current_subdomain);
            }
        }

        actual_conf = format!(
r#"$TTL 7200
; {}
@       IN      SOA     ns01.{}. admin.{}. (
                                        2018111111 ; Serial
                                        28800      ; Refresh
                                        1800       ; Retry
                                        604800     ; Expire - 1 week
                                        86400 )    ; Negative Cache TTL
{}
{}                                        
"#, domain_name, domain_name, domain_name, ns_declaration, subdomain_declaration);
    }
    else {
        actual_conf = "".to_string();
    }

    actual_conf
}