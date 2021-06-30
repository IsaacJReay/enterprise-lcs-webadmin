use actix_web::{
    HttpResponse, 
    Result, 
    put,
    web,
};
use crate::{
    db,
    handler::helper::return_httpsresponse_from_config_named_conf_external_zone,
    structs::{
        UpdateStatus,
        RenameDomain,
    },
};

#[put("/private/api/settings/dns/status/update")]
pub async fn put_update_dns_status(update_status_struct: web::Json<UpdateStatus>) -> Result<HttpResponse> {
    let id = update_status_struct.id.id.clone();
    let status = update_status_struct.status;

    db::insert_status_into_dnszone_by_id(id.as_str(), status);

    return_httpsresponse_from_config_named_conf_external_zone()
}

#[put("/private/api/settings/dns/domain_name/update")]
pub async fn put_rename_domain_name(rename_domain_struct: web::Json<RenameDomain>) -> Result<HttpResponse> {
    
    let foreign_key: String = rename_domain_struct.foreign_key.foreign_key.clone();
    let new_domain_name: String = rename_domain_struct.new_domain_name.clone();
    db::update_domain_name_by_foreign_key(&foreign_key, &new_domain_name);

    return_httpsresponse_from_config_named_conf_external_zone()
}

