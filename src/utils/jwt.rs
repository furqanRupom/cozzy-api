use crate::config;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::Error};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
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

pub fn decode_token(token: &str) -> Result<Claims, Error> {
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);

    match decode::<Claims>(
        token,
        &DecodingKey::from_secret(config::jwt_secret().as_ref()),
        &validation,
    ) {
        Ok(token_data) => {
            // tokens data contains .headers and .claims
            // we only need to return the claims
            Ok(token_data.claims)
        }
        Err(err) => Err(err),
    }
}
