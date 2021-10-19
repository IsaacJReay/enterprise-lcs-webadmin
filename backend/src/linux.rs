use byte_unit::Byte;
use run_script::{
    ScriptOptions, 
    run_script
};
use crate::{
    db,
    structs::{
        DriveDescription,
        DriveItem,
        ItemNamePath,
        PartUUID,
    }
};

pub fn get_all_partitions() -> (i32, String, String) {
    let options = ScriptOptions::new();

    let command = 
r#"
dash_reached=false;
count_line=0;

while read -r line;
do
    $dash_reached && count_line=$(( $count_line+1 )) && [[ $count_line > 2 ]] && current_disk=$(echo $line | awk -F' ' '{printf $NF}') && alldisks=$(printf "$alldisks /dev/$current_disk"); 
    [[ $line == -------------------------------------------------------------------------- ]] && dash_reached=true;

done <<< $(udisksctl status)

disks_array=($alldisks);
disks_array_length=$(echo "${#disks_array[@]}");

for (( i=0; i<$disks_array_length; i++ ));
do
    udisks_command_temp=$(udisksctl info -b ${disks_array[$(( $disks_array_length-$i-1 ))]} | grep /org/freedesktop/UDisks2/block_devices/ | sed '1d' | sed 's/Partitions://' | xargs);
    temp_array=($udisks_command_temp);
    temp_array_length=$(echo "${#temp_array[@]}");
    for (( j=0; j<$temp_array_length; j++ ));
    do
        partitions_temp=$(echo ${temp_array[$j]} | awk -F'/' '{printf $NF}');
        partitions_list=$(echo $partitions_temp $partitions_list);
    done
    partitions_list=$(echo "| $partitions_list");
done

partitions_list=$(echo $partitions_list | sed 's/^.//');
printf "$partitions_list""#;
    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn mount_ro_partition(password: &str, partition_name: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();
    let _command = 
r#"
part_uuid=$(ls -lha /dev/disk/by-uuid | grep partition_name | awk -F' ' '{printf $9}');
mount_location="/tmp/$part_uuid";
mkdir $mount_location -p;
echo password | sudo -S mount -o ro /dev/partition_name $mount_location;
printf "$part_uuid $mount_location"
"#;
    let _command = _command.replace("password", password);
    let command = _command.replacen("partition_name", partition_name,2);
    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    let splited_output: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();

    let (_code, partition_filesystem_type, _error) = get_partition_filesystem_type(password, &format!("/dev/{}", partition_name));

    db::insert_into_storage_table(partition_name, splited_output[0], splited_output[1], &partition_filesystem_type);

    (code, splited_output[1].to_string(), error)

}

pub fn mount_rw_partition(password: &str, partition_name: &str, uuid: &str) -> (i32, String, String) {

    let options = ScriptOptions::new();
    let full_path_name = format!("/dev/{}", partition_name);
    let filesystem_type = db::query_filesystem_type_by_path_from_storage_table(&partition_name);
    let mut command: String;

    if &filesystem_type == "vfat" || &filesystem_type == "ntfs" || &filesystem_type == "exfat" {
        command = 
r#"
echo password | sudo -S umount partition_name;
sudo mount -o gid=users,fmask=113,dmask=002 partition_name /tmp/uuid;
"#.to_string();
    }
    else {
        command = 
r#"
echo password | sudo -S umount partition_name;
sudo mount partition_name /tmp/uuid;
"#.to_string();
    }

    command = command.replace("password", password);
    command = command.replacen("partition_name", &full_path_name, 2);
    command = command.replace("uuid", uuid);
    let (code, output, error) = run_script!(
        &command,
        &vec![],
        &options
    ).unwrap();

    (code, output, error)
}

pub fn unmount_partition(password: &str, full_path: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S umount full_path username"#;
    let _command = _command.replace("password", password);
    let command = _command.replace("full_path", full_path);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)    
}

