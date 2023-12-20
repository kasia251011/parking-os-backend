use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParkingLotCode {
    pub code: String,
}

#[derive(Deserialize)]
pub struct QueryTicketCode {
    pub code: String,
}