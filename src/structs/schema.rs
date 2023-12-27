use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::model::{CostOfMaintenance, Location, Levels, ParkingLocation, VehicleType};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name: String,
    pub surname: String,
    #[serde(rename = "accountBalance")]
    pub account_balance: f64,
    pub blocked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateParkingSchema {
    #[serde(rename = "costOfMaintenance")]
    pub cost_of_maintenance: CostOfMaintenance,
    pub location: Location,
    pub levels: Vec<Levels>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateParkingSpaceSchema {
    pub parking_lot_id: ObjectId,
    pub location: ParkingLocation,
    pub vehicle_type: VehicleType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateVehicleSchema {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "type")]
    pub vehicle_type: String,
    pub brand: String,
    pub model: String,
    #[serde(rename = "licensePlateNumber")]
    pub license_plate_number: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTicketSchema {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "vehicleLicenseNumber")]
    pub vehicle_license_number: String,
    #[serde(rename = "parkingLotId")]
    pub parking_lot_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateTariffSchema {
    #[serde(rename = "parkingLotId")]
    pub parking_lot_id: String,
    #[serde(rename = "minTime")]
    pub min_time: i64,
    #[serde(rename = "maxTime")]
    pub max_time: i64,
    #[serde(rename = "pricePerHour")]
    pub price_per_hour: f64,
}