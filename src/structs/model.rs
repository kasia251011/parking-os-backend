use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub surname: String,
    pub account_balance: f64,
    pub blocked: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    pub _id: ObjectId,
    pub user_id: String,
    pub vehicle_license_number: String,
    pub parking_space_id: String,
    pub issue_timestamp: i64,
    pub end_timestamp: i64,
    pub amount_paid: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParkingLot {
    pub _id: ObjectId,
    pub cost_of_maintenance: CostOfMaintenance,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CostOfMaintenance {
    pub electricity: f64,
    pub cleaning: f64,
    pub security: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub city: String,
    pub address: String,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vehicle {
    pub _id: ObjectId,
    pub user_id: String,
    pub r#type: String,
    pub brand: String,
    pub model: String,
    pub licence_plate_number: String,
}