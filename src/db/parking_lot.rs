use std::str::FromStr;

use bson::{oid::ObjectId, doc};
use chrono::{Utc, TimeZone, Datelike};
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::{ParkingLot, ParkingLocation, VehicleType, Ticket},
    response::{ParkingLotResponse, ParkingLotStatsResponse, ParkingLotStats, IncomeStats}, 
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

        let mut spot_name: u32 = 0;
        for (idx, level) in body.levels.iter().enumerate() {
            for _ in 0..level.cars {
                self.create_parking_space(&CreateParkingSpaceSchema {
                    parking_lot_id: new_parking_lot_id,
                    location: ParkingLocation {
                        no_level: idx as u32,
                        no_space: spot_name,
                    },
                    vehicle_type: VehicleType::Car,
                }).await?;
                spot_name += 1;
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
                spot_name += 1;
            }
        }

        println!("body.tariffs: {:?}", body.tariffs);

        for tariff in &body.tariffs {
            self.create_tariff(tariff, &new_parking_lot_id.to_hex()).await?;
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

        println!("parking_lot: {:?}", parking_lot);
        
        match parking_lot {
            Some(doc) => Ok(self.doc_to_parking(&doc)?),
            None => Err(NotFoundError(parking_lot_id.to_string()))
        }
    }

    pub async fn get_parking_lot_levels_by_id(&self, parking_lot_id: &str) -> Result<Vec<ParkingLotStatsResponse>> {
        let parking_spaces = self
            .fetch_parking_spaces()
            .await?
            .into_iter()
            .filter(|parking_space| parking_space.parking_lot_id == ObjectId::from_str(parking_lot_id).map_err(|_| InvalidIDError(parking_lot_id.to_owned())).unwrap())
            .collect::<Vec<_>>();

        let mut parking_lot_stats: Vec<ParkingLotStatsResponse> = Vec::new();
        for parking_space in parking_spaces {
            let index = parking_space.location.no_level as usize;
            let occupied = parking_space.occupied;
            match parking_lot_stats.get_mut(index) {
                Some(parking_lot_stat) => 
                    self.add_new_stat_occupied_space(parking_lot_stat, parking_space.vehicle_type, occupied),
                None => {
                    let mut parking_lot_stat = ParkingLotStatsResponse {
                        truck: ParkingLotStats {
                            spots_occupied: 0,
                            spots_free: 0,
                        },
                        car: ParkingLotStats {
                            spots_occupied: 0,
                            spots_free: 0,
                        },
                    };
                    self.add_new_stat_occupied_space(&mut parking_lot_stat, parking_space.vehicle_type, occupied);
                    parking_lot_stats.push(parking_lot_stat);
                }
            }
        }

        Ok(parking_lot_stats)
    }

    fn add_new_stat_occupied_space(&self, parking_lot_stat: &mut ParkingLotStatsResponse, vehicle_type: VehicleType, occupied: bool) {
        match vehicle_type {
            VehicleType::Car => {
                if !occupied {
                    parking_lot_stat.car.spots_free += 1;
                } else {
                    parking_lot_stat.car.spots_occupied += 1;
                }
            }
            VehicleType::Truck => {
                if !occupied {
                    parking_lot_stat.truck.spots_free += 1;
                } else {
                    parking_lot_stat.truck.spots_occupied += 1;
                }
            
            }
        }
    }

    pub async fn get_parking_lot_by_code(&self, code: &str) -> Result<ParkingLotResponse> {
        println!("code: {}", code);
        match code.len() {
            0..=7 => return Err(InvalidCodeError(code.to_string())),
            8 => (),
            _ => return Err(InvalidCodeError(code.to_string())),
        }
        let parking_lot = self
            .fetch_parkings()
            .await?
            .into_iter()
            .find(|parking_lot| parking_lot.id.starts_with(code))
            .map(|parking_lot| parking_lot);

        println!("parking_lot: {:?}", parking_lot);
        
        match parking_lot {
            Some(doc) => Ok(doc),
            None => Err(NotFoundError(code.to_string()))
        }
    }

    pub async fn get_parking_lot_income(&self, parking_lot_id: &str) -> Result<Vec<IncomeStats>> {
        let mut cursor = self
            .ticket_collection
            .find(
                doc! {
                    "parking_lot_id": parking_lot_id.to_owned(),
                },
                None,
            )
            .await
            .map_err(MongoQueryError)?;

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
        Ok(json_result)
    }
}