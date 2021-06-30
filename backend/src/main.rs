mod file;
mod db;
mod tool;
mod linux;
mod structs;
mod security;
pub mod handler;

use std::{
    env::set_var, 
    io::Result,
};
use actix_web::{
    middleware,
    App,
    HttpServer,
};

use crate::db::{
    create_tables, 
    populate_dnszones, 
    populate_zonerecords
};

const IP_ADDRESS: &str = "0.0.0.0:8080";

#[actix_web::main]
async fn main() -> Result<()> {
    
    set_var("RUST_LOG", "actix_server=info,actix_web=info");
    create_tables();
    populate_dnszones();
    populate_zonerecords();

    let server = HttpServer::new(
        move || {
            App::new()
                .wrap(middleware::Logger::default())
                        //handling / and /private/api and get request
                .service(handler::get::get_logindata)                       // link: /private/api/user/query
                .service(handler::get::get_statuspage)                      // link: /private/api/settings/status
                .service(handler::get::get_wanpage)                         // link: /private/api/settings/wirednetwork/status
                .service(handler::get::get_wlanpage)                        // link: /private/api/settings/wirelessnetwork/status
                .service(handler::get::get_wifipage)                        // link: /private/api/settings/hostapd/status
                        //handling post request
                .service(handler::post::post_pam_login)                     // link: /private/api/user/login
                .service(handler::post::post_reset_password)                // link: /private/api/user/password
                .service(handler::post::post_settings_import)               // link: /private/api/settings/import
                .service(handler::post::post_settings_export)               // link: /private/api/settings/export
                .service(handler::post::post_settings_reset)                // link: /private/api/settings/reset
                .service(handler::post::post_hostapd_settings)              // link: /private/api/settings/hostapd
                .service(handler::post::post_wireless_network_settings)     // link: /private/api/settings/wirelessnetwork
                .service(handler::post::post_static_wired_network)          // link: /private/api/settings/wirednetwork/static
                .service(handler::post::post_dynamic_wired_network)         // link: /private/api/settings/wirednetwork/dynamic   
                .service(handler::post::post_create_domain_name)            // link: /private/api/settings/dns/domain_name/creation
                .service(handler::post::post_add_zone_record)               // link: /private/api/settings/dns/zone_record/creation
                        //handling delete request
                .service(handler::delete::delete_delete_zone_record)        // link: /private/api/settings/dns/zone_record/deletion
                .service(handler::delete::delete_delete_domain_name)        // link: /private/api/settings/dns/domain_name/deletion
                        //handling put request
                .service(handler::put::put_update_dns_status)               // link: /private/api/settings/dns/status/update
                .service(handler::put::put_rename_domain_name)              // link: /private/api/settings/dns/domain_name/update
        }
    )
        .bind(IP_ADDRESS)?
        .run();
    println!("Server running at http://{}", IP_ADDRESS);
    server.await
}
