use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use warp::reject::Reject;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct CustomError;
impl Reject for CustomError {}

pub fn create_jwt(username: &str) -> Result<String, CustomError> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(1))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref())).map_err(|_| CustomError)
}

pub fn validate_jwt(token: &str) -> Result<Claims, CustomError> {
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");
    decode::<Claims>(&token, &DecodingKey::from_secret(secret.as_ref()), &Validation::default())
        .map(|data| data.claims)
        .map_err(|_| CustomError)
}
