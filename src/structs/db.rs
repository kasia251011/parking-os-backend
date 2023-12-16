use std::time::Duration;
use bson::oid::ObjectId;
use futures::StreamExt;
use mongodb::{options::{Compressor, ClientOptions}, Collection, Client};

use super::{
    error::MyError::{*, self}, 
    model::{ParkingLot, Ticket, User, Vehicle}, 
    response::{UserListResponse, UserResponse, GenericResponse}, 
    schema::CreateUserSchema
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

    pub async fn fetch_users(&self) -> Result<UserListResponse> {
        let mut cursor = self
            .user_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<UserResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_user(&doc.unwrap())?);
        }
    
        Ok(UserListResponse {
            status: "200",
            users: json_result,
        })
    }

    pub async fn create_user(&self, body: &CreateUserSchema) -> Result<GenericResponse> {
        let user = User {
            _id: ObjectId::new(),
            name: body.name.to_owned(),
            surname: body.surname.to_owned(),
            account_balance: body.account_balance,
            blocked: body.blocked,
        };

        match self.user_collection.insert_one(user, None).await {
            Ok(result) => result,
            Err(e) => {
                if e.to_string()
                    .contains("E110000 duplicate key error collection")
                {
                    return Err(MongoDuplicateError(e));
                }
                return Err(MongoQueryError(e));
            }
        };

        Ok(GenericResponse {
            status: "200".to_string(),
            message: "Successful operation".to_string(),
        })
    }
    
    fn doc_to_user(&self, user: &User) -> Result<UserResponse> {
        let user_response = UserResponse {
            id: user._id.to_hex(),
            name: user.name.to_owned(),
            surname: user.surname.to_owned(),
            account_balance: user.account_balance.to_owned(),
            blocked: user.blocked.to_owned(),
        };

        Ok(user_response)
    }
}