mod config;
mod db;
mod tool;
mod linux;
mod structs;
mod security;
mod handler;

use std::{
    env::set_var, 
    io::Result,
};
use actix_web::{
    middleware,
    // http,
    App,
    HttpServer,
};
use actix_cors::Cors;
use crate::db::{
    create_tables, 
    named::populate_dnszones, 
    named::populate_zonerecords,
};

const IP_ADDRESS: &str = "0.0.0.0:8080";
const DECRYPT_KEY: &str = "Koompi-Onelab"; // Cannot Exceed 32 characters
const DECRYPT_NONCE: &str = "KoompiOnelab"; // Cannot Exceed 12 characters

#[actix_web::main]
async fn main() -> Result<()> {
    
    set_var("RUST_LOG", "actix_server=info,actix_web=info");
    create_tables();
    populate_dnszones();
    populate_zonerecords();
    // let production_cors = Cors::default()
    //           .allowed_origin("http://localhost:3000")
    //           .allowed_origin("http://127.0.0.1:3000")
    //           .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
    //           .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
    //           .allowed_header(http::header::CONTENT_TYPE)
    //           .max_age(900); 
    // let development_cors = Cors::permissive();

    let server = HttpServer::new(
        move || {
            App::new()
                .wrap(Cors::permissive())
                .wrap(middleware::Logger::default())
                                                            //handling GET request
                .service(handler::get::systemsettings::get_token_validated)                 // link: /private/api/token/validation
                .service(handler::get::users::get_logindata)                                // link: /private/api/user/query
                .service(handler::get::systemsettings::get_statuspage)                      // link: /private/api/settings/status
                .service(handler::get::systemdnetworkd::get_wanpage)                        // link: /private/api/settings/wirednetwork/status
                .service(handler::get::systemdnetworkd::get_wlanpage)                       // link: /private/api/settings/wirelessnetwork/status
                .service(handler::get::hostapd::get_wifipage)                               // link: /private/api/settings/hostapd/status
                .service(handler::get::named::get_domain_name_page)                         // link: /private/api/settings/dns/domain_name/status
                .service(handler::get::named::get_zone_record_page)                         // link: /private/api/settings/dns/zone_records/status
                .service(handler::get::timedatectl::get_timedatepage)                       // link: /private/api/settings/time/status
                .service(handler::get::storage::get_storage_page)                           // link: /private/api/settings/storage/status
                .service(handler::get::storage::get_storage_device_page_test)               // link: /private/api/settings/storage/device/status/{drive_partuuid}
                                                            //handling POST request
                .service(handler::post::users::post_pam_login)                              // link: /private/api/user/login
                .service(handler::post::users::post_reset_password)                         // link: /private/api/user/password
                .service(handler::post::systemsettings::post_settings_import)               // link: /private/api/settings/import
                .service(handler::post::systemsettings::post_settings_export)               // link: /private/api/settings/export
                .service(handler::post::systemsettings::post_settings_reset)                // link: /private/api/settings/reset
                .service(handler::post::hostapd::post_hostapd_settings)                     // link: /private/api/settings/hostapd
                .service(handler::post::systemdnetworkd::post_wireless_network_settings)    // link: /private/api/settings/wirelessnetwork
                .service(handler::post::systemdnetworkd::post_static_wired_network)         // link: /private/api/settings/wirednetwork/static
                .service(handler::post::systemdnetworkd::post_dynamic_wired_network)        // link: /private/api/settings/wirednetwork/dynamic   
                .service(handler::post::named::post_create_domain_name)                     // link: /private/api/settings/dns/domain_name/creation
                .service(handler::post::named::post_add_zone_record)                        // link: /private/api/settings/dns/zone_record/creation
                .service(handler::post::timedatectl::post_set_time)                         // link: /private/api/settings/time/timedate
                .service(handler::post::timedatectl::post_set_timezone)                     // link: /private/api/settings/time/timezone
                .service(handler::post::storage::post_storage_device_copy_or_move)          // link: /private/api/settings/storage/device/copy_or_move
                .service(handler::post::storage::post_storage_device_directory_creation)    // link: /private/api/settings/storage/device/directory/creation
                .service(handler::post::storage::post_storage_device_unmount)               // link: /private/api/settings/storage/device/unmount
                                                            //handling DELETE request
                .service(handler::delete::delete_delete_zone_record)                        // link: /private/api/settings/dns/zone_record/deletion
                .service(handler::delete::delete_delete_domain_name)                        // link: /private/api/settings/dns/domain_name/deletion
                .service(handler::delete::post_storage_device_remove_filedir)        // link: /private/api/settings/storage/device/deletion
                                                            //handling PUT request
                .service(handler::put::put_update_dns_status)                               // link: /private/api/settings/dns/status/update
                .service(handler::put::put_rename_domain_name)                              // link: /private/api/settings/dns/domain_name/update
        }
    )
        .bind(IP_ADDRESS)?
        .run();
    println!("Server running at http://{}", IP_ADDRESS);
    server.await
}
