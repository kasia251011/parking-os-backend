use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::structs::model::Role;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub user: User,
    pub exp: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: Role,
}

pub fn create_token(user_id: &str, user: User) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::seconds(180))
        .expect("valid timestamp")
        .timestamp();

    let claims = Claims {
        sub: user_id.to_owned(),
        user: user,
        exp: expiration as usize,
    };

    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    token
}