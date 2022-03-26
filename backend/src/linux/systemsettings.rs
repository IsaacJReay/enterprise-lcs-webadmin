use run_script::{
    ScriptOptions, 
    run_script
};

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

// pub fn tar_config(filename: &str) -> (i32, String, String, String){

//     let options = ScriptOptions::new();

//     let _command = 
// r#"
// mkdir -p /tmp/lcs-export/hostapd
// mkdir -p /tmp/lcs-export/systemd/network/
// cp /etc/named.conf*  /tmp/lcs-export/
// cp /etc/hostapd/hostapd.conf /tmp/lcs-export/hostapd
// cp /etc/systemd/network/*/tmp/lcs-export/systemd/network/
// tar --zstd -cpf /tmp/filename.tar.zst -C /tmp lcs-export/
// "#;
//     let command = _command.replace("filename", filename);

//     let (code, output, error) = run_script!(
//         &format!("{}", command),
//         &vec![],
//         &options
//     ).unwrap();

//     let filepath: String = format!("/tmp/{}.tar.zst", filename);
//     (code, output, error, filepath)
// }

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

    
    (code, output, error, "/tmp/lcs-import".to_string())
}
