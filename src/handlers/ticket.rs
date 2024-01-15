use std::sync::Arc;

use axum::extract::{Query, Path};
use axum::http::HeaderMap;
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
        Err(e) => Err((StatusCode::BAD_REQUEST, "Invalid input ".to_string() + &e.to_string())),
    }
}

pub async fn get_user_active_tickets(
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
        Err(e) => {
            println!("{}", e);
            return Err((StatusCode::BAD_REQUEST, "Invalid token".to_string()))
        },
    };

    match app_state
        .db
        .get_user_active_tickets(&user_id)
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(_) => Err((StatusCode::NOT_FOUND, "Ticket not found".to_string())),
    }
}

pub async fn create_user_ticket(
    headers: HeaderMap,
    State(app_state): State<Arc<AppState>>,
    Json(body): Json<CreateTicketUserSchema>,
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

    match app_state.db.create_user_ticket(&user_id, &body).await.map_err(MyError::from) {
        Ok(res) => Ok((StatusCode::CREATED, Json(res))),
        Err(_) => Err((StatusCode::BAD_REQUEST, "Invalid input".to_string())),
    }
}