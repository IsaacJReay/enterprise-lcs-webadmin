use crate::linux;
use std::{
    usize,
    isize,
    convert::TryInto
};

pub fn comparedate(olddate: u64) -> bool {

    let mut newdate: u64 = 18446744073709551615; //max value of u64
    let (code, output, error) = linux::query_date_for_calculate();

    match &code {
        1 => println!("{}", &error),
        0 => newdate = output.to_owned().as_str().parse::<u64>().unwrap(),
        _ => println!("Broken"),
    }

    let elapse: u64 = newdate - olddate;

    if elapse <= 1500 {
        true
    }
    else{
        false
    }
}

pub fn from_binary(binary_ip: String) -> String {

    let binary_ip_length: usize = binary_ip.len();
    let mut full_binary_ip = binary_ip;
    if binary_ip_length < 32 {
        let variation: usize = 32 - binary_ip_length;
        for each_fill in 0..variation{
            full_binary_ip = format!("{}{}", 0, full_binary_ip);
        }
    }
 
    let mut splited_binary_ip: Vec<&str> = full_binary_ip.split("").collect();
    let mut octet_index: usize = 0;

    let mut first_octet_vec: Vec<&str> = Vec::new();
    let mut second_octet_vec: Vec<&str> = Vec::new();
    let mut third_octet_vec: Vec<&str> = Vec::new();
    let mut fourth_octet_vec: Vec<&str> = Vec::new();

    let mut first_octet_string: String = String::new();
    let mut second_octet_string: String = String::new();
    let mut third_octet_string: String = String::new();
    let mut fourth_octet_string: String = String::new();


    splited_binary_ip.retain(|element| element != &"");

    for increment in 0..splited_binary_ip.len() {
        if increment == 7 || increment == 15 || increment == 23 {
            octet_index = 0;
        }
        if increment <= 7 {
            first_octet_vec.insert(octet_index, splited_binary_ip[increment]);
        }
        else if increment <= 15 {
            second_octet_vec.insert(octet_index, splited_binary_ip[increment]);
        }
        else if increment <= 23 {
            third_octet_vec.insert(octet_index, splited_binary_ip[increment]);
        }
        else if increment <= 31 {
            fourth_octet_vec.insert(octet_index, splited_binary_ip[increment]);
        }
    }
    
    first_octet_vec.reverse();
    second_octet_vec.reverse();
    third_octet_vec.reverse();
    fourth_octet_vec.reverse();

    for increments in 0..8 {
        first_octet_string.push_str(&first_octet_vec[increments]);
        second_octet_string.push_str(&second_octet_vec[increments]);
        third_octet_string.push_str(&third_octet_vec[increments]);
        fourth_octet_string.push_str(&fourth_octet_vec[increments]);
    }

    format!("{}.{}.{}.{}", 
        isize::from_str_radix(&first_octet_string, 2).unwrap(), 
        isize::from_str_radix(&second_octet_string, 2).unwrap(), 
        isize::from_str_radix(&third_octet_string, 2).unwrap(),
        isize::from_str_radix(&fourth_octet_string,2).unwrap()
    )

}

pub fn to_binary(ip: String) -> usize {

    let splited_ip = ip.split(".").collect::<Vec<&str>>();
    
    let concated_binary_ip = 
        padding_convert(splited_ip[0].to_string())                  //First Octet of IP
    +
        padding_convert(splited_ip[1].to_string()).as_str()         //Second_Octet of IP
    +
        padding_convert(splited_ip[2].to_string()).as_str()         //Third_Octet of IP
    +
        padding_convert(splited_ip[3].to_string()).as_str();        //Fourth_Octet of IP

    let binary: usize = isize::from_str_radix(&concated_binary_ip, 2).unwrap().try_into().unwrap();
    binary

}

pub fn padding_convert(octet: String) -> String {

    let mut binary_octet = format!("{:b}", octet.parse::<u8>().unwrap());
    let mut zeros: String = String::new();

    for _i in binary_octet.chars().count()..8{
        zeros = "0".to_string() + zeros.as_str();
    }

    binary_octet = zeros + binary_octet.as_str();
    binary_octet

}
