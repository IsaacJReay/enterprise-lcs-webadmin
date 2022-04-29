use crate::{structs::DnsRecords, tool};
use ipnetwork::Ipv4Network;

pub fn generate_zone_config(domain_name: &str, status: bool, zone_is_internal: bool) -> String {
    let location = match zone_is_internal {
        true => "internal",
        false => "external",
    };
    match status {
        true => format!(
            "zone \"{}\" IN {{\n    type master;\n    file \"{}.{}.zone\";\n    allow-update {{ none; }};\n    notify no;\n}};\n", 
            domain_name, domain_name, location
        ),
        false => format!(
            "# zone \"{}\" IN {{\n#     type master;\n#     file \"{}.{}.zone\";\n#     allow-update {{ none; }};\n#     notify no;\n# }};\n", 
            domain_name, domain_name, location
        )
    }
}

pub fn generate_records_for_zone(domain_name: &str, vec_record: Option<Vec<DnsRecords>>) -> String {
    let mut records_str: String = String::new();
    let date = chrono::Utc::now()
        .format("%Y%m%d%H%M")
        .to_string()
        .parse::<u64>()
        .unwrap();

    if let Some(vec_record) = vec_record {
        vec_record.iter().for_each(|each_record| {
            records_str.push_str(
                format!(
                    "{}              IN      {}       {}\n",
                    each_record.subdomain_name, each_record.dns_type, each_record.address
                )
                .as_ref(),
            );
            if each_record.dns_type == "A" {
                records_str.insert_str(
                    0,
                    format!(
                        "                IN      NS      {}\n",
                        each_record.subdomain_name
                    )
                    .as_ref(),
                );
            }
        });
    }

    records_str.insert_str(
        0,
        format!(
            "$TTL 7200\n; {}\n@       IN      SOA     ns.{}. admin.{}. (
                            {} ; Serial
                            28800      ; Refresh
                            1800       ; Retry
                            604800     ; Expire - 1 week
                            86400 )    ; Negative Cache TTL\n",
            domain_name, domain_name, domain_name, date
        )
        .as_ref(),
    );
    records_str
}

pub fn gen_hostapd_conf(
    ssid: &str,
    hide_ssid: bool,
    hw_mode: &str,
    channel: &u8,
    wpa: u8,
    passphrase: &str,
    hw_n_mode: bool,
    qos: bool,
) -> String {
    format!(
        "interface=wlan0
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
        ssid, hw_mode, channel, wpa, passphrase, hide_ssid as u8, hw_n_mode as u8, qos as u8
    )
}

pub fn gen_systemd_networkd_wireless(
    router_ip: &str,
    netmask: &str,
    range_start: &str,
    range_end: &str,
    dns: &str,
    default_lease: &str,
    max_lease: &str,
    timezone: &str,
) -> String {
    let struct_gateway_address =
        Ipv4Network::with_netmask(router_ip.parse().unwrap(), netmask.parse().unwrap()).unwrap();
    let gateway_address = struct_gateway_address.to_string();
    let network_ip = struct_gateway_address.network().to_string();

    let pool_offset =
        tool::to_binary(range_start.to_string()) - tool::to_binary(network_ip.to_string());
    let pool_size =
        tool::to_binary(range_end.to_string()) - tool::to_binary(range_start.to_string());
    let router_dns = router_ip.to_owned() + " " + dns;

    format!(
        "[Match]
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
Timezone={}",
        gateway_address, pool_offset, pool_size, router_dns, default_lease, max_lease, timezone
    )
}

pub fn gen_systemd_networkd_wired_static(
    internet_ip: &str,
    netmask: &str,
    gateway: &str,
    dns: &str,
) -> String {
    let internet_address =
        Ipv4Network::with_netmask(internet_ip.parse().unwrap(), netmask.parse().unwrap())
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
IPv6PrivacyExtensions=yes",
        internet_address, gateway, dns
    )
}

pub fn gen_systemd_networkd_wired_dynamic() -> String {
    r#"
[Match]
Name=eth0

[Network]
DHCP=yes
IPv6PrivacyExtensions=yes

[DHCP]
RouteMetric=1024
"#
    .to_string()
}

pub fn gen_named_conf_acl(router_ip: &str, netmask: &str) -> String {
    let network_address =
        Ipv4Network::with_netmask(router_ip.parse().unwrap(), netmask.parse().unwrap())
            .unwrap()
            .network();
    let full_network_address = Ipv4Network::with_netmask(network_address, netmask.parse().unwrap())
        .unwrap()
        .to_string();
    format!(
        r#"acl local-networks {{\n127.0.0.0/8;\n{};\n}};"#,
        full_network_address
    )
}

pub fn gen_named_conf_options(dns: &str) -> String {
    let mut splited_dns: Vec<&str> = dns.split_whitespace().collect::<Vec<&str>>();
    splited_dns.reverse();

    let mut joint_dns: String = String::new();

    for each_ip in splited_dns {
        joint_dns = each_ip.to_owned() + "; " + joint_dns.as_str();
    }

    format!(
        r#"options {{
    directory "/var/named";
    pid-file "/var/named/run/named.pid";
    session-keyfile "/var/named/run/session.key";

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
}};"#,
        joint_dns
    )
}
