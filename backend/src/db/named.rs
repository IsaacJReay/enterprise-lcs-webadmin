use sqlite::State;
use wildmatch::WildMatch;
use crate::{ 
    db::read_lines,
    structs::{
        DnsZones,
        ZoneRecords,
        CustomZoneRecords,
        PartialZoneRecords,
        DnsId,
    }, 
};
use std::convert::TryInto;

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

pub fn query_status_by_foreign_key(foreign_key: &str) -> bool {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut current_status: bool = false;
    let mut check_empty_statement = connection
        .prepare("SELECT COUNT(*) FROM dnszones;")
        .unwrap();
    
    check_empty_statement.next().unwrap();
    let line_count: u64 = check_empty_statement.read::<i64>(0).unwrap().try_into().unwrap();

    if line_count != 0 {
        let mut read_statement = connection
            .prepare(
                format!("SELECT status FROM dnszones WHERE id = '{}';", foreign_key)
            )
            .unwrap();
        
        while let State::Row = read_statement.next().unwrap() {
            current_status = read_statement.read::<String>(0).unwrap().parse().unwrap();
        }
    }
    current_status 
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

pub fn read_zonerecords_for_get_by_foreign_key(foreign_key: &str) -> Vec<CustomZoneRecords> {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut records_vec: Vec<CustomZoneRecords> = Vec::new();
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
            CustomZoneRecords{
                id: current_id,
                subdomain_name: current_subdomain_name,
                dns_type: current_type,
                address: current_address,
                foreign_key: current_foriegn_key
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
                r#"INSERT INTO dnszones VALUES ('{}', '{}', 'false');"#,
                latest_id, domain_name),
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
                r#"INSERT INTO zonerecords VALUES ('{}', '{}', '{}', '{}', '{}');"#,
                latest_id, subdomain_name, dns_type, address, foreign_key),
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
                r#"DELETE FROM zonerecords WHERE id='{}' AND foreign_key='{}'"#,
                id, foreign_key)
            )
            .unwrap();

    if row_affected > 0 {
        for new_id in u64_id..latest_id {
            connection
            .execute(format!(
                r#"UPDATE zonerecords SET id = '{}' WHERE id = '{}' AND foreign_key='{}';"#, 
                new_id, new_id+1, foreign_key),
            )
            .unwrap();
        }
    }
}

pub fn update_domain_name_by_foreign_key(foreign_key: &str, domain_name: &str) {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    connection
        .execute(format!(
                r#"UPDATE dnszones SET domain_name = '{}' WHERE id = '{}'"#, 
                domain_name, foreign_key
            )
        )
        .unwrap();
}

pub fn query_domain_name_by_foreign_key(foreign_key: &str) -> String {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();
    let mut domain_name: String = String::new();
    let mut statement = connection
        .prepare(
            format!("SELECT domain_name FROM dnszones WHERE id = '{}'", foreign_key)
        )
        .unwrap();

    while let State::Row = statement.next().unwrap() {
        domain_name = statement.read::<String>(0).unwrap();
    }
    
    domain_name

}