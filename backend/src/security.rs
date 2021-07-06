use crate::{
    file,
    linux
};
use aes::Aes128;
use hex_literal::hex;
use block_modes::{
    BlockMode, 
    Cbc, 
    block_padding::Pkcs7
};
use rand::Rng;

pub fn padding_convert(password: &str) -> [u8; 16] {

    let mutpassword: String;
    let mut padding: String = String::new();

    if password.len() < 16 {
        for _i in 0..16-password.len(){
            padding = " ".to_owned() + padding.as_str();
        }
        mutpassword = password.to_owned() + padding.as_str();
    }
    else {
        mutpassword = password.to_string();
    }

    let password_vec = mutpassword.as_bytes().to_owned();
    
    let mut password_array: [u8; 16] = [0; 16];

    for each_index in 0..16 {
        password_array[each_index] = password_vec[each_index];
    }
    
    password_array
}

pub fn encrypt(plaintext: String, key: [u8; 16]) -> String {
    
    type Aes128Cbc = Cbc<Aes128, Pkcs7>;

    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let byteplaintext = plaintext.as_bytes();
    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let mut ciphertext = cipher.encrypt_vec(byteplaintext);
    ciphertext.reverse();
    let mut stringciphertext: String = String::new();

    for array in ciphertext {
        stringciphertext = array.to_string() + " " + stringciphertext.as_str();
    }
    stringciphertext
}

pub fn decrypt(encrypted_text: String, key: [u8; 16]) -> String {

    type Aes128Cbc = Cbc<Aes128, Pkcs7>;
    let mut encrypted_vec_u8: Vec<u8> = Vec::new();
    
    let iv = hex!("f0f1f2f3f4f5f6f7f8f9fafbfcfdfeff");
    let mut encrypted_vec_str = encrypted_text.split(" ").collect::<Vec<&str>>();
    encrypted_vec_str.remove(encrypted_vec_str.len()-1);

    for text in encrypted_vec_str{
        encrypted_vec_u8.push(text.parse::<u8>().unwrap());
    }

    let ciphertext: &[u8] = &encrypted_vec_u8;

    let cipher = Aes128Cbc::new_from_slices(&key, &iv).unwrap();
    let decrypted_ciphertext = cipher.decrypt_vec(&ciphertext).unwrap();
    let decryptedstring = String::from_utf8(decrypted_ciphertext).unwrap();
    decryptedstring
}

pub fn encrypt_file(filename: &str, password: &str) -> String {
    let mut byte_file = file::get_file_as_byte_vec(&filename.to_string());

    byte_file.reverse();

    let mut string_file = String::new();

    for each_u8 in byte_file {
        string_file = each_u8.to_string() + " " + string_file.as_str();
    }

    let enc_o_string = encrypt(string_file, padding_convert(password));

    let processed_file = filename.replace("tar.zst", "kconf");

    let _result = file::createfile(&processed_file, enc_o_string.as_bytes());

    processed_file
}

pub fn decrypt_file(filename: &str, password: &str) -> String {
    let byte_file = String::from_utf8(file::get_file_as_byte_vec(&filename.to_string())).unwrap();
    let mut byte_file_sanitized = byte_file.split_whitespace().collect::<Vec<&str>>();
    byte_file_sanitized.reverse();
    let mut string_byte_file: String = String::new();

    for each_str in byte_file_sanitized {
        string_byte_file = each_str.to_owned() + " " + string_byte_file.as_str();
    }

    let decrypted_byte = decrypt(string_byte_file, padding_convert(password));

    let decrypted_byte_vec_str = decrypted_byte.split_whitespace().collect::<Vec<&str>>();

    let mut decrypted_byte_vec_u8: Vec<u8> = Vec::new();

    for each_str in decrypted_byte_vec_str {
        decrypted_byte_vec_u8.push(each_str.parse::<u8>().unwrap());
    }

    let file = filename.replace("kconf", "tar.zst");

    let _result = file::createfile(&file, &decrypted_byte_vec_u8);

    file
}

pub fn generate_token(username: &str, password: &str) -> String{

    let (_code, output, _error) = linux::query_date_for_calculate();

    let encrypted_userame = encrypt(username.to_string(), padding_convert("Koompi-Onelab"));
    let encrypted_password= encrypt(password.to_string(), padding_convert("Koompi-Onelab"));
    let encrypted_time = encrypt(output, padding_convert("Koompi-Onelab"));

    
    let random_text1 = encrypt(generate_random(32), padding_convert("Koompi-Onelab"));

    let random_text2 = encrypt(generate_random(32), padding_convert("Koompi-Onelab"));

    let token: String = format!("{}.{}.{}.{}.{}", base64::encode(random_text1), base64::encode(encrypted_userame), base64::encode(encrypted_password), base64::encode(encrypted_time), base64::encode(random_text2));

    token
}

pub fn extract_token(auth: &str) -> u64 {
    let auth_split_whitespace_vec=auth.split_ascii_whitespace().collect::<Vec<&str>>();
    let auth_split_dot_vec: Vec<&str> = auth_split_whitespace_vec[1].split(".").collect::<Vec<&str>>();
    let decoded_base64 = base64::decode(auth_split_dot_vec[3]).unwrap();
    let base64_string = String::from_utf8(decoded_base64).unwrap();
    let decrypted_base64 = decrypt(base64_string, padding_convert("Koompi-Onelab")).parse::<u64>().unwrap();               
    decrypted_base64
}

fn generate_random(string_len: usize) -> String{
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let random_string: String = (0..string_len)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    random_string
}