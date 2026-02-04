use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

// jwt payload containing claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

// createse and signs jwt token for user
pub fn create_jwt(username: &str, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // set expiry date using current time
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600;

    // initialise Claims instance
    let claims = Claims {
        sub: username.to_string(),
        exp: expiration as usize,
    };

    // encode and sign token
    jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
