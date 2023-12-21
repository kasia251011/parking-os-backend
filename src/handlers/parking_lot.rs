use std::sync::Arc;

use axum::extract::{Path, Query};
use axum::{response::IntoResponse, http::StatusCode, extract::State, Json};

use crate::AppState;
use crate::structs::{
    error::MyError,
    schema::*,
    query::QueryParkingLotCode,
};

pub async fn get_parkings(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .fetch_parkings()
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn get_parking(
    Path(parking_lot_id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .get_parking_lot_by_id(&parking_lot_id)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_parking(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateParkingSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state.db.create_parking(&body).await.map_err(MyError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}

pub async fn generate_parking_lot_code(
    Path(parking_lot_id): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)>
{
    match app_state
        .db
        .get_parking_lot_by_id(&parking_lot_id)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => {
            let res: String = res.id.chars().take(8).collect();
            Ok((StatusCode::CREATED, Json(res)))
        },
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}

pub async fn get_parking_by_code(
    Query(QueryParkingLotCode { code }): Query<QueryParkingLotCode>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)>
{
    match app_state
        .db
        .get_parking_lot_by_code(&code)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}