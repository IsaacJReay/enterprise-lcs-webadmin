use serde::{
    Deserialize, 
    Serialize,
};

#[derive(Serialize)]
pub struct UserName {
    pub username: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub operation_status: String,
    pub token: String,
}

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
pub struct BackupParam {
    pub filename: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RestoreParam {
    pub password: String,
}

#[derive(Deserialize)]
pub struct PasswdParam {
    pub old_password: String,
    pub new_password: String,
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

#[derive(Serialize)]
pub struct HttpResponseCustom {
    pub operation_status: String,
    pub reason: String,
}

#[derive(Serialize, Deserialize)]
pub struct DnsZones {
    pub id: String,
    pub domain_name: String,
    pub status: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ZoneRecords {
    pub id_zonerecords: DnsId,
    pub partial_zonerecords: PartialZoneRecords,    
}

#[derive(Serialize)]
pub struct GetZoneRecords {
    pub domain_name: String,
    pub status: bool,
    pub record_table: Vec<CustomZoneRecords>,
}

#[derive(Serialize)]
pub struct CustomZoneRecords {
    pub id: String,
    pub subdomain_name: String,
    pub dns_type: String,
    pub address: String,
    pub foreign_key: String,
}


#[derive(Serialize, Deserialize, Clone)]
pub struct PartialZoneRecords {
    pub subdomain_name: String,
    pub dns_type: String,
    pub address: String,
    pub foreign_key: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct DnsId {
    pub id: String,
}

#[derive(Serialize, Deserialize)]
pub struct ForeignKey {
    pub foreign_key: String,
}

#[derive(Deserialize)]
pub struct DeleteRecord {
    pub id: String,
    pub foreign_key: String,
}

#[derive(Deserialize)]
pub struct RenameDomain {
    pub foreign_key: ForeignKey,
    pub new_domain_name: String,
}

#[derive(Deserialize)]
pub struct UpdateStatus {
    pub id: String,
    pub status: bool,
}

#[derive(Deserialize)]
pub struct CreateDomainName {
    pub domain_name: String,
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
}


#[derive(Serialize)]
pub struct DriveDescription {
    pub drive_label: String,
    pub drive_partuuid: PartUUID,
    pub free_space: String,
    pub total_space: String,
    pub percentage: f32
}

#[derive(Serialize)]
pub struct ItemList {
    pub item_path: String,
    pub item_date: String,
    pub item_type: String,
    pub item_size: String,
}

#[derive(Serialize)]
pub struct ItemListExtended {
    pub drive_label: String,
    pub item_list: Vec<ItemList>,
}

#[derive(Deserialize)]
pub struct MoveOrCopyArgs {
    pub operation: String,
    pub source: Vec<String>,
    pub destination: String,
}

#[derive(Deserialize)]
pub struct DeleteArgs {
    pub selected_filedir: Vec<String>,
}


#[derive(Debug)]
pub struct Path {
    pub parts: Vec<String>,
}
impl Path {
    pub fn new(path: &str) -> Path {
        Path {
            parts: path.to_string().split("/").map(|s| s.to_string()).collect(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Metadata {
    pub isFile: bool,
    pub isDir: bool,
    pub size: u64,
}

impl Metadata {
    pub fn new(meta: std::fs::Metadata) -> Self {
        let size = meta.clone().len();
        let isFile = meta.clone().is_file();
        let isDir = meta.clone().is_dir();
        Self {
            isFile,
            isDir,
            size,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Dir {
    name: String,
    meta: Option<Metadata>,
    children: Vec<Box<Dir>>,
}

impl Dir {
    pub fn new(name: &str, meta: Option<Metadata>) -> Dir {
        Dir {
            meta,
            name: name.to_string(),
            children: Vec::<Box<Dir>>::new(),
        }
    }

    pub fn find_child(&mut self, name: &str) -> Option<&mut Dir> {
        for c in self.children.iter_mut() {
            if c.name == name {
                return Some(c);
            }
        }
        None
    }

    pub fn add_child<T>(&mut self, leaf: T) -> &mut Self
    where
        T: Into<Dir>,
    {
        self.children.push(Box::new(leaf.into()));
        self
    }
}