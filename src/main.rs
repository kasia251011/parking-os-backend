use axum::{
    routing::{get, post},
    Router,
};
use dotenv::dotenv;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod users;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Enable tracing https://tokio.rs/#tk-lib-tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();
    let config = utils::load_config()?;
    let client = utils::setup_mongodb(&config.mongodb_host).await?;
    utils::check_mongodb_connection(&client).await?;

    let app = Router::new().route("/", get(users::root)).route("/users", post(users::create_user));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
