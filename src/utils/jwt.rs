use crate::config;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::Serialize;

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub fn create_token(user_id: String) -> String {
    let claims = Claims {
        sub: user_id,
        exp: 200000000,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config::jwt_secret().as_ref()),
    )
    .unwrap()
}
