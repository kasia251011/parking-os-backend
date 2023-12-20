use serde::Serialize;

use super::model::{CostOfMaintenance, Location};

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub surname: String,
    #[serde(rename = "accountBalance")]
    pub account_balance: f64,
    pub blocked: bool,
}

#[derive(Serialize, Debug)]
pub struct ParkingLotResponse {
    pub id: String,
    #[serde(rename = "costOfMaintenance")]
    pub cost_of_maintance: CostOfMaintenance,
    pub location: Location,
    pub no_levels: u32,
}