pub mod storage;
pub mod systemsettings;
pub mod update;

use run_script::{run_script, ScriptOptions};

pub fn chown_chmod(
    password: &str,
    owner: &str,
    group: &str,
    permission_bit: &str,
    item_name: &str,
) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S chown OWNER:GROUP item_name; echo password | sudo -S chmod permission_bit item_name"#;
    let _command = _command.replace("OWNER", owner);
    let _command = _command.replace("GROUP", group);
    let _command = _command.replace("permission_bit", permission_bit);
    let _command = _command.replacen("item_name", item_name, 2);
    let command = _command.replacen("password", password, 2);

    let (code, output, error) = run_script!(&format!("{}", command), &vec![], &options).unwrap();

    (code, output, error)
}

pub fn passwd(username: &str, old_password: &str, new_password: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = r#"echo -e "old_password\nnew_password\nnew_password" | passwd username"#;
    let _command = _command.replace("old_password", old_password);
    let _command = _command.replace("new_password", new_password);
    let _command = _command.replace("new_password", new_password);
    let command = _command.replace("username", username);

    let (code, output, error) = run_script!(&format!("{}", command), &vec![], &options).unwrap();

    (code, output, error)
}

pub fn restartservice(password: &str, servicename: &str) -> (i32, String, String) {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S systemctl reload-or-restart servicename"#;
    let _command = _command.replace("servicename", servicename);
    let command = _command.replace("password", password);

    let (code, output, error) = run_script!(&format!("{}", command), &vec![], &options).unwrap();

    (code, output, error)
}
