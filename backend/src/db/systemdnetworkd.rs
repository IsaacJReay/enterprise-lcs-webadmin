use ipnetwork::Ipv4Network;
use crate::{ 
    tool,
    config,
};

use run_script::{
    ScriptOptions, 
    run_script
};

pub fn read_resolvconf(interface_name: &str) -> String {

    let options = ScriptOptions::new();

    let _command = r#"resolvectl dns | grep INTERFACE_NAME | awk -F': ' '{printf $2}'"#;
    let command = _command.replace("INTERFACE_NAME", interface_name);

    let (_code, output, _error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    output
}

pub fn read_wlan_networkd() -> (String, String, String, String, String, String, String, String) {

    let mut router_ip = String::new();
    let mut prefix_length: u8 = 0;
    let mut pool_offset: String = String::new();
    let mut pool_size: String = String::new();
    let mut dns: String = String::new();
    let mut default_lease: String = String::new();
    let mut max_lease: String = String::new();
    let mut timezone: String = String::new();

    let config = config::read_file("/etc/systemd/network/20-wireless.network");

    config
        .lines()
        .for_each(|each_line| {
            if each_line.contains("Address="){
                router_ip = each_line.to_owned().split(&['=', '/'][..]).nth(1).unwrap().to_string();
                prefix_length = each_line.to_owned().split(&['=', '/'][..]).nth(2).unwrap().parse::<u8>().unwrap();
            }
            else if each_line.contains("PoolOffset="){
                pool_offset = each_line.split("=").last().unwrap().to_string();
            }
            else if each_line.contains("PoolSize="){
                pool_size = each_line.split("=").last().unwrap().to_string();
            }
            else if each_line.contains("DNS="){
                dns = each_line.split("=").last().unwrap().to_string();
            }
            else if each_line.contains("DefaultLeaseTimeSec="){
                default_lease = each_line.split("=").last().unwrap().to_string();
            }
            else if each_line.contains("MaxLeaseTimeSec="){
                max_lease = each_line.split("=").last().unwrap().to_string();
            }
            else if each_line.contains("Timezone="){
                timezone = each_line.split("=").last().unwrap().to_string();
            }
        }
    );

    let struct_ipv4network = Ipv4Network::new(
        router_ip.parse().unwrap(), 
        prefix_length
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

    let config = config::read_file("/etc/systemd/network/20-wired.network");
    let dhcp_status = match config
        .lines()
        .find(|each_line| each_line.contains("DHCP="))
        .unwrap()
        .split("=")
        .last()
        .unwrap() {
            "yes" => true,
            _ => false
        };
    

    let (_macaddr_output, internet_ip, netmask, gateway) = read_eth0();
    let dns = read_resolvconf("enp1s0");

    (dhcp_status, internet_ip, netmask, gateway, dns)
}

pub fn read_eth0() -> (String, String, String, String){
    let options = run_script::ScriptOptions::new();
    let eth0_macaddress = r#"ip address show enp1s0 | grep link/ether | awk -F' ' '{printf $2}'"#;   
    let eth0_ipaddress = r#"ip address | grep enp1s0 |grep inet | awk -F' ' '{printf $2}' |awk -F '/' '{printf $1}'"#;   
    let eth0_prefixaddr = r#"ip address | grep enp1s0 |grep inet | awk -F' ' '{printf $2}'"#;
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