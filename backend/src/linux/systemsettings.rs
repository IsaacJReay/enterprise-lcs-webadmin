use run_script::{run_script, ScriptOptions};

pub fn set_time(password: &str, current_time: &str, current_date: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S timedatectl set-ntp 0;
echo password | sudo -S date --set='tvalue';
echo password | sudo -S date --set='dvalue';"#;

    let _command = _command.replacen("password", password, 3);
    let _command = _command.replace("tvalue", current_time);
    let command = _command.replace("dvalue", current_date);

    let (code, output, error) = run_script!(&format!("{}", command), &vec![], &options).unwrap();

    (code, output, error)
}

pub fn set_timezone(password: &str, timezone: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S timedatectl set-ntp 1;
echo password | sudo -S timedatectl set-timezone tvalue;"#;

    let _command = _command.replace("password", password);
    let _command = _command.replace("password", password);
    let command = _command.replace("tvalue", timezone);

    let (code, output, error) = run_script!(&format!("{}", command), &vec![], &options).unwrap();

    (code, output, error)
}

pub fn query_date_for_display() -> (i32, String, String) {
    let options = run_script::ScriptOptions::new();

    let command = r#"date +"%Y-%M-%d %H:%m:%S""#;

    let (code, mut output, error) =
        run_script::run_script!(&format!("{}", command), &vec![], &options).unwrap();

    output.truncate(output.len() - 1);

    (code, output, error)
}

pub fn query_timezone() -> (i32, String, String) {
    let options = run_script::ScriptOptions::new();

    let command = r#"timedatectl | grep "Time zone" | awk -F' ' '{printf $3}'"#;

    let (code, mut output, error) =
        run_script::run_script!(&format!("{}", command), &vec![], &options).unwrap();

    output.truncate(output.len() - 1);

    (code, output, error)
}

pub fn query_ntp_status() -> (i32, String, String) {
    let options = run_script::ScriptOptions::new();

    let command = r#"timedatectl | grep "NTP service" | awk -F': ' '{printf $2}'"#;

    let (code, output, error) =
        run_script::run_script!(&format!("{}", command), &vec![], &options).unwrap();

    (code, output, error)
}