pub fn get_partition_filesystem_type(password: &str, full_path: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();
    let _command = r#"echo password | sudo -S blkid full_path | grep -o 'TYPE=......' | head -1 | awk -F'=' '{printf $2}' | sed 's/\"//g'"#;
    let _command = _command.replace("password", password);
    let command = _command.replace("full_path", full_path);
    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn get_partition_information(mount_location: &str) -> DriveDescription {

    let options = ScriptOptions::new();
    let _command = 
r#"
part_information=$(df -h | grep mount_location);
partition_name=$(echo $part_information | awk -F' ' '{printf $1}' | awk -F'/' '{printf $3}');
part_uuid=$(ls -lha /dev/disk/by-uuid | grep $partition_name | awk -F' ' '{printf $9}');
total_size=$(echo $part_information | awk -F' ' '{printf $2}');
free_space=$(echo $part_information | awk -F' ' '{printf $4}');
printf "$part_uuid $total_size $free_space"
"#;
    let command = _command.replace("mount_location", mount_location);
    let (_code, output, _error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    let splited_output = output.split_whitespace().collect::<Vec<&str>>();
    let drive_struct = DriveDescription{
        drive_label: "Removeable Device".to_string(),
        drive_partuuid: PartUUID{
            drive_partuuid: splited_output[0].to_string()
        },
        total_space: splited_output[1].to_string(),
        free_space: splited_output[2].to_string(),
    };
    
    drive_struct

}

pub fn query_file_in_partition(password: &str, path: &str) -> Vec<DriveItem> {
    let options = ScriptOptions::new();
    let _command = r#"echo password | sudo -S stat -c ''%n'|'%F'|'%y'|'%s'|' path/*"#;
    let _command = _command.replace("path", path);
    let command = _command.replace("password", password);
    let (_code, output, _error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    let mut vec_items: Vec<DriveItem> = Vec::new();
    let mut vec_items_length: usize = 0;

    let splited_output = output.trim_end();

    for each_line in splited_output.lines() {

        let split_each_line: Vec<&str> = each_line.split("|").collect::<Vec<&str>>();

        let filename_vec = split_each_line[0].split("/").collect::<Vec<&str>>();
        let filename = filename_vec[filename_vec.len()-1];


        let item_type = match split_each_line[1] {
            "directory" => "directory",
            _ => "file",
        };

        let last_modify_split: Vec<&str> = split_each_line[2].split('.').collect::<Vec<&str>>();
        let last_modify: &str = last_modify_split[0];

        vec_items.insert(vec_items_length, DriveItem {
            item_name: ItemNamePath {
                item_name: filename.to_string(),
                parent_directory: path.to_string(),
            },
            item_date: last_modify.to_string(),
            item_type: item_type.to_string(),
            item_size: Byte::from_bytes(split_each_line[3].parse::<u128>().unwrap()).get_appropriate_unit(false).format(1),
        });
        vec_items_length +=1;
    }

    if vec_items.is_empty() {
        vec_items.insert(0, DriveItem {
            item_name: ItemNamePath {
                item_name: ".".to_string(),
                parent_directory: path.to_string(),
            },
            item_date: "0000-00-00 00:00:00".to_string(),
            item_type: "file".to_string(),
            item_size: "0 Byte".to_string(),
        });
        vec_items
    }
    else {
        vec_items
    }
}

pub fn is_read_writeable(mount: &str) -> bool {
    let options = ScriptOptions::new();
    let _command = r#"grep "[[:space:]]ro[[:space:],]" /proc/mounts | grep actual_mount"#;
    let command = _command.replace("actual_mount", mount);
    let (_code, output, _error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    match output.is_empty() {
        true => true,
        false => false,
    } 

}

pub fn passwd(username: &str, old_password: &str, new_password: &str) -> (i32, String, String) {

    let options = ScriptOptions::new();

    let _command = r#"echo -e "old_password\nnew_password\nnew_password" | passwd username"#;
    let _command = _command.replace("old_password", old_password);
    let _command = _command.replace("new_password", new_password);
    let _command = _command.replace("new_password", new_password);
    let command = _command.replace("username", username);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn copy_filedir(password: &str, source: &str, destination: &str, source_is_external: bool, source_uuid: &str, destination_is_external: bool, destination_uuid: &str) -> (i32, String, String) {

    if source_is_external || destination_is_external {

        let mut source_user_rw_able: bool = false; 
        let mut destination_user_rw_able: bool = false;

        if source_is_external {
            let path = db::query_path_by_uuid_from_storage_table(source_uuid);
            let (_code, filesystem_type, _error) = get_partition_filesystem_type(password, &format!("/dev/{}",  path));
            if &filesystem_type == "vfat" || &filesystem_type == "ntfs" || &filesystem_type == "exfat" {
                source_user_rw_able = true;
            }
            else {
                source_user_rw_able = false;
            }
        }
    
        if destination_is_external {
            let path = db::query_path_by_uuid_from_storage_table(destination_uuid);
            let (_code, filesystem_type, _error) = get_partition_filesystem_type(password, &format!("/dev/{}",  path));
            if &filesystem_type == "vfat" || &filesystem_type == "ntfs" || &filesystem_type == "exfat" {
                destination_user_rw_able = true;
            }
            else {
                destination_user_rw_able = false;
            }
        }
    
        if source_user_rw_able && destination_user_rw_able {
            let options = ScriptOptions::new();
            let command = r#"cp -r source destination"#;
            let _command = command.replace("source", source);
            let command = _command.replace("destination", destination);

            let (code, output, error) = run_script!(
                &format!("{}", command),
                &vec![],
                &options
            ).unwrap();
            
            (code, output, error)
        }
        else {
            copy_filedir_root(password, source, destination)
        }
    }
    else {
        copy_filedir_root(password, source, destination)
    }
}

pub fn copy_filedir_root(password: &str, source: &str, destination: &str) -> (i32, String, String){

    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S cp -r source destination"#;
    let _command = _command.replace("password", password);
    let _command = _command.replace("source", source);
    let command = _command.replace("destination", destination);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn move_filedir(password: &str, source: &str, destination: &str, source_is_external: bool, source_uuid: &str,  destination_is_external: bool, destination_uuid: &str) -> (i32, String, String) {

    if source_is_external || destination_is_external {

        let mut source_user_rw_able: bool = false; 
        let mut destination_user_rw_able: bool = false;

        if source_is_external {
            let path = db::query_path_by_uuid_from_storage_table(source_uuid);
            let (_code, filesystem_type, _error) = get_partition_filesystem_type(password, &format!("/dev/{}",  path));
            if &filesystem_type == "vfat" || &filesystem_type == "ntfs" || &filesystem_type == "exfat" {
                source_user_rw_able = true;
            }
            else {
                source_user_rw_able = false;
            }
        }
    
        if destination_is_external {
            let path = db::query_path_by_uuid_from_storage_table(destination_uuid);
            let (_code, filesystem_type, _error) = get_partition_filesystem_type(password, &format!("/dev/{}",  path));
            if &filesystem_type == "vfat" || &filesystem_type == "ntfs" || &filesystem_type == "exfat" {
                destination_user_rw_able = true;
            }
            else {
                destination_user_rw_able = false;
            }
        }
    
        if source_user_rw_able && destination_user_rw_able {
            let options = ScriptOptions::new();
            let command = r#"mv source destination"#;
            let _command = command.replace("source", source);
            let command = _command.replace("destination", destination);

            let (code, output, error) = run_script!(
                &format!("{}", command),
                &vec![],
                &options
            ).unwrap();
            
            (code, output, error)
        }
        else {
            move_filedir_root(password, source, destination)
        }
    }
    else {
        move_filedir_root(password, source, destination)
    }
}

pub fn move_filedir_root(password: &str, source: &str, destination: &str) -> (i32, String, String){

    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S mv source destination"#;
    let _command = _command.replace("password", password);
    let _command = _command.replace("source", source);
    let command = _command.replace("destination", destination);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn make_dir(password: &str, dir_location: &str, drive_is_external: bool, drive_uuid: &str) -> (i32, String, String) {
    if drive_is_external {
        // let user_rw_able: bool;
        let path = db::query_path_by_uuid_from_storage_table(drive_uuid);
        let (_code, filesystem_type, _error) = get_partition_filesystem_type(password, &format!("/dev/{}",  path));
        if &filesystem_type == "vfat" || &filesystem_type == "ntfs" || &filesystem_type == "exfat" {
            let options = ScriptOptions::new();
            let command = r#"mkdir dir_location"#;
            let command = command.replace("dir_location", dir_location);
            // let command = _command.replace("destination", destination);

            let (code, output, error) = run_script!(
                &format!("{}", command),
                &vec![],
                &options
            ).unwrap();
            
            (code, output, error)
        }
        else {
            make_dir_root(password, dir_location)
        }
    }
    else {
        make_dir_root(password, dir_location)
    }
}

pub fn make_dir_root(password: &str, dir_location: &str) -> (i32, String, String){

    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S mkdir dir_location"#;
    let _command = _command.replace("password", password);
    let command = _command.replace("dir_location", dir_location);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn remove_filedir_root(password: &str, filepath: &str) -> (i32, String, String){

    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S rm -rf filepath"#;
    let _command = _command.replace("password", password);
    let command = _command.replace("filepath", filepath);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn restartservice(password: &str, servicename: &str) -> (i32, String, String){

    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S systemctl restart servicename"#;
    let _command = _command.replace("servicename", servicename);
    let command = _command.replace("password", password);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn set_time(password: &str, current_time: &str, current_date: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = 
r#"echo password | sudo -S timedatectl set-ntp 0;
echo password | sudo -S date --set='tvalue';
echo password | sudo -S date --set='dvalue';"#;

    let _command = _command.replacen("password", password, 3);
    let _command = _command.replace("tvalue", current_time);
    let command = _command.replace("dvalue", current_date);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)

}

pub fn set_timezone(password: &str, timezone: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = 
r#"echo password | sudo -S timedatectl set-ntp 1;
echo password | sudo -S timedatectl set-timezone tvalue;"#;

    let _command = _command.replace("password", password);
    let _command = _command.replace("password", password);
    let command = _command.replace("tvalue", timezone);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)
}

pub fn query_date_for_display() -> (i32, String, String) {
    let options = run_script::ScriptOptions::new();

    let command = r#"date +"%Y-%M-%d %H:%m:%S""#;

    let (code, mut output, error) = run_script::run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    output.truncate(output.len() - 1 );

    (code, output, error)
}

pub fn query_timezone() -> (i32, String, String) {
    let options = run_script::ScriptOptions::new();

    let command = r#"timedatectl | grep "Time zone" | awk -F' ' '{printf $3}'"#;

    let (code, mut output, error) = run_script::run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    output.truncate(output.len() - 1 );

    (code, output, error)
}

pub fn query_ntp_status() -> (i32, String, String) {
    let options = run_script::ScriptOptions::new();

    let command = r#"timedatectl | grep "NTP service" | awk -F': ' '{printf $2}'"#;

    let (code, output, error) = run_script::run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();    

    (code, output, error)
}

pub fn query_date_for_calculate() -> (i32, String, String){

    let options = ScriptOptions::new();

    let command = r#"date +"%y%m%d%H%M%S""#;

    let (code, mut output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    output.truncate(output.len() - 1 );

    (code, output, error)

}

pub fn tar_config(filename: &str) -> (i32, String, String, String){

    let options = ScriptOptions::new();

    let _command = 
r#"
mkdir -p /tmp/lcs-export/hostapd
mkdir -p /tmp/lcs-export/systemd/network/
cp /etc/named.conf*  /tmp/lcs-export/
cp /etc/hostapd/hostapd.conf /tmp/lcs-export/hostapd
cp /etc/systemd/network/*/tmp/lcs-export/systemd/network/
tar --zstd -cpf /tmp/filename.tar.zst -C /tmp lcs-export/
"#;
    let command = _command.replace("filename", filename);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    let filepath: String = format!("/tmp/{}.tar.zst", filename);
    (code, output, error, filepath)
}

pub fn untar_config(filename: &str) -> (i32, String, String, String) {
    let options = ScriptOptions::new();

    let _command = 
r#"
mkdir -p /tmp/lcs-import
tar --zstd -xpf filename -C /tmp/lcs-import/
mv /tmp/lcs-import/lcs-export/* /tmp/lcs-import
rm -r /tmp/lcs-import/lcs-export
"#;
    let command = _command.replace("filename", filename);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    // let filepath: String = format!("/tmp/{}.tar.zst", filename);
    (code, output, error, "/tmp/lcs-import".to_string())
}

