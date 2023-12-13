use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mongodb_host: String,
}

pub fn load_config() -> Result<Config, envy::Error> {
    envy::from_env()
}

pub async fn setup_mongodb(mongo_host: &str) -> mongodb::error::Result<Client> {
    let mut client_options = ClientOptions::parse(mongo_host).await?;
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub async fn check_mongodb_connection(client: &Client) -> mongodb::error::Result<()> {
    let database = client.database("parking-os");
    database.run_command(doc! {"ping": 1}, None).await?;
    println!("Pinged your deployment. You successfully connected to MongoDB!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mongodb_connection() {
        dotenv::from_filename(".env").ok();

        let config = load_config().expect("Failed to load config");

        let client = setup_mongodb(&config.mongodb_host).await.expect("Failed to connect to MongoDB");

        let result = check_mongodb_connection(&client).await;
        assert!(result.is_ok(), "Failed to ping MongoDB: {:?}", result);
    }
}