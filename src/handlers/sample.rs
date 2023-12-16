use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::structs::sample::{CreateUser, User};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> impl IntoResponse {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON response
    (StatusCode::CREATED, Json(user))
}