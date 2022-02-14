use std::convert::TryInto;

pub fn insert_into_storage_table(path: &str, uuid: &str, mount: &str, filesystem_type: &str) {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    connection
        .execute(
            format!("INSERT INTO storagetable VALUES ('{}', '{}', '{}', '{}');", path, uuid, mount, filesystem_type)
        )
            .unwrap();
}

pub fn delete_from_storage_table(uuid: &str) {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    connection
        .execute(
            format!("DELETE FROM storagetable WHERE part_uuid='{}';", uuid)
        )
            .unwrap();

}

pub fn query_existence_from_storage_table_by_path(path: &str) -> bool {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let mut check_empty_statement = connection
        .prepare(
            format!("SELECT EXISTS(SELECT dev_path FROM storagetable WHERE dev_path='{}' LIMIT 1);", path)
        )
            .unwrap();

    check_empty_statement.next().unwrap();
    let output: u64 = check_empty_statement.read::<i64>(0).unwrap().try_into().unwrap();

    output!=0
    
}

pub fn query_mount_by_path_from_storage_table(path: &str) -> String {
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let mut read_path_mount = connection
        .prepare(
            format!("SELECT mount_location FROM storagetable WHERE dev_path='{}';", path)
        )
            .unwrap();

    read_path_mount.next().unwrap();
    let path: String = read_path_mount.read::<String>(0).unwrap();

    path
}

pub fn query_mount_by_uuid_from_storage_table(uuid: &str) -> String {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let mut read_path_mount = connection
        .prepare(
            format!("SELECT mount_location FROM storagetable WHERE part_uuid='{}';", uuid)
        )
            .unwrap();

    read_path_mount.next().unwrap();
    let mount: String = read_path_mount.read::<String>(0).unwrap();

    mount
}

pub fn query_path_by_uuid_from_storage_table(uuid: &str) -> String {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let mut read_path_mount = connection
        .prepare(
            format!("SELECT dev_path FROM storagetable WHERE part_uuid='{}';", uuid)
        )
            .unwrap();

    read_path_mount.next().unwrap();
    let mount: String = read_path_mount.read::<String>(0).unwrap();

    mount
}