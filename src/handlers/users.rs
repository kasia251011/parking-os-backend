use std::sync::Arc;

use axum::http::HeaderMap;
use axum::{response::IntoResponse, http::StatusCode, extract::State, Json};

use crate::AppState;
use crate::structs::error::MyError;
use crate::structs::schema::{CreateUserSchema, RegisterUserSchema, LoginUserSchema};

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

pub async fn login_user(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state.db.login_user(&body).await.map_err(MyError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(e) => Err((StatusCode::BAD_REQUEST, "Invalid input ".to_string() + &e.to_string())),
    }
}

pub async fn get_user_balance(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    let authorization_header = match headers.get("Authorization") {
        Some(header) => header.to_str().unwrap(),
        None => return Err((StatusCode::BAD_REQUEST, "Invalid header".to_string())),
    };

    let user_id = match crate::utils::jwt::decode_token(authorization_header) {
        Ok(claims) => claims.sub,
        Err(_) => return Err((StatusCode::BAD_REQUEST, "Invalid token".to_string())),
    };


    match app_state
        .db
        .get_user_balance(&user_id)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    }
}