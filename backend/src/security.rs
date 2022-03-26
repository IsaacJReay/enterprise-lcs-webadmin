use crate::{
    config,
    DECRYPT_NONCE
};
use aes_gcm_siv::{Aes256GcmSiv, Key, Nonce}; // Or `Aes128GcmSiv`
use aes_gcm_siv::aead::{Aead, NewAead};
use rand::Rng;

pub fn padding_convert(password: &str) -> Vec<u8> {

    let mutpassword: String;
    let mut padding: String = String::new();

    if password.len() < 32 {
        for _i in 0..32-password.len(){
            padding = " ".to_owned() + padding.as_str();
        }
        mutpassword = password.to_owned() + padding.as_str();
    }
    else {
        mutpassword = password.to_string();
    }

    let password_vec = mutpassword.as_bytes().to_owned();

    password_vec
}


pub fn encrypt(plaintext: String, key: Vec<u8>) -> String {

    let mut stringciphertext: String = String::new();

    let key = Key::from_slice(key.as_slice());
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(DECRYPT_NONCE.as_bytes());
    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes().as_ref())
        .expect("encryption failure!");  // NOTE: handle this error to avoid panics!

    for array in ciphertext {
        stringciphertext = array.to_string() + " " + stringciphertext.as_str();
    }
    stringciphertext    

}

pub fn decrypt(encrypted_text: String, key: Vec<u8>) -> Result<String, String> {

    let mut ciphertext = encrypted_text.split_whitespace().map(|each_arg| each_arg.parse::<u8>().unwrap()).collect::<Vec<u8>>();
    ciphertext.reverse();

    let key = Key::from_slice(key.as_slice());
    let cipher = Aes256GcmSiv::new(key);
    let nonce = Nonce::from_slice(DECRYPT_NONCE.as_bytes());

    match cipher.decrypt(nonce, ciphertext.as_ref()) {
        Ok(binary) => match String::from_utf8(binary) {
            Ok(string) => Ok(string),
            Err(message) => Err(message.to_string())
        },
        Err(message) => Err(message.to_string())
    }
    
}


// pub fn encrypt_file(filename: &str, password: &str) -> String {
//     let mut byte_file = config::get_file_as_byte_vec(&filename.to_string());

//     byte_file.reverse();

//     let mut string_file = String::new();

//     for each_u8 in byte_file {
//         string_file = each_u8.to_string() + " " + string_file.as_str();
//     }

//     let enc_o_string = encrypt(string_file, padding_convert(password));

//     let processed_file = filename.replace("tar.zst", "kconf");

//     let _result = config::createfile(&processed_file, enc_o_string.as_bytes());

//     processed_file
// }

pub fn decrypt_file(filename: &str, password: &str) -> String {
    let byte_file = String::from_utf8(config::get_file_as_byte_vec(&filename.to_string())).unwrap();
    let mut byte_file_sanitized = byte_file.split_whitespace().collect::<Vec<&str>>();
    byte_file_sanitized.reverse();
    let mut string_byte_file: String = String::new();

    for each_str in byte_file_sanitized {
        string_byte_file = each_str.to_owned() + " " + string_byte_file.as_str();
    }

    let decrypted_byte = decrypt(string_byte_file, padding_convert(password)).unwrap();

    let decrypted_byte_vec_str = decrypted_byte.split_whitespace().collect::<Vec<&str>>();

    let mut decrypted_byte_vec_u8: Vec<u8> = Vec::new();

    for each_str in decrypted_byte_vec_str {
        decrypted_byte_vec_u8.push(each_str.parse::<u8>().unwrap());
    }

    let file = filename.replace("kconf", "tar.zst");

    let _result = config::createfile(&file, &decrypted_byte_vec_u8);

    file
}

pub fn generate_random(string_len: usize, custom_charset: Option<String>) -> String{

    let custom_charset = match custom_charset {
        Some(set) => set,
        None => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789)(*&^%$#@!~".to_string()
    };

    let mut rng = rand::thread_rng();

    let random_string: String = (0..string_len)
        .map(|_| {
            let idx = rng.gen_range(0..custom_charset.as_bytes().len());
            custom_charset.as_bytes()[idx] as char
        })
        .collect();

    random_string
}