use wildmatch::WildMatch;
use ipnetwork::Ipv4Network;
use crate::{ 
    tool,
    db::read_lines,
};

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