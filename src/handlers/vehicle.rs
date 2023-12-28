use std::sync::Arc;

use axum::extract::Path;
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