pub mod users;
pub mod systemdnetworkd;
pub mod storage;
pub mod hostapd;

pub fn create_tables() {
    let result = std::fs::remove_file("/tmp/lcs.db");
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
CREATE TABLE logindata (variable TXT, value TXT);
CREATE TABLE storagetable(dev_path TXT, part_uuid TXT, mount_location TXT, filesystem_type TXT);
CREATE TABLE tokentable(token TXT);

INSERT INTO logindata VALUES ('username', 'NULL');
INSERT INTO logindata VALUES ('password', 'NULL');
"#,)
            .unwrap();
    }
}
