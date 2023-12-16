mod structs;
mod handlers;

use std::time::Duration;
use axum::{
    http::{header, HeaderValue},
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use mongodb::{
    bson::doc,
    options::ClientOptions,
    Client,
};
use tower_http::{
    limit::RequestBodyLimitLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
    timeout::TimeoutLayer
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use handlers::{
    common::handler_404,
    sample::{create_user, root},
};
use structs::common::DatabaseConfig;

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

    let app = app().await;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

pub async fn app() -> Router {
    let app = Router::new()
        .route("/sample/", get(root))
        .route("/sample/users/", post(create_user))
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // don't allow request bodies larger than 1024 bytes, returning 413 status code
        .layer(RequestBodyLimitLayer::new(1024))
        .layer(TraceLayer::new_for_http())
        .layer(SetResponseHeaderLayer::if_not_present(
            header::SERVER, 
            HeaderValue::from_static("rust-axum"),
    ));

    let client = setup_mongodb().await;
    app.fallback(handler_404).with_state(client)
}

pub async fn setup_mongodb() -> Client {
    let database_config = DatabaseConfig::new();
    let mut client_options = ClientOptions::parse(database_config.uri).await.unwrap();
    client_options.connect_timeout = database_config.connection_timeout;
    client_options.max_pool_size = database_config.max_pool_size;
    client_options.min_pool_size = database_config.min_pool_size;
    // the server will select the algorithm it supports from the list provided by the driver
    client_options.compressors = database_config.compressors;

    Client::with_options(client_options).unwrap()
}

pub async fn check_mongodb_connection(client: &Client) -> mongodb::error::Result<()> {
    let database = client.database("parking-os");
    database.run_command(doc! {"ping": 1}, None).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
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
    async fn test_mongodb_connection() {
        dotenv().ok();
        let client = setup_mongodb().await;

        let result = check_mongodb_connection(&client).await;
        assert!(result.is_ok(), "Failed to ping MongoDB: {:?}", result);
    }

    #[tokio::test]
    async fn hello_world() {
        dotenv().ok();
        let app = app().await;

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
        let app = app().await;

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
        let app = app().await;

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
        let mut app = app().await.into_service();

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