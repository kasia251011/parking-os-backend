use std::str::FromStr;

use bson::{oid::ObjectId, doc};
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::ParkingSpace, 
    schema::CreateParkingSpaceSchema,
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    // pub async fn fetch_parking_spaces(&self) -> Result<Vec<ParkingSpace>> {
    //     let mut cursor = self
    //         .parking_space_collection
    //         .find(None, None)
    //         .await
    //         .map_err(MongoQueryError)?;

    //     let mut json_result: Vec<ParkingSpace> = Vec::new();
    //     while let Some(doc) = cursor.next().await {
    //         json_result.push(doc.unwrap());
    //     }

    //     Ok(json_result)
    // }

    pub async fn create_parking_space(&self, parking_space: &CreateParkingSpaceSchema) -> Result<String> {
        let new_parking_space_id = ObjectId::new();
        let parking_space = ParkingSpace {
            _id: new_parking_space_id,
            parking_lot_id: parking_space.parking_lot_id.to_owned(),
            location: parking_space.location.to_owned(),
            vehicle_type: parking_space.vehicle_type.to_owned(),
            occupied: false,
            price_modifier: 1.0,
        };

        match self.parking_space_collection.insert_one(parking_space, None).await {
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

        Ok("Successful operation".to_string())
    }

    pub async fn get_new_parking_space_by_licence_number(&self, licence_number: &str, parking_lot_id: &str) -> Result<ParkingSpace> {
        let vehicle_type: &str = match licence_number.as_bytes()[0] {
            b'A' => "Truck",
            _ => "Car",
        };

        let mut cursor = self
            .parking_space_collection
            .find(
                doc! {
                    "parking_lot_id": ObjectId::from_str(parking_lot_id).map_err(|_| InvalidIDError(parking_lot_id.to_owned()))?,
                    "vehicle_type": vehicle_type,
                    "occupied": false,
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<ParkingSpace> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(doc.unwrap());
            break;
        }

        if json_result.len() == 0 {
            return Err(NoParkingSpaceError(licence_number.to_owned()));
        }

        self.toggle_occupied_parking_space(&json_result[0]._id.to_hex()).await?;

        println!("json_result: {:?}", json_result[0]);

        Ok(json_result[0].to_owned())
    }

    async fn toggle_occupied_parking_space(&self, parking_space_id: &str) -> Result<String> {
        let oid = ObjectId::from_str(parking_space_id).map_err(|_| InvalidIDError(parking_space_id.to_owned()))?;

        let parking_space = self
            .parking_space_collection
            .find_one(
                doc! {
                    "_id": oid,
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        let mut parking_space = parking_space.unwrap();
        parking_space.occupied = !parking_space.occupied;

        self.parking_space_collection
            .update_one(
                doc! {
                    "_id": oid,
                },
                doc! {
                    "$set": {
                        "occupied": parking_space.occupied,
                    },
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        Ok("Successful operation".to_string())
    }

    // pub async fn get_parking_spaces_by_parking_lot_id(&self, parking_lot_id: &str) -> Result<Vec<ParkingSpace>> {
    //     let oid = ObjectId::from_str(parking_lot_id).map_err(|_| InvalidIDError(parking_lot_id.to_owned()))?;

    //     let mut cursor = self
    //         .parking_space_collection
    //         .find(
    //             doc! {
    //                 "parking_lot_id": oid,
    //             },
    //             None,
    //         )
    //         .await
    //         .map_err(MongoQueryError)?;

    //     let mut json_result: Vec<ParkingSpace> = Vec::new();
    //     while let Some(doc) = cursor.next().await {
    //         json_result.push(doc.unwrap());
    //     }

    //     Ok(json_result)
    // }
}