use std::time::Duration;
use mongodb::{options::{Compressor, ClientOptions}, Collection, Client};

use crate::structs::{
    error::MyError::{*, self}, 
    model::{ParkingLot, Ticket, User, Vehicle}, 
};

#[derive(Clone, Debug)]
pub struct DB {
    pub user_collection:            Collection<User>,
    pub ticket_collection:          Collection<Ticket>,
    pub parking_lot_collection:     Collection<ParkingLot>,
    pub vehicle_collection:         Collection<Vehicle>,
}

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn new() -> Result<Self> {
        let mongo_uri: String = std::env::var("MONGO_URI")
            .expect("Failed to load `MONGO_URI` environment variable.");

        let mongo_connection_timeout: u64 = std::env::var("MONGO_CONNECTION_TIMEOUT")
            .expect("Failed to load `MONGO_CONNECTION_TIMEOUT` environment variable.")
            .parse()
            .expect("Failed to parse `MOGNO_CONNECTION_TIMEOUT` environment variable.");

        let mongo_min_pool_size: u32 = std::env::var("MONGO_MIN_POOL_SIZE")
            .expect("Failed to load `MONGO_MIN_POLL_SIZE` environment variable.")
            .parse()
            .expect("Failed to parse `MONGO_MIN_POOL_SIZE` environment variable.");

        let mongo_max_pool_size: u32 = std::env::var("MONGO_MAX_POOL_SIZE")
            .expect("Failed to load `MONGO_MAX_POOL_SIZE` environment variable.")
            .parse()
            .expect("Failed to parse `MONOG_MAX_POOL_SIZE` environment variable.");

        let compressors = Some(vec![
            Compressor::Snappy,
            Compressor::Zlib { level: Default::default(), },
            Compressor::Zstd { level: Default::default(), },
        ]);

        let mut client_options = ClientOptions::parse(mongo_uri).await.unwrap();
        client_options.connect_timeout = Some(Duration::from_secs(mongo_connection_timeout));
        client_options.max_pool_size = Some(mongo_max_pool_size);
        client_options.min_pool_size = Some(mongo_min_pool_size);
        // the server will select the algorithm it supports from the list provided by the driver
        client_options.compressors = compressors;
    
        let client = Client::with_options(client_options).unwrap();
        let database_name = std::env::var("MONGO_DB_NAME")
            .expect("Failed to load `MONGO_DB_NAME` environment variable.");
        let database = client.database(&database_name);

        let user_collection: Collection<User> = database.collection("user");
        let ticket_collection: Collection<Ticket> = database.collection("ticket");
        let parking_lot_collection: Collection<ParkingLot> = database.collection("parking_lot");
        let vehicle_collection: Collection<Vehicle> = database.collection("vehicle");

        println!("Database connected successfully");

        Ok(Self {
            user_collection,
            ticket_collection,
            parking_lot_collection,
            vehicle_collection,
        })
    }
}