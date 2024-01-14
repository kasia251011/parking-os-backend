use jsonwebtoken::{encode, Algorithm, EncodingKey, Header, DecodingKey};
use serde::{Deserialize, Serialize};

use crate::structs::model::Role;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub user: User,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub surname: String,
    pub email: String,
    pub role: Role,
}

pub fn create_token(user_id: &str, user: User) -> String {
    let claims = Claims {
        sub: user_id.to_owned(),
        user: user,
    };

    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap();

    token
}

pub fn decode_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token = token.replace("Bearer ", "");
    let token_data = jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret("secret".as_ref()),
        &jsonwebtoken::Validation::new(Algorithm::HS512),
    );

    match token_data {
        Ok(data) => Ok(data.claims),
        Err(e) => Err(e),
    }
}