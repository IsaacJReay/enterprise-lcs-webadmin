use actix_web::{
    HttpResponse, 
    Result, 
};
use crate::{
    config,
    db,
    structs::{
        HttpResponseCustom,
        PartialZoneRecords,
    },
};

pub fn return_httpsresponse_from_config_named_conf_external_zone() -> Result<HttpResponse> {
    let (
        cleanup_named_status,
        write_named_status, 
        move_named_status, 
        restart_named_status
    ) = config::config_name_conf_external_zones();

    if cleanup_named_status {
        if write_named_status {
            if move_named_status {
                if restart_named_status {
                    Ok(HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Success".to_string(),
                                reason: "".to_string(),
                            }
                        )  
                    )
                }
                else {
                    Ok(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "restart_service_error".to_string(),
                            }
                        )
                    )
                }
            }
            else {
                Ok(
                    HttpResponse::InternalServerError().json(
                        HttpResponseCustom {
                            operation_status: "Failed".to_string(),
                            reason: "move_file_error".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::InternalServerError().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "write_file_error".to_string(),
                    }
                )
            )
        }
    }
    else{
        Ok(
            HttpResponse::InternalServerError().json(
                HttpResponseCustom {
                    operation_status: "Failed".to_string(),
                    reason: "clean-file-error".to_string(),
                }
            )
        )
    }
}

pub fn return_httpsresponse_from_config_var_named_external_zone(foreign_key: &str) -> Result<HttpResponse> {
    let zone_vec = db::named::read_zonerecords_by_foreign_key(foreign_key);
    let mut record_vec: Vec<PartialZoneRecords> = Vec::new();
    for increments in 0..zone_vec.len(){
        record_vec.insert(
            increments, 
            PartialZoneRecords {
                subdomain_name: zone_vec[increments].partial_zonerecords.subdomain_name.to_owned(),
                dns_type: zone_vec[increments].partial_zonerecords.dns_type.to_owned(),
                address: zone_vec[increments].partial_zonerecords.address.to_owned(),
                foreign_key: zone_vec[increments].partial_zonerecords.foreign_key.to_owned(),
            }
        );
    }
    
    let (
        write_var_zone_status, 
        move_var_zone_status, 
        restart_named_status
    ) = config::config_var_named_external_zones(record_vec);


        if write_var_zone_status{
            if move_var_zone_status {
                if restart_named_status {
                    Ok(
                        HttpResponse::Ok().json(
                            HttpResponseCustom {
                                operation_status: "Success".to_string(),
                                reason: "".to_string(),
                            }
                        )
                    )
                }
                else{
                    Ok(
                        HttpResponse::InternalServerError().json(
                            HttpResponseCustom {
                                operation_status: "Failed".to_string(),
                                reason: "restart_service_error".to_string(),
                            }
                        )
                    )
                }
            }
            else {
                Ok(
                    HttpResponse::InternalServerError().json(
                        HttpResponseCustom {
                            operation_status: "Failed".to_string(),
                            reason: "move_file_error".to_string(),
                        }
                    )
                )
            }
        }
        else{
            Ok(
                HttpResponse::InternalServerError().json(
                    HttpResponseCustom {
                        operation_status: "Failed".to_string(),
                        reason: "write_file_error".to_string(),
                    }
                )
            )
        }
}