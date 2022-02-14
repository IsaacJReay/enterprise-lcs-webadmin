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

pub fn mount_partition(password: &str, partition_name: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();
    let _command = 
r#"
echo password | sudo -S umount /dev/partition_name;
part_uuid=$(ls -lha /dev/disk/by-uuid | grep partition_name | awk -F' ' '{printf $9}');
mount_location="/tmp/$part_uuid";
mkdir $mount_location -p;
echo password | sudo -S mount -o gid=users,fmask=113,dmask=002 /dev/partition_name $mount_location;
printf "$part_uuid $mount_location"
"#;
    let _command = _command.replacen("password", password, 2);
    let command = _command.replacen("partition_name", partition_name,3);
    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    let splited_output: Vec<&str> = output.split_whitespace().collect::<Vec<&str>>();

    let (_code, partition_filesystem_type, _error) = get_partition_filesystem_type(partition_name);

    db::insert_into_storage_table(partition_name, splited_output[0], splited_output[1], &partition_filesystem_type);

    (code, splited_output[1].to_string(), error)

}

pub fn unmount_partition(password: &str, full_path: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S umount full_path"#;
    let _command = _command.replace("password", password);
    let command = _command.replace("full_path", full_path);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)    
}

pub fn get_partition_filesystem_type(dev_path: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();
    let _command = r#"lsblk -f | grep drive_path | awk -F' ' '{printf $2}'"#;
    // let _command = _command.replace("password", password);
    let command = _command.replace("drive_path", dev_path);
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
percentage=$(df -h | grep mount_location | awk -F' ' '{printf $5}' | awk -F'%' '{printf $1}')
printf "$part_uuid $total_size $free_space $percentage"
"#;
    let command = _command.replacen("mount_location", mount_location, 2);
    let (_code, output, _error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    // println!("Command: {}\nOutput: {}", command, output);

    let splited_output = output.split_whitespace().collect::<Vec<&str>>();
    let drive_struct = DriveDescription{
        drive_label: "Removeable Device".to_string(),
        drive_partuuid: PartUUID{
            drive_partuuid: splited_output[0].to_string()
        },
        total_space: splited_output[1].to_string(),
        free_space: splited_output[2].to_string(),
        percentage: splited_output[3].parse::<f32>().unwrap()
    };
    
    drive_struct

}

pub fn make_dir(dir_location: &str) -> (i32, String, String) {

    let options = ScriptOptions::new();
    let command = r#"mkdir dir_location"#;
    let command = command.replace("dir_location", dir_location);

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

pub fn copy_or_move(copy: bool, source: &str, destination: &str) -> (i32, String, String){
    let options = ScriptOptions::new();

    let _command = r#"operation source destination"#;
    let _command = _command.replace(
        "operation", 
    match copy {
        true => "cp -ax",
        false => "mv"
        }
    );
    let _command = _command.replace("source", source);
    let command = _command.replace("destination", destination);

    let (code, output, error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    (code, output, error)
}