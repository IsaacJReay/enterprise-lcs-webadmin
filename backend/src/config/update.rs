use std::convert::TryInto;

use crate::{
    linux::storage::remove_filedir_root,
    config::{
        download_file,
        read_file,
        continue_file,
        insert_update_information_to_toml,
        remove_update_information_from_toml,
        untar_file,
    },
    structs::{
        SystemUpdateInformation,
        ContentServerUpdate
    },
    linux::update::{
        update_patch_script,
        update_sys_pacman
    }
};

pub fn display_new_update_lists() -> Vec<SystemUpdateInformation> {

    download_file("https://dev.koompi.org/contentserver/update_db.toml", continue_file("/tmp/update_db.toml"));

    let new_update = toml::from_str::<ContentServerUpdate>(&read_file("/tmp/update_db.toml")).unwrap();
    let current_update = toml::from_str::<ContentServerUpdate>(&read_file("/kmp/update_db.toml")).unwrap();
    let current_installing = toml::from_str::<ContentServerUpdate>(&read_file("/tmp/update_db.toml.installing")).unwrap();
    let current_downloading = toml::from_str::<ContentServerUpdate>(&read_file("/tmp/update_db.toml.downloading")).unwrap();

    let mut vec_updatable: Vec<SystemUpdateInformation> = Vec::new();
    let mut vec_current_patch_update: Vec<String> = Vec::new();
    let mut vec_new_patch_update: Vec<String> = Vec::new();
    let mut vec_downloading_patch_update: Vec<String> = Vec::new();
    let mut vec_installing_patch_update: Vec<String> = Vec::new();
    
    let current_sys_update = match current_update.sys_update {
        Some(update) => update.keys().last().unwrap().to_owned(),
        None => String::new()
    };
    let current_downloading_sys_update = match current_downloading.sys_update {
        Some(update) => update.keys().last().unwrap().to_owned(),
        None => String::new()
    };
    let current_installing_sys_update = match current_installing.sys_update {
        Some(update) => update.keys().last().unwrap().to_owned(),
        None => String::new()
    };
    let new_sys_update = match new_update.sys_update.as_ref() {
        Some(update) => update.keys().last().unwrap().to_owned(),
        None => String::new()
    };
    match current_update.patch_update {
        Some(update) => update.keys().for_each(|each_key| vec_current_patch_update.push(each_key.to_string())),
        None => vec_current_patch_update.push(String::new())
    };
    match new_update.patch_update.as_ref() {
        Some(update) => update.keys().for_each(|each_key| vec_new_patch_update.push(each_key.to_string())),
        None => vec_new_patch_update.push(String::new())
    };
    match current_installing.patch_update.as_ref() {
        Some(update) => update.keys().for_each(|each_key| vec_installing_patch_update.push(each_key.to_string())),
        None => vec_installing_patch_update.push(String::new())
    };
    match current_downloading.patch_update.as_ref() {
        Some(update) => update.keys().for_each(|each_key| vec_downloading_patch_update.push(each_key.to_string())),
        None => vec_downloading_patch_update.push(String::new())
    };

    match current_sys_update == new_sys_update {
        true => (),
        false => {
            let current_display_name =  new_update.sys_update.as_ref().unwrap().get(&new_sys_update).unwrap().get("display_name").unwrap().as_str().unwrap().to_string();
            let current_update_size = new_update.sys_update.as_ref().unwrap().get(&new_sys_update).unwrap().get("size").unwrap().as_integer().unwrap().try_into().unwrap();
            let current_status = match current_downloading_sys_update == new_sys_update {
                true => String::from("Downloading"),
                false => match current_installing_sys_update == new_sys_update {
                    true => String::from("Installing"),
                    false => String::from("New")
                }
            };
        
            vec_updatable.push(
                SystemUpdateInformation {
                    id: new_sys_update.clone(),
                    display_name: current_display_name,
                    update_size: current_update_size,
                    sys_update: true,
                    status: current_status
                }
            )
        }
    };

    for each_update in vec_new_patch_update {
        match vec_current_patch_update.contains(&each_update) {
            true => (),
            false => {
                let current_update_info = new_update.patch_update.as_ref().unwrap().get(&each_update).unwrap();
                let current_display_name = current_update_info.get("display_name").unwrap().as_str().unwrap().to_string();
                let current_update_size = current_update_info.get("size").unwrap().as_integer().unwrap().try_into().unwrap();
                let current_status = match vec_downloading_patch_update.contains(&each_update) {
                    true => String::from("Downloading"),
                    false => match vec_installing_patch_update.contains(&each_update) {
                        true => String::from("Installing"),
                        false => String::from("New"),
                    }
                };
                vec_updatable.push(
                    SystemUpdateInformation {
                        id: each_update,
                        display_name: current_display_name,
                        update_size: current_update_size,
                        sys_update: false,
                        status: current_status
                    }
                );
            }
        }
    }

    vec_updatable 
}

