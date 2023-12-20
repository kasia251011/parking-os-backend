use std::str::FromStr;

use bson::{oid::ObjectId, doc};
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::{ParkingLot, ParkingLocation, VehicleType, ParkingSpace},
    response::ParkingLotResponse, 
    schema::{CreateParkingSchema, CreateParkingSpaceSchema},  
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_parkings(&self) -> Result<Vec<ParkingLotResponse>> {
        let mut cursor = self
            .parking_lot_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<ParkingLotResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_parking(&doc.unwrap())?);
        }
    
        Ok(json_result)
    }

    pub async fn create_parking(&self, body: &CreateParkingSchema) -> Result<String> {
        let new_parking_lot_id = ObjectId::new();
        let parking = ParkingLot {
            _id: new_parking_lot_id,
            cost_of_maintenance: body.cost_of_maintenance.to_owned(),
            location: body.location.to_owned(),
            no_levels: body.levels.len() as u32,
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

        for (idx, level) in body.levels.iter().enumerate() {
            for num in 0..level.cars {
                self.create_parking_space(&CreateParkingSpaceSchema {
                    parking_lot_id: new_parking_lot_id,
                    location: ParkingLocation {
                        no_level: idx as u32,
                        no_space: num,
                    },
                    vehicle_type: VehicleType::Car,
                }).await?;
            }
            for num in 0..level.trucks {
                self.create_parking_space(&CreateParkingSpaceSchema {
                    parking_lot_id: new_parking_lot_id,
                    location: ParkingLocation {
                        no_level: idx as u32,
                        no_space: num,
                    },
                    vehicle_type: VehicleType::Truck,
                }).await?;
            }
        }

        Ok("Successful operation".to_string())
    }
    
    fn doc_to_parking(&self, parking: &ParkingLot) -> Result<ParkingLotResponse> {
        let parking_response = ParkingLotResponse {
            id: parking._id.to_hex(),
            cost_of_maintance: parking.cost_of_maintenance.to_owned(),
            location: parking.location.to_owned(),
            no_levels: parking.no_levels.to_owned(),
        };

        Ok(parking_response)
    }

    pub async fn get_parking_lot_by_id(&self, parking_lot_id: &str) -> Result<ParkingLotResponse> {
        let oid = ObjectId::from_str(parking_lot_id).map_err(|_| InvalidIDError(parking_lot_id.to_owned()))?;

        let parking_lot = self
            .parking_lot_collection
            .find_one(
                doc! {
                    "_id": oid,
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;
        
        match parking_lot {
            Some(doc) => Ok(self.doc_to_parking(&doc)?),
            None => Err(NotFoundError(parking_lot_id.to_string()))
        }
    }

    pub async fn get_parking_spaces_by_parking_lot_id(&self, parking_lot_id: &str) -> Result<Vec<ParkingSpace>> {
        let oid = ObjectId::from_str(parking_lot_id).map_err(|_| InvalidIDError(parking_lot_id.to_owned()))?;

        let mut cursor = self
            .parking_space_collection
            .find(
                doc! {
                    "parking_lot_id": oid,
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<ParkingSpace> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(doc.unwrap());
        }

        Ok(json_result)
    }
}