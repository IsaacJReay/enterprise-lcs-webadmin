use crate::{security, DECRYPT_KEY, SESSION_LIMIT, TOKEN_EXPIRATION_SEC};
use actix_web::HttpRequest;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use pam::Authenticator;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    auth: String,
    session_id: String,
    iat: u64,
    exp: u64,
}

pub fn insert_into_auth_table(
    username: &str,
    encrypted_password: &str,
    session_id: &str,
    iat: u64,
) {
    let connection = Connection::open("/tmp/lcs.db").unwrap();

    let mut check_overflowed_statement = connection
        .prepare("SELECT COUNT(UserName) FROM tblAuth WHERE UserName=:name")
        .unwrap();
    let mut rows = check_overflowed_statement
        .query(&[(":name", username)])
        .unwrap();
    if rows.next().unwrap().unwrap().get::<usize, u64>(0).unwrap() > SESSION_LIMIT - 1 {
        // Minus 1 because System Count from Zero and Human Count from One
        connection.execute(
            "DELETE FROM tblAuth WHERE UserName=? AND IAT=(SELECT MIN(IAT) from tblAuth WHERE UserName=?)",
            &[username, username]
        ).unwrap();
    }
    connection
        .execute(
            "INSERT INTO tblAuth VALUES (?, ?, ?, ?)",
            params![username, encrypted_password, session_id, iat],
        )
        .unwrap();
}

pub fn logout(claims: &Claims) -> Result<(), String> {
    let connection = Connection::open("/tmp/lcs.db").unwrap();

    match connection.execute(
        "DELETE FROM tblAuth WHERE UserName=? AND SessionID=? AND IAT=?",
        params![&claims.aud, &claims.session_id, claims.iat],
    ) {
        Ok(_t) => Ok(()),
        Err(message) => Err(message.to_string()),
    }
}

pub fn get_pass_from_tbl(
    username: &str,
    session_id: &str,
    iat: u64,
) -> Result<String, (u32, String)> {
    let connection = Connection::open("/tmp/lcs.db").unwrap();

    let mut check_overflowed_statement = connection
        .prepare("SELECT CryptedPass FROM tblAuth WHERE UserName=? AND SessionID=? AND IAT=?")
        .unwrap();
    let mut rows = check_overflowed_statement
        .query(params![username, session_id, iat])
        .unwrap();

    match rows.next().unwrap() {
        Some(row) => Ok(row.get::<usize, String>(0).unwrap()),
        None => Err((401, "Token invalid".to_string())),
    }
}

pub fn login(username: &str, password: &str) -> Result<String, String> {
    let mut authenticator = Authenticator::with_password("sudo").unwrap();

    authenticator
        .get_handler()
        .set_credentials(username, password);

    match authenticator.authenticate() {
        Ok(()) => Ok(()),
        Err(err) => Err(format!("{}", err)),
    }?;

    let timestamp = jsonwebtoken::get_current_timestamp();

    let rand_charset = format!("{}{}", username, timestamp);
    let current_session_id: String = security::generate_random(48, Some(rand_charset));
    let random_key = security::generate_random(32, None);
    let encrypted_password =
        security::encrypt(password.to_string(), security::padding_convert(&random_key));

    insert_into_auth_table(
        username,
        &encrypted_password,
        &current_session_id,
        timestamp,
    );

    let token = encode(
        &Header::default(),
        &Claims {
            aud: username.to_string(),
            auth: random_key,
            session_id: current_session_id.to_string(),
            iat: timestamp,
            exp: timestamp + TOKEN_EXPIRATION_SEC,
        },
        &EncodingKey::from_secret(DECRYPT_KEY.as_ref()),
    )
    .unwrap();

    Ok(token)
}

pub fn extract_claims_from_token(token: &str) -> Result<Claims, (u32, String)> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_required_spec_claims(&["aud", "auth", "session_id", "iat", "exp"]);
    validation.validate_exp = true;
    let token_message = match decode::<Claims>(
        &token,
        &DecodingKey::from_secret(DECRYPT_KEY.as_ref()),
        &validation,
    ) {
        Ok(token) => Ok(token),
        Err(_err) => Err((410, String::from("Token expired or incorrect"))),
    }?;

    Ok(token_message.claims)
}

pub fn extract_auth_from_claims(claims: &Claims) -> Result<(String, String), (u32, String)> {
    let encrypted_password = get_pass_from_tbl(&claims.aud, &claims.session_id, claims.iat)?;
    let decrypted_password =
        match security::decrypt(encrypted_password, security::padding_convert(&claims.auth)) {
            Ok(encrypted_password) => Ok(encrypted_password),
            Err(message) => Err((401, message)),
        }?;

    Ok((claims.aud.to_owned(), decrypted_password))
}

pub fn validate_token(req: &HttpRequest) -> Result<(String, String), (u32, String)> {
    let token = match req.headers().get("AUTHORIZATION") {
        Some(token) => Ok(token.to_str().unwrap().split_whitespace().last().unwrap()),
        None => Err((410, "Token Missing".to_string())),
    }?;

    let claims = match extract_claims_from_token(&token) {
        Ok(claims) => Ok(claims),
        Err((code, message)) => Err((code, message)),
    }?;
    let (username, password) = match extract_auth_from_claims(&claims) {
        Ok((username, password)) => Ok((username, password)),
        Err((code, message)) => Err((code, message)),
    }?;

    Ok((username, password))
}
