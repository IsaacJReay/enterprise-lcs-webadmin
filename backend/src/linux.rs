pub mod storage;
pub mod systemsettings;

use run_script::{
    ScriptOptions, 
    run_script
};

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

