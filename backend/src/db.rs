pub mod users;
pub mod systemdnetworkd;
pub mod storage;
pub mod hostapd;

pub fn create_tables() {

    if let Err(err) = std::fs::remove_file("/tmp/lcs.db") {
        if &err.to_string() == "Operation not permitted (os error 1)"{
            eprintln!("{:#?}", &err);
        }
    }

    rusqlite::Connection::open("/tmp/lcs.db")
        .unwrap()
        .execute_batch(
            "BEGIN;
            CREATE TABLE tblStorage(UdevPath CHARACTER, PartUUID VARCHAR, MountLocation VARCHAR, FSysType CHARACTER);
            CREATE TABLE tblAuth(UserName TXT, CryptedPass TXT, SessionID TXT, IAT UNSIGNED BIG INT);
            COMMIT;"
        )
        .unwrap();
          
}