pub fn query_updatable_depedencies_update_content_server(id: &str, sys_update: bool) -> Vec<SystemUpdateInformation> {

    let mut vec_updatable: Vec<SystemUpdateInformation> = Vec::new();
    query_all_depedencies_update_content_server(&mut vec_updatable, id, sys_update);

    display_new_update_lists().iter().for_each(
        |each_update_info| {
            vec_updatable.iter_mut().for_each(
                |each_current_update_info| {
                    if each_current_update_info.get_id() == each_update_info.get_id() && 
                    each_current_update_info.get_sys_update() == each_update_info.get_sys_update() {
                        each_current_update_info.status = each_update_info.get_status();
                    }
                }
            )
        }
    );

    vec_updatable.into_iter().filter(|each_update| each_update.get_status() == "New").collect::<Vec<SystemUpdateInformation>>()



}

fn query_all_depedencies_update_content_server(vec_updatable: &mut Vec<SystemUpdateInformation>, id: &str, is_sys_update: bool)  { 

    // read new update information
    let new_update = toml::from_str::<ContentServerUpdate>(&read_file("/tmp/update_db.toml")).unwrap();
    let current_update = toml::from_str::<ContentServerUpdate>(&read_file("/kmp/update_db.toml")).unwrap();

    
    // get the dependencies and systemupdate update lists of the selected update from ID from toml 
    let update_info = match &is_sys_update {
        true => new_update.sys_update.as_ref().unwrap().get(id).unwrap(),
        false => new_update.patch_update.as_ref().unwrap().get(id).unwrap()
    };
    
    // get the selected_update update's dependencies if it isn't a systemupdate, 
    // because a systemupdate doesn't depend on anythings
    let get_dependencies_value = match &is_sys_update {
        true => None,
        false => Some(update_info.get("depend").unwrap())
    };

    // This vector will host the ID of all the patch update is needed
    let mut depend_on_patch_update: Vec<u16> = Vec::new(); 

    // check if the update depends on an up-to-dated systemupdate, 
    // and get the systemupdate information if it does
    let depend_on_sys_update: bool = match get_dependencies_value.as_ref() {
        Some(current_value) => current_value.get("sys_update").unwrap().as_bool().unwrap(),
        None => false,
    };

    // check if the update depends on any patch updates and get the list of patch update if it does
    match get_dependencies_value.as_ref() {
        Some(current_value) => match current_value.get("patch_update") {
            Some(value) => value.as_array().unwrap().into_iter().for_each(|each_value| depend_on_patch_update.push(each_value.as_integer().unwrap().try_into().unwrap())),  
            None => {},
        }
        None => {},
    };

    // Operation for update systemupdate if the current update needs systemupdate
    if depend_on_sys_update == true {
        // get the latest systemupdate ID
        let new_update_id = new_update.sys_update.as_ref().unwrap().keys().next().unwrap().to_owned();

        // try to find the new update id within the local systemupdate database, and return false if it doesn't exists
        // because if it doesn't exists, it hasn't been updated yet
        let sys_up_to_date = match current_update.sys_update.as_ref() {
            Some(data) => match data.get(&new_update_id) {
                Some(_data) => true,
                None => false
            },
            None => false,
        };
 
        if sys_up_to_date != true {
            let new_update_info = new_update.sys_update.as_ref().unwrap().get(&new_update_id).unwrap();
            let current_display_name = new_update_info.get("display_name").unwrap().as_str().unwrap().to_string();
            let current_update_size = new_update_info.get("size").unwrap().as_integer().unwrap().try_into().unwrap();
            let current_info = SystemUpdateInformation {
                id: new_update_id,
                display_name: current_display_name,
                update_size: current_update_size,
                sys_update: true,
                status: String::from("Undefine")
            };

            if !vec_updatable.contains(&current_info) {
                vec_updatable.push(current_info)
            }
            
        }
    };

    // Operation for when the current update needs patch_update 
    if !depend_on_patch_update.is_empty() {
        // Loop to go through all of the patch update ID (new_update_id) from the vector above one by one
        depend_on_patch_update.iter().for_each(|new_update_id| {
            // convert the ID originally in U16 into String for query with update database
            let new_update_id = new_update_id.to_string();

            // try to find the new update id within the local patch update database, and return false if it doesn't exists
            // because if it doesn't exists, it hasn't been updated yet
            let patch_up_to_date = match &current_update.patch_update {
                Some(data) => match data.get(new_update_id.as_str()) {
                    Some(_data) => true,
                    None => false
                },
                None => false,
            };

            // Operation for when the content server is missing the patch dependencies
            if patch_up_to_date != true {              
                // Check the depedencies of the current dependencies
                query_all_depedencies_update_content_server(vec_updatable, &new_update_id, false);
                let new_update_info = new_update.patch_update.as_ref().unwrap().get(&new_update_id).unwrap();
                let current_display_name = new_update_info.get("display_name").unwrap().as_str().unwrap().to_string();
                let current_update_size = new_update_info.get("size").unwrap().as_integer().unwrap().try_into().unwrap();
                let current_info = SystemUpdateInformation {
                    id: new_update_id,
                    display_name: current_display_name,
                    update_size: current_update_size,
                    sys_update: false,
                    status: String::from("Undefine")
                };

                if !vec_updatable.contains(&current_info) {
                    vec_updatable.push(current_info)
                }
            } 
        });
    }

}

