use serde::{Deserialize, Serialize};

use super::model::{CostOfMaintenance, Location};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name: String,
    pub surname: String,
    pub account_balance: f64,
    pub blocked: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateParkingSchema {
    pub cost_of_maintenance: CostOfMaintenance,
    pub location: Location,
}