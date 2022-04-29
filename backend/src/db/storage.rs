use rusqlite::{params, Connection};

pub fn insert_into_storage_table(
    udevpath: &str,
    partuuid: &str,
    mountlocation: &str,
    fsystype: &str,
) {
    Connection::open("/tmp/lcs.db")
        .unwrap()
        .execute(
            "INSERT INTO tblStorage VALUES (?, ?, ?, ?)",
            params![udevpath, partuuid, mountlocation, fsystype],
        )
        .unwrap();
}

pub fn delete_from_storage_table(partuuid: &str) {
    Connection::open("/tmp/lcs.db")
        .unwrap()
        .execute("DELETE FROM tblStorage WHERE PartUUID=?", &[partuuid])
        .unwrap();
}

pub fn query_existence_from_storage_table_by_path(udevpath: &str) -> bool {
    let connection = Connection::open("/tmp/lcs.db").unwrap();

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
    let connection = Connection::open("/tmp/lcs.db").unwrap();

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

// pub fn query_mount_by_path_from_storage_table(path: &str) -> String {
//     let connection = sqlite::open("/tmp/lcs.db").unwrap();

//     let mut read_path_mount = connection
//         .prepare(
//             format!("SELECT mount_location FROM storagetable WHERE dev_path='{}';", path)
//         )
//             .unwrap();

//     read_path_mount.next().unwrap();
//     let path: String = read_path_mount.read::<String>(0).unwrap();

//     path
// }

// pub fn query_mount_by_uuid_from_storage_table(uuid: &str) -> String {

//     let connection = sqlite::open("/tmp/lcs.db").unwrap();

//     let mut read_path_mount = connection
//         .prepare(
//             format!("SELECT mount_location FROM storagetable WHERE part_uuid='{}';", uuid)
//         )
//             .unwrap();

//     read_path_mount.next().unwrap();
//     let mount: String = read_path_mount.read::<String>(0).unwrap();

//     mount
// }

// pub fn query_path_by_uuid_from_storage_table(uuid: &str) -> String {

//     let connection = sqlite::open("/tmp/lcs.db").unwrap();

//     let mut read_path_mount = connection
//         .prepare(
//             format!("SELECT dev_path FROM storagetable WHERE part_uuid='{}';", uuid)
//         )
//             .unwrap();

//     read_path_mount.next().unwrap();
//     let mount: String = read_path_mount.read::<String>(0).unwrap();

//     mount
// }
