use serde::{Deserialize, Serialize};

use super::model::{CostOfMaintenance, Location, Levels};

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