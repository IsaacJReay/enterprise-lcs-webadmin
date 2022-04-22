use serde::{
    Deserialize, 
    Serialize,
};
use chrono::{
    DateTime, 
    Utc,
};
use reqwest::header::HeaderValue;
use toml::value::Table;

#[derive(Deserialize)]
pub struct LoginParam {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct HostapdParam {
    pub ssid: String, 
    pub hide_ssid: bool,
    pub hw_mode: String, 
    pub channel: u8, 
    pub wpa: u8,
    pub passphrase: String,
    pub hw_n_mode: bool,
    pub qos: bool,
}

impl HostapdParam{
    pub fn default() -> Self {
        Self{
            ssid: String::from("Sala"),
            hide_ssid: false,
            hw_mode: String::from("g"),
            channel: 11,
            wpa: 2,
            passphrase: String::from("Koompi-Onelab"),
            hw_n_mode: true,
            qos: true,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct WirelessNetworkParam {
    pub router_ip: String,
    pub netmask: String,
    pub range_start: String,
    pub range_end: String,
    pub dns: String,
    pub default_lease: String,
    pub max_lease: String,
    pub timezone: String,   
}

impl WirelessNetworkParam {
    pub fn default() -> Self {
        Self{
            router_ip: String::from("10.100.100.1"),
            netmask: String::from("255.255.255.0"),
            range_start: String::from("10.100.100.1"),
            range_end: String::from("10.100.100.254"),
            dns: String::from("1.1.1.1 8.8.8.8"),
            default_lease: String::from("1800"),
            max_lease: String::from("7200"),
            timezone: String::from("Asia/Phnom_Penh")
        }
    }
}

#[derive(Deserialize)]
pub struct PasswdParam {
    pub old_password: String,
    pub new_password: String,
}

impl PasswdParam {
    pub fn default(old: &str) -> Self {
        Self {
            old_password: old.to_string(),
            new_password: String::from("123")
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct StaticWiredNetworkParam {
    pub internet_ip: String,
    pub netmask: String,
    pub gateway: String,
    pub dns: String,
}

#[derive(Serialize)]
pub struct WanPageResult {
    pub dhcp: bool,
    pub wired_network_param: StaticWiredNetworkParam,
}

#[derive(Serialize)]
pub struct StatusPageResult {
    pub wan_macaddress: String,
    pub wan_ip: String,
    pub wan_netmask: String,
    pub wan_gateway: String,

    pub wlan_macaddress: String,
    pub wlan_ip: String,
    pub wlan_netmask: String,
    pub wlan_dns: String,

    pub wlan_ssid: String,
    pub wlan_hw_mode: String,
    pub wlan_channel: u8,
    pub wlan_hw_n_mode: bool,
    pub wlan_qos: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DnsZonesInfo {
    pub domain_name: String,
    pub status: bool,
    pub zone_record: Option<Vec<DnsRecords>>
}

impl DnsZonesInfo {
    pub fn default(wan_ip: Option<&str>) -> Self {
        Self {
            domain_name: "koompi.com".to_string(),
            status: true,
            zone_record: Some(vec![
                DnsRecords { subdomain_name: "ns".to_string(), dns_type: "A".to_string(), address: wan_ip.unwrap_or_else(||"10.100.100.1").to_string() },
                DnsRecords { subdomain_name: "sala".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "salabackend".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "rachel".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "admin".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "w3".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "phet".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "admin".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "wiktionary".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "wikipedia".to_string(), dns_type: "A".to_string(), address: "ns".to_string()},
                DnsRecords { subdomain_name: "wikibook".to_string(), dns_type: "A".to_string(), address: "ns".to_string()}
            ])
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DnsRecords {
    pub subdomain_name: String,
    pub dns_type: String,
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct Timezone {
    pub timezone: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimeDateZone {
    pub timezone: Timezone,
    pub timedate: TimeDate,
}


#[derive(Serialize)]
pub struct TimeDateZoneNTP {
    pub ntp_status: bool,
    pub time: String,
    pub date: String,
    pub timezone: String,
}

#[derive(Serialize, Deserialize)]
pub struct TimeDate {
    pub time: String,
    pub date: String,
}

#[derive(Serialize, Deserialize)]
pub struct PartUUID{
    pub drive_partuuid: String,
}
#[derive(Deserialize)]
pub struct MakeDirectoryArgs {
    pub directory_name: String,
    pub parent_directory: String,
    pub drive_partuuid: String
}


#[derive(Serialize)]
pub struct DriveDescription {
    pub drive_label: String,
    pub drive_partuuid: PartUUID,
    pub free_space: String,
    pub total_space: String,
    pub percentage: f32
}

#[derive(Deserialize)]
pub struct MoveOrCopyArgs {
    pub operation: String,
    pub source_uuid: String,
    pub source_items: Vec<String>,
    pub destination_uuid: String,
    pub items_destination: String,
}

#[derive(Deserialize)]
pub struct DeleteArgs {
    pub drive_partuuid: String,
    pub selected_filedir: Vec<String>,
}
pub struct PathPartition {
    pub parts: Vec<String>,
}

impl PathPartition {
    pub fn new(current_path: &str) -> PathPartition {
        PathPartition {
            parts: {
                current_path
                    .to_string()
                    .split("/")
                    .filter(
                        |filter_argument| 
                        !filter_argument.is_empty()
                    )
                        .map(
                            |s|
                            s.to_string()
                    )
                    .collect::<Vec<String>>()
            },
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct ItemMetaData {
    pub item_last_modify_date: String,
    pub item_is_dir: bool,
    pub item_size: u64,
}

impl ItemMetaData {
    pub fn new(meta: std::fs::Metadata) -> Self {
        let size = meta.clone().len();
        let is_dir = meta.clone().is_dir();
        let system_modified_date: DateTime<Utc> = meta.clone().modified().unwrap().into();
        let modified_date = system_modified_date
            .to_rfc3339()
            .split("T")
            .map(
                |argument| {
                    let new_argument = match argument.contains(".") {
                        true => argument
                            .split(".")
                            .map(
                                |new_part| new_part.to_string()
                            )
                            .collect::<Vec<String>>()
                            .first()
                            .unwrap()
                            .to_string(),
                        false => argument.to_string(),
                    };
                    new_argument
                }
            )
            .collect::<Vec<String>>()
            .join(" ");

        Self {
            item_last_modify_date: modified_date,
            item_is_dir: is_dir,
            item_size: size,
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DirectoryInfo {
    pub name: String,
    pub meta: Option<ItemMetaData>,
    pub children: Vec<Box<DirectoryInfo>>,
}

impl DirectoryInfo {
    pub fn new(new_child_name: &str, meta: Option<ItemMetaData>) -> DirectoryInfo {
        DirectoryInfo {
            meta,
            name: new_child_name.to_string(),
            children: Vec::<Box<DirectoryInfo>>::new(),
        }
    }

    pub fn find_child(&mut self, child_name: &str) -> Option<&mut DirectoryInfo> {
        for each_item in self.children.iter_mut() {
            if each_item.name == child_name {
                return Some(each_item);
            }
        }
        None
    }

    pub fn add_child<T>(&mut self, leaf: T) -> &mut Self
    where
        T: Into<DirectoryInfo>,
    {
        self.children.push(Box::new(leaf.into()));
        self
    }
}
pub struct PartialRangeIter {
    pub start: u64,
    pub end: u64,
    pub buffer_size: u32,
}

impl PartialRangeIter {
    pub fn new(start: u64, end: u64, buffer_size: u32) -> reqwest::Result<Self> {
        if buffer_size == 0 {
            panic!("invalid buffer_size, give a value greater than zero.");
        }
        Ok(
            PartialRangeIter {
                start,
                end,
                buffer_size,
            }
        )
    }
}

impl Iterator for PartialRangeIter {
    type Item = HeaderValue;
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.end {
            None
        }
        else {
            let prev_start = self.start;
            self.start += std::cmp::min(self.buffer_size as u64, self.end - self.start + 1);
            Some(HeaderValue::from_str(&format!("bytes={}-{}", prev_start, self.start - 1)).expect("string provided by format!"))
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ContentServerUpdate {
    pub sys_update: Option<Table>,
    pub patch_update: Option<Table>,
}

#[derive(Serialize)]
pub struct SystemUpdateInformation {
    pub id: String,
    pub display_name: String,
    pub update_size: u32,
    pub sys_update: bool,
    pub status: String
}

impl SystemUpdateInformation {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    pub fn get_sys_update(&self) -> bool {
        self.sys_update
    }
    pub fn get_status(&self) -> String {
        self.status.clone()
    }
}

impl PartialEq for SystemUpdateInformation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id &&
        self.display_name == other.display_name &&
        self.update_size == other.update_size &&
        self.sys_update == other.sys_update &&
        self.status == other.status 
    }
}

#[derive(Deserialize)]
pub struct SystemUpdateRequest {
    pub id: String,
    pub sys_update: bool
}

#[derive(Serialize)]
pub struct TempLogin {
    pub operation_status: String,
    pub token: String
}