pub mod users;
pub mod systemdnetworkd;
pub mod storage;
pub mod named;
pub mod hostapd;

use std::{
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
CREATE TABLE storagetable(dev_path TXT, part_uuid TXT, mount_location TXT, filesystem_type TXT);
CREATE TABLE tokentable(token TXT);

INSERT INTO logindata VALUES ('username', 'NULL');
INSERT INTO logindata VALUES ('password', 'NULL');
"#,)
            .unwrap();
    }
}
