use axum::{
    routing::{get, post}, 
    http::StatusCode,
    Router, Json
};
use mongodb::{
    bson::doc, 
    options::{ClientOptions, ServerApi, ServerApiVersion}, 
    Client
};
use serde::{Deserialize, Serialize};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    // Enable tracing https://tokio.rs/#tk-lib-tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into())
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO move connection to mongodb to other file
    dotenv().ok();
    let config: Config = envy::from_env().unwrap();
    let mongo_host = config.mongodb_host;

    let mut client_options = ClientOptions::parse(mongo_host).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    let client = Client::with_options(client_options)?;
    let database = client.database("parking-os");
    database.run_command(doc! {"ping": 1}, None).await?;

    println!("Pinged your deployment. You successfully connected to MongoDB!");

    let app = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::debug!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn create_user(
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    let user = User {
        id: 1337,
        username: payload.username,
    };

    (StatusCode::CREATED, Json(user))
}

#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Debug, Deserialize)]
struct Config {
    mongodb_host: String,
}