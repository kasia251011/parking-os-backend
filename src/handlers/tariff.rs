use std::sync::Arc;

use axum::{extract::{State, Path}, response::IntoResponse, http::StatusCode, Json};

use crate::{AppState, structs::{error::MyError, schema::CreateTariffSchema}};


pub async fn get_tariffs(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .fetch_tariffs()
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_tariff(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateTariffSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state.db.create_tariff(&body).await.map_err(MyError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}

pub async fn get_tariffs_by_parking_lot_id(
    Path(parking_lot_id): Path<String>,
    State(app_state): State<Arc<AppState>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .get_tariffs_by_parking_lot_id_ascending(&parking_lot_id)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}