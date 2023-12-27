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
}

#[derive(Deserialize)]
pub struct QueryParkingSpaceCode {
    pub level: u32,
}