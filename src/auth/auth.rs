use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, DecodingKey, EncodingKey};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub fn generate_jwt(username: &str) -> Result<String, String> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
        .map_err(|e| format!("Error generating JWT: {:?}", e))
}

pub fn validate_jwt(token: &str) -> Result<Claims, Box<dyn std::error::Error>> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
