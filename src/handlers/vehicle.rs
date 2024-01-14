use std::sync::Arc;

use axum::body::Body;
use axum::extract::Path;
use axum::http::{Request, HeaderMap};
use axum::{response::IntoResponse, http::StatusCode, extract::State, Json};

use crate::AppState;
use crate::structs::{
    error::MyError,
    schema::*,
};

pub async fn get_vehicles(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .fetch_vehicles()
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_vehicle(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateVehicleSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state.db.create_vehicle(&body).await.map_err(MyError::from) {
        Ok(_) => Ok((StatusCode::CREATED, "successful operation")),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}

pub async fn get_vehicle_by_license_plate_number(
    Path(license_plate_number): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state
        .db
        .get_vehicle_by_license_plate_number(&license_plate_number)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err((StatusCode::NOT_FOUND, "Vehicle not found".to_string())),
    }
}

pub async fn get_user_vehicles(
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
        .fetch_user_vehicles(&user_id)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err((StatusCode::NOT_FOUND, "Vehicle not found".to_string())),
    }
}

pub async fn create_user_vehicle(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateVehicleUserSchema>,
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

    match app_state.db.create_user_vehicle(&user_id, &body).await.map_err(MyError::from) {
        Ok(_) => Ok((StatusCode::CREATED, "successful operation")),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}