use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParkingLotCode {
    pub code: String,
}