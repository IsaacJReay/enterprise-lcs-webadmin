use run_script::{run_script, ScriptOptions};

pub fn create_update_script() {
    let options = ScriptOptions::new();

    let command = r#"echo '#!/bin/bash
arr=( "$@" ); for ((y=0; y<$#; y++));do val="$val ${arr[$y]}";done; yes | pacman -U $val --overwrite="*"' | tee /tmp/sys_update_script;"#;

    let (_code, _output, _error) = run_script!(&format!("{}", command), &vec![], &options).unwrap();
}

pub fn update_sys_pacman(password: &str, package_folder: &str) -> bool {
    let options = ScriptOptions::new();

    let _command = r#"echo password | sudo -S bash /tmp/sys_update_script package_folder/* "#;
    let _command = _command.replace("package_folder", package_folder);
    let command = _command.replace("password", password);

    let (code, _output, _error) = run_script!(&format!("{}", command), &vec![], &options).unwrap();

    code == 0 // return statement

}

pub fn update_patch_script(password: &str, directory_path: &str) -> bool {
    let options = ScriptOptions::new();

    let _command =
        r#"cd directory_path && echo password | sudo -S bash *.sh"#;
    let _command = _command.replace("directory_path", directory_path);
    let command = _command.replace("password", password);

    let (code, _output, _error) = run_script!(&format!("{}", command), &vec![], &options).unwrap();

    code == 0 // return statement

}
