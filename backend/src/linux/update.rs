use run_script::{
    ScriptOptions, 
    run_script
};

pub fn create_update_script() {
    let options = ScriptOptions::new();

    let command = 
r#"echo '#!/bin/bash
arr=( "$@" ); for ((y=0; y<$#; y++));do val="$val ${arr[$y]}";done;yes | pacman -U $val' | tee /tmp/sys_update_script && chmod +x /tmp/update_script && echo '#!/bin/bash
cd $1 && for ((y=3; y<$#; y++));do val="$val ${arr[$y]}";done; echo $2 | sudo -S bash $val' | tee /tmp/patch_update_script"#;

    let (_code, _output, _error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();
}

pub fn update_sys_pacman(password: &str, package_folder: &str) -> bool {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S bash /tmp/sys_update_script 'package_folder/*' "#;
    let _command = _command.replace("package_folder", package_folder);
    let command = _command.replace("password", password);

    let (code, _output, _error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    match code {
        0 => true,
        _ => false
    }
}

pub fn update_patch_script(password: &str, scripts_list: &str) -> bool {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S bash /tmp/patch_update_script directory_path password ''"#;
    let _command = _command.replace("scripts_list", scripts_list);
    let command = _command.replace("password", password);

    let (code, _output, _error) = run_script!(
        &format!("{}", command),
        &vec![],
        &options
    ).unwrap();

    match code {
        0 => true,
        _ => false
    }
}