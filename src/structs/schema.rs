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