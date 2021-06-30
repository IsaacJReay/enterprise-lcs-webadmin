use crate::file;
use aes::Aes128;
use hex_literal::hex;
use block_modes::{
    BlockMode, 
    Cbc, 
    block_padding::Pkcs7
};

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