use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub name: String,
    pub surname: String,
    pub account_balance: f64,
    pub blocked: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub vehicle_license_number: String,
    pub parking_space_id: String,
    pub issue_timestamp: i64,
    pub end_timestamp: i64,
    pub amount_paid: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParkingLot {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub cost_electricity: f64,
    pub cost_cleaning: f64,
    pub cost_security: f64,
    pub city: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vehicle {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub r#type: String,
    pub brand: String,
    pub model: String,
    pub licence_plate_number: String,
}