use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParkingLotCode {
    pub code: String,
}

#[derive(Deserialize)]
pub struct QueryTicketCode {
    pub code: String,
}

#[derive(Deserialize)]
pub struct QueryTicket {
    #[serde(rename = "userId", default = "String::new")]
    pub user_id: String,
    #[serde(default)]
    pub active: bool,
    #[serde(rename = "parkingLotId", default = "String::new")]
    pub vehicle_license_number: String,
    #[serde(rename = "vehicleLicenseNumber", default = "String::new")]
    pub parking_spot_id: String,
    #[serde(rename = "issueTimeStamp", default)]
    pub issue_time_stamp: u32,
    #[serde(rename = "endTimeStamp", default)]
    pub end_time_stamp: u32,
    #[serde(default)]
    pub level: u32,
    #[serde(rename = "parkingLotId", default = "String::new")]
    pub parking_lot_id: String,
}

#[derive(Deserialize)]
pub struct QueryParkingSpaceCode {
    #[serde(default = "default_level")]
    pub level: i32,
}

fn default_level() -> i32 {
    -1
}