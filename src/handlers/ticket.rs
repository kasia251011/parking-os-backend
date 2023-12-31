use std::sync::Arc;

use axum::extract::{Query, Path};
use axum::{response::IntoResponse, http::StatusCode, extract::State, Json};

use crate::AppState;
use crate::structs::query::QueryTicket;
use crate::structs::{
    error::MyError,
    schema::*,
};

pub async fn get_tickets(
    Query(QueryTicket { user_id, active, vehicle_license_number, parking_spot_id, issue_time_stamp, end_time_stamp, level, parking_lot_id} ): Query<QueryTicket>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .fetch_tickets(&user_id, active, &vehicle_license_number, &parking_spot_id, issue_time_stamp, end_time_stamp, level, &parking_lot_id)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}

pub async fn create_ticket(
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateTicketSchema>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state.db.create_ticket(&body).await.map_err(MyError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}

pub async fn put_ticket(
    Path(code): Path<String>,
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, String)> 
{
    match app_state.db.put_ticket(&code).await.map_err(MyError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}