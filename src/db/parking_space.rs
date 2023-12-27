use std::str::FromStr;

use bson::{oid::ObjectId, doc};
use chrono::{Datelike, DateTime, TimeZone, Utc};
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::{ParkingSpace, VehicleType, Ticket}, 
    schema::CreateParkingSpaceSchema, response::{ParkingSpaceResponse, IncomeStatsResponse, IncomeStats},
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_parking_spaces(&self) -> Result<Vec<ParkingSpace>> {
        let mut cursor = self
            .parking_space_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<ParkingSpace> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(doc.unwrap());
        }

        Ok(json_result)
    }

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

    pub async fn get_new_parking_space_by_license_number(&self, licence_number: &str, parking_lot_id: &str) -> Result<ParkingSpace> {
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

    pub async fn toggle_occupied_parking_space(&self, parking_space_id: &str) -> Result<String> {
        let oid = ObjectId::from_str(parking_space_id).map_err(|_| InvalidIDError(parking_space_id.to_owned()))?;

        let parking_space = match self
            .parking_space_collection
            .find_one(
                doc! {
                    "_id": oid,
                },
                None,
            )
            .await
            .map_err(MongoQueryError)? 
        {
            Some(parking_space) => Some(parking_space),
            None => return Err(NotFoundError(format!("parking_space with id: {}", parking_space_id))),
        };

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

    pub async fn get_parking_space_by_location(&self, parking_lot_id: &str, spot_name: &str, level: u32) -> Result<ParkingSpace> {
        let parking_space = self
            .fetch_parking_spaces()
            .await?
            .into_iter()
            .find(|parking_space| {
                parking_space.parking_lot_id == ObjectId::from_str(parking_lot_id).map_err(|_| InvalidIDError(parking_lot_id.to_owned())).unwrap()
                    && parking_space.location.no_space.to_string() == spot_name
                    && parking_space.location.no_level == level
            });

        println!("parking_space: {:?}", parking_space);

        match parking_space {
            Some(parking_space) => Ok(parking_space),
            None => Err(NotFoundError(format!("parking_space with spot_name: {}", spot_name))),
        }
    }

    pub async fn get_parking_spaces_by_parking_lot_id(&self, parking_lot_id: &str, level: u32) -> Result<Vec<ParkingSpaceResponse>> {
        let mut cursor = self
            .parking_space_collection
            .find(
                doc! {
                    "parking_lot_id": ObjectId::from_str(parking_lot_id).map_err(|_| InvalidIDError(parking_lot_id.to_owned()))?,
                    "location.no_level": level,
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<ParkingSpaceResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            let parking_space: ParkingSpace = doc.unwrap();
            json_result.push(ParkingSpaceResponse {
                id: parking_space._id.to_hex(),
                parking_lot_id: parking_space.parking_lot_id.to_hex(),
                level: parking_space.location.no_level,
                ordinal_number: parking_space.location.no_space,
                vehicle_type: match parking_space.vehicle_type {
                    VehicleType::Car => "Car".to_string(),
                    VehicleType::Truck => "Truck".to_string(),
                },
                is_occupied: parking_space.occupied,
            });
        }

        Ok(json_result)
    }

    pub async fn get_parking_space_income(&self, parking_lot_id: &str, parking_space_id: &str) -> Result<IncomeStatsResponse> {
        let parking_space = self
            .fetch_parking_spaces()
            .await?
            .into_iter()
            .find(|parking_space| {
                parking_space.parking_lot_id == ObjectId::from_str(parking_lot_id).map_err(|_| InvalidIDError(parking_lot_id.to_owned())).unwrap()
                    && parking_space._id == ObjectId::from_str(parking_space_id).map_err(|_| InvalidIDError(parking_space_id.to_owned())).unwrap()
            });

        let parking_space = match parking_space {
            Some(parking_space) => parking_space,
            None => return Err(NotFoundError(format!("parking_space with id: {}", parking_space_id))),
        };

        let mut cursor = self
            .ticket_collection
            .find(
                doc! {
                    "parking_lot_id": parking_lot_id.to_owned(),
                    "spot_name": parking_space.location.no_space.to_string(),
                    "level": parking_space.location.no_level,
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

        println!("parking_space: {:?}", parking_space);
        println!("cursor: {:?}", cursor.next().await);

        let mut json_result: Vec<IncomeStats> = Vec::new();
        while let Some(doc) = cursor.next().await {
            let ticket: Ticket = doc.unwrap();
            let issue_timestamp = chrono::NaiveDateTime::from_timestamp_opt(ticket.issue_timestamp, 0).unwrap();
            let date_time = Utc.from_utc_datetime(&issue_timestamp).month();
            let month = match date_time {
                1 => "January",
                2 => "February",
                3 => "March",
                4 => "April",
                5 => "May",
                6 => "June",
                7 => "July",
                8 => "August",
                9 => "September",
                10 => "October",
                11 => "November",
                _ => "December",
            }.to_string();

            let income = ticket.amount_paid;
            if let Some(stats) = json_result.iter_mut().find(|stats| stats.month == month) {
                stats.income += income;
            } else {
                json_result.push(IncomeStats {
                    month,
                    income,
                });
            }
        }
        println!("json_result: {:?}", json_result);

        let mut today_income = 0.0;
        let mut now_income = 0.0;
        let today_date_time = Utc::now();  
        while let Some(doc) = cursor.next().await {
            let ticket: Ticket = doc.unwrap();
            let end_timestamp = DateTime::<Utc>::from_utc(chrono::NaiveDateTime::from_timestamp_opt(ticket.issue_timestamp, 0).unwrap(), Utc);
            let diff = today_date_time.signed_duration_since(end_timestamp);
            let days = diff.num_days();

            if days == 0 && days/30 == 0 && days/365 == 0 {
                let amount_paid = ticket.amount_paid;
                if ticket.end_timestamp == 0 {
                    now_income += amount_paid;
                }

                today_income += amount_paid;
            }
        }

        Ok(IncomeStatsResponse {
            stats: json_result,
            today: today_income,
            now: now_income,
        })
    }
}