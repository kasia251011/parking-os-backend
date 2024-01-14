mod structs;
mod handlers;
mod db;
mod utils;

use std::{time::Duration, sync::Arc};
use axum::{
    http::{header, HeaderValue, Method,
            header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},},
    routing::{get, post, put},
    Router,
};
use dotenv::dotenv;
use tower_http::{
    limit::RequestBodyLimitLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer,
    cors::CorsLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use handlers::{
    common::handler_404,
    sample::{create_sample_user, root},
    users::{create_user, get_users, register_user, login_user}, 
    parking_lot::{create_parking, get_parkings, get_parking_by_code, generate_parking_lot_code, get_parking, get_parking_lot_levels, get_parking_lot_income},
    vehicle::{create_vehicle, get_vehicles, get_vehicle_by_license_plate_number, get_user_vehicles, create_user_vehicle}, 
    ticket::{get_tickets, create_ticket, put_ticket, get_user_active_tickets, create_user_ticket},
    tariff::get_tariffs_by_parking_lot_id,
    parking_space::{get_parking_spaces_by_parking_lot_id, get_parking_space_income},
};
use db::common::DB;

pub struct AppState {
    db: DB,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| {
                "rust_axum=debug,axum=debug,tower_http=debug,mongodb=debug".into()
            }),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db = DB::new().await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin("http://0.0.0.0:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = app(Arc::new(AppState { db: db.clone() })).await.layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn app(app_state: Arc<AppState>) -> Router {
    let app = Router::new()
        .route("/sample/", get(root))
        .route("/sample/users/", post(create_sample_user))
        .route("/users", get(get_users).post(create_user))
        .route("/user", post(register_user))
        .route("/login", post(login_user))
        .route("/parking-lots", get(get_parkings).post(create_parking))
        .route("/parking-lots/:id/code", get(generate_parking_lot_code))
        .route("/parking-lots/", get(get_parking_by_code))
        .route("/parking-lots/:id", get(get_parking))
        .route("/parking-lots/:id/levels", get(get_parking_lot_levels))
        .route("/parking-lots/:id/tariffs", get(get_tariffs_by_parking_lot_id))
        .route("/parking-lots/:id/income", get(get_parking_lot_income))
        .route("/vehicles", get(get_vehicles).post(create_vehicle))
        .route("/vehicles/:license_plate_number", get(get_vehicle_by_license_plate_number))
        .route("/me/vehicles", get(get_user_vehicles).post(create_user_vehicle))
        .route("/tickets", get(get_tickets).post(create_ticket))
        .route("/tickets/:code", put(put_ticket))
        .route("/me/ticket", get(get_user_active_tickets).post(create_user_ticket))
        .route("/parking-lots/:id/parking-spots", get(get_parking_spaces_by_parking_lot_id))
        .route("/parking-lots/:id/parking-spots/:id/income", get(get_parking_space_income))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // don't allow request bodies larger than 1024 bytes, returning 413 status code
        .layer(RequestBodyLimitLayer::new(1024))
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER, 
            HeaderValue::from_static("rust-axum"),
    ));

    app.fallback(handler_404).with_state(app_state)
}

#[cfg(test)]
mod tests {
    use crate::structs::sample::CreateUser;

    use super::*;
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use http_body_util::BodyExt; // for `collect`
    use serde_json::{json, Value};
    use tower::{Service, ServiceExt}; // for `call`, `oneshot`, and `ready`

    #[tokio::test]
    async fn hello_world() {
        dotenv().ok();

        let db = DB::new().await.unwrap();
        let app = app(Arc::new(AppState { db: db.clone() })).await;

        let response = app
            .oneshot(Request::builder().uri("/sample/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
            
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Hello, World!");
    }

    #[tokio::test]
    async fn json() {
        dotenv().ok();

        let db = DB::new().await.unwrap();
        let app = app(Arc::new(AppState { db: db.clone() })).await;

        let response = app
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .uri("/sample/users/")
                    .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                    .body(Body::from(
                        serde_json::to_string(&CreateUser {
                            username: "test_world".to_string(),
                        }).unwrap(),
                    ))
                    .unwrap(),
            ) 
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body, json!({
            "id": 1337,
            "username": "test_world",
        }));
    }

    #[tokio::test]
    async fn not_found() {
        dotenv().ok();

        let db = DB::new().await.unwrap();
        let app = app(Arc::new(AppState { db: db.clone() })).await;

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/this-endpoint-does-not-exist")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"This request does not exist.");
    }

    #[tokio::test]
    async fn multiple_request() {
        dotenv().ok();

        let db = DB::new().await.unwrap();
        let mut app = app(Arc::new(AppState { db: db.clone() })).await.into_service();

        let request = Request::builder().uri("/sample/").body(Body::empty()).unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);

        let request = Request::builder().uri("/sample/").body(Body::empty()).unwrap();
        let response = ServiceExt::<Request<Body>>::ready(&mut app)
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}