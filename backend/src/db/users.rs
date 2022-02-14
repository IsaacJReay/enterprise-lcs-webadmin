use sqlite::State;
use crate::{ 
    security, 
    DECRYPT_KEY,
};
use std::convert::TryInto;


pub fn insert_into_token_table(token: &str){
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    connection
        .execute(
            format!("INSERT INTO tokentable VALUES ('{}');", token)
        )
            .unwrap();
}

pub fn delete_from_token_table(token: &str) {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let auth_split_whitespace_vec = token.split_ascii_whitespace().collect::<Vec<&str>>();

    connection
        .execute(
            format!("DELETE FROM tokentable WHERE token='{}';", auth_split_whitespace_vec[1])
        )
            .unwrap()
}

pub fn query_token(token: &str) -> bool {

    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let auth_split_whitespace_vec = token.split_ascii_whitespace().collect::<Vec<&str>>();


    let mut check_empty_statement = connection
        .prepare(
            format!("SELECT EXISTS(SELECT token FROM tokentable WHERE token='{}' LIMIT 1);", auth_split_whitespace_vec[1])
        )
            .unwrap();

    check_empty_statement.next().unwrap();
    let output: u64 = check_empty_statement.read::<i64>(0).unwrap().try_into().unwrap();

    output!=0
     
}

pub fn update_logindata(username: &str, password: &str){
    
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let encrypted_password = security::encrypt(password.to_string(), security::padding_convert(DECRYPT_KEY));
    
    connection
        .execute(
format!("
UPDATE logindata SET value = '{}' WHERE variable = 'username';
UPDATE logindata SET value = '{}' WHERE variable = 'password';
", &username, &encrypted_password)
        )
        .unwrap()
}

pub fn query_logindata() -> (String, String){
    
    let mut username: String = String::new();
    let mut password: String = String::new();
    
    let connection = sqlite::open("/tmp/lcs.db").unwrap();

    let mut statement = connection
        .prepare("SELECT value FROM logindata")
        .unwrap();

    let mut increment: u8 = 0;

    while let State::Row = statement.next().unwrap() {
        if increment == 0 {
            username = statement.read::<String>(0).unwrap();
        }
        else if increment == 1 {
            password = statement.read::<String>(0).unwrap();
        }
        increment = increment + 1;
    };

    let decrypted_password = security::decrypt(password, security::padding_convert(DECRYPT_KEY));

    (username, decrypted_password)
}