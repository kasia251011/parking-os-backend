use bson::oid::ObjectId;
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::ParkingLot,
    response::{GenericResponse, ParkingLotResponse, ParkingLotListResponse}, 
    schema::CreateParkingSchema
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_parkings(&self) -> Result<ParkingLotListResponse> {
        let mut cursor = self
            .parking_lot_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<ParkingLotResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_parking(&doc.unwrap())?);
        }
    
        Ok(ParkingLotListResponse {
            status: "200",
            parkings: json_result,
        })
    }

    pub async fn create_parking(&self, body: &CreateParkingSchema) -> Result<GenericResponse> {
        let parking = ParkingLot {
            _id: ObjectId::new(),
            cost_of_maintenance: body.cost_of_maintenance.to_owned(),
            location: body.location.to_owned(),
            levels: body.levels.to_owned(),
        };

        match self.parking_lot_collection.insert_one(parking, None).await {
            Ok(result) => result,
            Err(e) => {
                if e.to_string()
                    .contains("E110000 duplicate key error collection")
                {
                    return Err(MongoDuplicateError(e));
                }
                return Err(MongoQueryError(e));
            }
        };

        Ok(GenericResponse {
            status: "200".to_string(),
            message: "Successful operation".to_string(),
        })
    }
    
    fn doc_to_parking(&self, parking: &ParkingLot) -> Result<ParkingLotResponse> {
        let parking_response = ParkingLotResponse {
            id: parking._id.to_hex(),
            cost_of_maintance: parking.cost_of_maintenance.to_owned(),
            location: parking.location.to_owned(),
            levels: parking.levels.to_owned(),
        };

        Ok(parking_response)
    }
}