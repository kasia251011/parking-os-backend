use serde::Serialize;

use super::model::{CostOfMaintenance, Location, Levels};

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub account_balance: f64,
    pub blocked: bool,
}

#[derive(Serialize, Debug)]
pub struct UserListResponse {
    pub status: &'static str,
    pub users: Vec<UserResponse>,
}

#[derive(Serialize, Debug)]
pub struct ParkingLotResponse {
    pub id: String,
    pub cost_of_maintance: CostOfMaintenance,
    pub location: Location,
    pub levels: Vec<Levels>,
}

#[derive(Serialize, Debug)]
pub struct ParkingLotListResponse {
    pub status: &'static str,
    pub parkings: Vec<ParkingLotResponse>,
}