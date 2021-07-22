use run_script::{
    ScriptOptions, 
    run_script
};

use crate::{
    db,
    structs::{
        DriveDescription,
        PartUUID,
    }
};

pub fn get_partitions() -> (i32, String, String) {
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


    db::insert_into_storage_table(partition_name, splited_output[0], splited_output[1]);

    (code, splited_output[1].to_string(), error)

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

pub fn passwd(username: &str, old_password: &str, new_password: &str) -> (i32, String, String){

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

pub fn mvfile(password: &str, file: &str, path: &str) -> (i32, String, String){

    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S mv file path"#;
    let _command = _command.replace("password", password);
    let _command = _command.replace("file", file);
    let command = _command.replace("path", path);

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

pub fn set_time(password: &str, current_time: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = 
r#"echo password | sudo -S timedatectl set-ntp 0;
echo password | sudo -S timedatectl set-time tvalue;"#;

    let _command = _command.replace("password", password);
    let _command = _command.replace("password", password);
    let command = _command.replace("tvalue", current_time);

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

pub fn rmfile(password: &str, filepath: &str) -> (i32, String, String){

    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S rm -f filepath"#;
    let _command = _command.replace("password", password);
    let command = _command.replace("filepath", filepath);

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

