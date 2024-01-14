use std::sync::Arc;

use axum::{response::IntoResponse, http::StatusCode, extract::State, Json};

use crate::AppState;
use crate::structs::error::MyError;
use crate::structs::schema::{CreateUserSchema, RegisterUserSchema};

pub async fn get_users(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .fetch_users()
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state.db.create_user(&body).await.map_err(MyError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}

pub async fn register_user(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state.db.register_user(&body).await.map_err(MyError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}