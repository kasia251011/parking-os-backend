use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CreateUser {
    pub username: String,
}

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub username: String,
}

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn create_user(Json(payload): Json<CreateUser>) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::StatusCode;

    #[tokio::test]
    async fn test_root() {
        let response = root().await;
        assert_eq!(response, "Hello, World!");
    }

    #[tokio::test]
    async fn test_create_user() {
        let payload = CreateUser {
            username: "test_user".to_string(),
        };

        let (status, Json(user)) = create_user(Json(payload)).await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(user.id, 1337);
        assert_eq!(user.username, "test_user");
    }
}