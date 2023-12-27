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

#[derive(Serialize, Debug)]
pub struct VehicleResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "type")]
    pub vehicle_type: String,
    pub brand: String,
    pub model: String,
    #[serde(rename = "licensePlateNumber")]
    pub license_plate_number: String,
}

#[derive(Serialize, Debug)]
pub struct TicketResponse {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "vehicleLicenseNumber")]
    pub vehicle_license_number: String,
    #[serde(rename = "issueTimestamp")]
    pub issue_timestamp: i64,
    #[serde(rename = "endTimestamp")]
    pub end_timestamp: i64,
    #[serde(rename = "amountPaid")]
    pub amount_paid: f64,
    #[serde(rename = "spotName")]
    pub spot_name: String,
    pub level: u32,
    #[serde(rename = "parkingLotId")]
    pub parking_lot_id: String,
    pub code: String,
}

#[derive(Serialize, Debug)]
pub struct TariffResponse {
    #[serde(rename = "parkingLotId")]
    pub parking_lot_id: String,
    #[serde(rename = "minTime")]
    pub min_time: i64,
    #[serde(rename = "maxTime")]
    pub max_time: i64,
    #[serde(rename = "pricePerHour")]
    pub price_per_hour: f64,
}