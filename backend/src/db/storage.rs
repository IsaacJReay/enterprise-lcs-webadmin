use rusqlite::{params, Connection};
use crate::DATABASE;

pub fn insert_into_storage_table(
    udevpath: &str,
    partuuid: &str,
    mountlocation: &str,
    fsystype: &str,
) {
    Connection::open(DATABASE)
        .unwrap()
        .execute(
            "INSERT INTO tblStorage VALUES (?, ?, ?, ?)",
            params![udevpath, partuuid, mountlocation, fsystype],
        )
        .unwrap();
}

pub fn delete_from_storage_table(partuuid: &str) {
    Connection::open(DATABASE)
        .unwrap()
        .execute("DELETE FROM tblStorage WHERE PartUUID=?", &[partuuid])
        .unwrap();
}

pub fn query_existence_from_storage_table_by_path(udevpath: &str) -> bool {
    let connection = Connection::open(DATABASE).unwrap();

    let mut stmt = connection
        .prepare("SELECT EXISTS(SELECT UdevPath FROM tblStorage WHERE UdevPath=? LIMIT 1);")
        .unwrap();
    let mut rows = stmt.query(&[udevpath]).unwrap();

    rows.next().unwrap().unwrap().get::<usize, u64>(0).unwrap() != 0
}

pub fn query_from_storage_table(
    udevpath: Option<&str>,
    partuuid: Option<&str>,
) -> (String, String) {
    let connection = Connection::open(DATABASE).unwrap();

    let mut stmt = connection
        .prepare(match udevpath {
            Some(_udevpath) => "SELECT UdevPath,MountLocation FROM tblStorage WHERE UdevPath=?",
            None => "SELECT UdevPath,MountLocation FROM tblStorage WHERE PartUUID=?",
        })
        .unwrap();
    let mut rows = stmt
        .query(params!(match udevpath {
            Some(udevpath) => udevpath,
            None => partuuid.unwrap(),
        }))
        .unwrap();

    match rows.next().unwrap() {
        Some(each_row) => (
            each_row.get("UdevPath").unwrap(),
            each_row.get("MountLocation").unwrap(),
        ),
        None => (String::new(), String::new()),
    }
}
