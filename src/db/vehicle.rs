use bson::oid::ObjectId;
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::{VehicleType, Vehicle},
    response::VehicleResponse, schema::CreateVehicleSchema,  
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_vehicles(&self) -> Result<Vec<VehicleResponse>> {
        let mut cursor = self
            .vehicle_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<VehicleResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_vehicle(&doc.unwrap())?);
        }
    
        Ok(json_result)
    }

    pub async fn create_vehicle(&self, body: &CreateVehicleSchema) -> Result<String> {
        let new_vehicle_id = ObjectId::new();
        let vehicle = Vehicle {
            _id: new_vehicle_id,
            vehicle_type: match body.vehicle_type.as_str() {
                "Car" => VehicleType::Car,
                "Truck" => VehicleType::Truck,
                _ => return Err(InvalidVehicleTypeError(body.vehicle_type.to_owned())),
            },
            user_id: body.user_id.to_owned(),
            brand: body.brand.to_owned(),
            model: body.model.to_owned(),
            licence_plate_number: body.licence_plate_number.to_owned(),
        };

        match self.vehicle_collection.insert_one(vehicle, None).await {
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

        Ok(new_vehicle_id.to_hex())
    }

    fn doc_to_vehicle(&self, vehicle: &Vehicle) -> Result<VehicleResponse> {
        Ok(VehicleResponse {
            user_id: vehicle.user_id.to_owned(),
            vehicle_type: match vehicle.vehicle_type.to_owned() {
                VehicleType::Car => "Car".to_owned(),
                VehicleType::Truck => "Truck".to_owned(),
            },
            brand: vehicle.brand.to_owned(),
            model: vehicle.model.to_owned(),
            licence_plate_number: vehicle.licence_plate_number.to_owned(),
        })
    }
}