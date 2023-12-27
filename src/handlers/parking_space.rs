use std::sync::Arc;

use axum::{
    extract::{Path, State, Query}, 
    Json, http::StatusCode, response::IntoResponse
};

use crate::{
    structs::{error::MyError, query::QueryParkingSpaceCode}, 
    AppState
};

pub async fn get_parking_spaces_by_parking_lot_id(
    Path(parking_lot_id): Path<String>,
    Query(QueryParkingSpaceCode { level }): Query<QueryParkingSpaceCode>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .get_parking_spaces_by_parking_lot_id(&parking_lot_id, level)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}