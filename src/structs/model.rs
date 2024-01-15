use serde::{Deserialize, Serialize};
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Role {
    Admin,
    User,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub _id: ObjectId,
    pub name: String,
    pub surname: String,
    pub email: String,
    pub password: String,
    pub account_balance: f64,
    pub role: Role,
    pub blocked: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Ticket {
    pub _id: ObjectId,
    pub user_id: String,
    pub vehicle_license_number: String,
    pub parking_spot_id: String,
    pub issue_timestamp: i64,
    pub end_timestamp: i64,
    pub amount_paid: f64,
    pub level: u32,
    pub spot_ordinal_number: u32,
    pub parking_lot_id: String,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParkingLot {
    pub _id: ObjectId,
    pub cost_of_maintenance: CostOfMaintenance,
    pub location: Location,
    pub no_levels: u32,
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
pub struct Levels {
    pub cars: u32,
    pub trucks: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vehicle {
    pub _id: ObjectId,
    pub user_id: String,
    #[serde(rename = "type")]
    pub vehicle_type: VehicleType,
    pub brand: String,
    pub model: String,
    pub license_plate_number: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParkingSpace {
    pub _id: ObjectId,
    pub parking_lot_id: ObjectId,
    pub location: ParkingLocation,
    pub vehicle_type: VehicleType,
    pub occupied: bool,
    pub price_modifier: f64, // default 1.0
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum VehicleType {
    Car,
    Truck,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ParkingLocation {
    pub no_level: u32,
    pub no_space: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tariff {
    pub _id: ObjectId,
    pub parking_lot_id: String,
    pub min_time: i64,
    pub max_time: i64,
    pub price_per_hour: f64,
}