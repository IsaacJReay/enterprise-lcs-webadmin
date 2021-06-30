use actix_web::{
    HttpResponse, 
    Result, 
    delete,
    web,
};
use crate::{
    db,
    handler::helper::{
        return_httpsresponse_from_config_var_named_external_zone,
        return_httpsresponse_from_config_named_conf_external_zone,
    },
    structs::{
        DeleteRecord,
        DnsId,
    }
};

#[delete("/private/api/settings/dns/zone_record/deletion")]
pub async fn delete_delete_zone_record(delete_record_struct: web::Json<DeleteRecord>) -> Result<HttpResponse> {
    let foreign_key = &delete_record_struct.foreign_key.foreign_key.to_owned();
    let id = &delete_record_struct.id.id.to_owned();

    db::delete_from_zonerecords_by_id(
        id, 
        &foreign_key,
    );

    return_httpsresponse_from_config_var_named_external_zone(foreign_key)
}

#[delete("/private/api/settings/dns/domain_name/deletion")]
pub async fn delete_delete_domain_name(dns_id_struct: web::Json<DnsId>) -> Result<HttpResponse> {
    db::delete_from_dnszones_by_id(dns_id_struct.id.to_owned().as_str());

    return_httpsresponse_from_config_named_conf_external_zone()
}