pub fn update_content_server(password: &str, id: &str, is_sys_update: bool) {
    let all_new_update_information = toml::from_str::<ContentServerUpdate>(&read_file("/tmp/update_db.toml")).unwrap();
    let vec_updatable = query_updatable_depedencies_update_content_server(id, is_sys_update);

    let mut download_status: bool = true;
    let mut install_status: bool = true;

    for each_update in &vec_updatable {
        let current_update_information = match each_update.get_sys_update() {
            true => all_new_update_information.sys_update.as_ref().unwrap(),
            false => all_new_update_information.patch_update.as_ref().unwrap(),
        }.get(&each_update.get_id()).unwrap();

        insert_update_information_to_toml(
            (true, false), 
            &each_update.get_id(), 
            current_update_information, 
            each_update.get_sys_update()
        );
    }    

    for each_update in &vec_updatable {
        let current_update_information = match each_update.get_sys_update() {
            true => all_new_update_information.sys_update.as_ref().unwrap(),
            false => all_new_update_information.patch_update.as_ref().unwrap(),
        }.get(&each_update.get_id()).unwrap();

        let filename = current_update_information.get("path").unwrap().as_str().unwrap();

        let output_file = continue_file(&("/tmp/".to_owned()+filename));

        if !download_file(&("https://dev.koompi.org/contentserver/".to_owned()+filename), output_file) {
            download_status = false
        };
    }

    for each_update in &vec_updatable {
        let current_update_information = match each_update.get_sys_update() {
            true => all_new_update_information.sys_update.as_ref().unwrap(),
            false => all_new_update_information.patch_update.as_ref().unwrap(),
        }.get(&each_update.get_id()).unwrap();

        remove_update_information_from_toml((true, false), &each_update.get_id(), each_update.get_sys_update());
        if download_status {
            insert_update_information_to_toml(
                (false, true), 
                &each_update.get_id(), 
                current_update_information, 
                each_update.get_sys_update()
            );
        }
    }

    if download_status {
        for each_update in &vec_updatable {
            let current_update_information = match each_update.get_sys_update() {
                true => all_new_update_information.sys_update.as_ref().unwrap(),
                false => all_new_update_information.patch_update.as_ref().unwrap(),
            }.get(&each_update.get_id()).unwrap();
    
            let filename = current_update_information.get("path").unwrap().as_str().unwrap();
            let extract_location = &("/tmp/".to_owned()+&filename+"_extract");
    
            if !untar_file(&("/tmp/".to_owned()+&filename), extract_location) {
                match each_update.get_sys_update() {
                    true => if !update_sys_pacman(password, extract_location) {
                        install_status = false
                    },
                    false => if !update_patch_script(password, extract_location) {
                        install_status = false
                    },
                };
                
            };
        }
    }

    for each_update in &vec_updatable {
        let current_update_information = match each_update.get_sys_update() {
            true => all_new_update_information.sys_update.as_ref().unwrap(),
            false => all_new_update_information.patch_update.as_ref().unwrap(),
        }.get(&each_update.get_id()).unwrap();

        remove_update_information_from_toml((false, true), &each_update.get_id(), each_update.get_sys_update());

        if download_status && install_status {
            if each_update.get_sys_update() {
                let old_update_informaton = toml::from_str::<ContentServerUpdate>(&read_file("/kmp/update_db.toml")).unwrap();
                match old_update_informaton.sys_update.as_ref() {
                    Some(update) => remove_update_information_from_toml((false, false), update.keys().last().unwrap(), each_update.get_sys_update()),
                    None => ()
                }
            }
            insert_update_information_to_toml((false, false), &each_update.get_id(), current_update_information, each_update.get_sys_update());
        }
    }
    remove_filedir_root(password, "/tmp/update_db.lock");
}