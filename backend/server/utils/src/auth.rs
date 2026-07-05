use chrono::{TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::prelude::CONFIG;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
}

pub fn create_token(username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = Utc::now();
    let expire = now + TimeDelta::hours(CONFIG.jwt.expire_hours);
    let claims = Claims {
        sub: username.to_string(),
        exp: expire.timestamp(),
        iat: now.timestamp(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
    )
}

pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(CONFIG.jwt.secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
