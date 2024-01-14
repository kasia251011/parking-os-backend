use std::str::FromStr;

use bson::{oid::ObjectId, doc};
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::{Ticket, ParkingSpace},
    response::{TicketResponse, TicketUserResponse}, 
    schema::{CreateTicketSchema, CreateTicketUserSchema}
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_tickets(&self, user_id: &str, active: bool, vehicle_license_number: &str, parking_spot_id: &str, 
        issue_time_stamp: u32, end_time_stamp: u32, level: u32, parking_lot_id: &str
    ) -> Result<Vec<TicketResponse>> {
        let user_id_query = if user_id.is_empty() {
            doc! { "$ne": "" }
        } else {
            doc! { "$eq": user_id }
        };

        let vehicle_license_number_query = if vehicle_license_number.is_empty() {
            doc! { "$ne": "" }
        } else {
            doc! { "$eq": vehicle_license_number }
        };

        let parking_spot_id_query = if parking_spot_id.is_empty() {
            doc! { "$ne": "" }
        } else {
            doc! { "$eq": parking_spot_id }
        };

        let issue_time_stamp_query = if issue_time_stamp == 0 {
            doc! { "$ne": 0 }
        } else {
            doc! { "$eq": issue_time_stamp }
        };
    
        let end_timestamp_query = if active {
            doc! { "$eq": 0 }
        } else {
            if end_time_stamp == 0 {
                doc! { "$ne": 0 }
            } else {
                doc! { "$eq": end_time_stamp }
            }
        };

        let level_query = if level == 0 {
            doc! { "$ne": 0 }
        } else {
            doc! { "$eq": level }
        };

        let parking_lot_id_query = if parking_lot_id.is_empty() {
            doc! { "$ne": "" }
        } else {
            doc! { "$eq": parking_lot_id }
        };
    
        let filter = doc! {
            "user_id": user_id_query,
            "vehicle_license_number": vehicle_license_number_query,
            "parking_spot_id": parking_spot_id_query,
            "issue_timestamp": issue_time_stamp_query,
            "end_timestamp": end_timestamp_query,
            "level": level_query,
            "parking_lot_id": parking_lot_id_query,
        };
        let mut cursor = self
            .ticket_collection
            .find(filter, None)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<TicketResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_ticket(&doc.unwrap())?);
        }
    
        Ok(json_result)
    }

    pub async fn create_ticket(&self, body: &CreateTicketSchema) -> Result<String> {
        let parking_lot = self
            .get_parking_lot_by_id(&body.parking_lot_id)
            .await?;

        let parking_space = self
            .get_new_parking_space_by_license_number(&body.vehicle_license_number, &parking_lot.id)
            .await?;

        println!("parking_space: {:?}", parking_space);

        let ticket_id = ObjectId::new();
        let ticket = Ticket {
            _id: ticket_id,
            user_id: body.user_id.to_owned(),
            vehicle_license_number: body.vehicle_license_number.to_owned(),
            parking_spot_id: parking_space._id.to_hex(),
            issue_timestamp: chrono::Utc::now().timestamp(),
            end_timestamp: 0,
            amount_paid: 0.0,
            level: parking_space.location.no_level,
            parking_lot_id: body.parking_lot_id.to_owned(),
            code: ticket_id.to_hex().chars().take(8).collect(),
        };

        match self.ticket_collection.insert_one(ticket, None).await {
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

        Ok(ticket_id.to_hex().chars().take(8).collect())
    }

    pub async fn get_ticket_by_code(&self, code: &str) -> Result<Ticket> {
        let filter = doc! { "code": code };
        let ticket = self
            .ticket_collection
            .find_one(filter, None)
            .await
            .map_err(MongoQueryError)?;

        match ticket {
            Some(ticket) => Ok(ticket),
            None => Err(MongoNotFound(format!("ticket with code: {}", code))),
        }
    }

    pub async fn put_ticket(&self, code: &str) -> Result<TicketResponse> {
        let ticket = self
            .get_ticket_by_code(code)
            .await?;

        let parking_space = self
            .get_parking_space_by_parking_spot_id(&ticket.parking_lot_id, &ticket.parking_spot_id)
            .await?;

        let ticket = self
            .update_ticket(&ticket, &parking_space)
            .await?;

        Ok(ticket)
    }

    pub async fn update_ticket(&self, ticket: &Ticket, parking_space: &ParkingSpace) -> Result<TicketResponse> {
        let filter = doc! { "_id": ticket._id.clone() };

        let tariffs = self
            .get_tariffs_by_parking_lot_id_ascending(&ticket.parking_lot_id)
            .await?;

        let mut amount_paid = 0.0;
        let end_timestamp = chrono::Utc::now().timestamp();
        let total_time = (end_timestamp - ticket.issue_timestamp) / 36000 + 1;
        while let Some(tariff) = tariffs.iter().next() {
            if total_time >= tariff.min_time && total_time <= tariff.max_time {
                amount_paid = parking_space.price_modifier * tariff.price_per_hour * total_time as f64;
                break;
            }
        }

        if amount_paid == 0.0 {
            amount_paid = parking_space.price_modifier * tariffs.last().unwrap().price_per_hour * total_time as f64;
        }
        
        let update = doc! { 
            "$set": { 
                "end_timestamp": end_timestamp,
                "amount_paid": amount_paid,
        }};

        self
            .ticket_collection
            .update_one(filter, update, None)
            .await
            .map_err(MongoQueryError)?;


        let ticket = self
            .get_ticket_by_id(&ticket._id.to_hex())
            .await;

        println!("ticket: {:?}", ticket);

        self.toggle_occupied_parking_space(&parking_space._id.to_hex()).await?;

        self.transfer_balance(&ticket.as_ref().unwrap().user_id, amount_paid).await?;

        match ticket {
            Ok(ticket) => Ok(self.doc_to_ticket(&ticket)?),
            Err(_) => Err(MongoNotFound(format!("ticket with code: {}", ticket.unwrap().code))),
        }
    }

    pub async fn get_ticket_by_id(&self, id: &str) -> Result<Ticket> {
        let filter = doc! { "_id": ObjectId::from_str(id).unwrap() };
        let ticket = self
            .ticket_collection
            .find_one(filter, None)
            .await
            .map_err(MongoQueryError)?;

        match ticket {
            Some(ticket) => Ok(ticket),
            None => Err(MongoNotFound(format!("ticket with id: {}", id))),
        }
    }

    fn doc_to_ticket(&self, ticket: &Ticket) -> Result<TicketResponse> {
        let ticket_response = TicketResponse {
            id: ticket._id.to_hex(),
            user_id: ticket.user_id.to_owned(),
            vehicle_license_number: ticket.vehicle_license_number.to_owned(),
            parking_spot_id: ticket.parking_spot_id.to_owned(),
            issue_timestamp: ticket.issue_timestamp,
            end_timestamp: ticket.end_timestamp,
            amount_paid: ticket.amount_paid,
            level: ticket.level,
            parking_lot_id: ticket.parking_lot_id.to_owned(),
            code: ticket.code.to_owned(),
        };

        Ok(ticket_response)
    }

    pub async fn get_user_active_tickets(&self, user_id: &str) -> Result<Vec<TicketUserResponse>> {
        let filter = doc! { 
            "user_id": user_id,
            "end_timestamp": 0,
        };
        let mut cursor = self
            .ticket_collection
            .find(filter, None)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<TicketUserResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_ticket_user(&doc.unwrap())?);
        }
    
        Ok(json_result)
    }

    fn doc_to_ticket_user(&self, ticket: &Ticket) -> Result<TicketUserResponse> {
        let ticket_response = TicketUserResponse {
            vehicle_license_number: ticket.vehicle_license_number.to_owned(),
            parking_spot_id: ticket.parking_spot_id.to_owned(),
            issue_timestamp: ticket.issue_timestamp,
            end_timestamp: ticket.end_timestamp,
            amount_paid: ticket.amount_paid,
            level: ticket.level,
            parking_lot_id: ticket.parking_lot_id.to_owned(),
            code: ticket.code.to_owned(),
        };

        Ok(ticket_response)
    }

    pub async fn create_user_ticket(&self, user_id: &str, body: &CreateTicketUserSchema) -> Result<String> {
        let parking_lot = self
            .get_parking_lot_by_id(&body.parking_lot_id)
            .await?;

        let parking_space = self
            .get_new_parking_space_by_license_number(&body.vehicle_license_number, &parking_lot.id)
            .await?;

        println!("parking_space: {:?}", parking_space);

        let ticket_id = ObjectId::new();
        let ticket = Ticket {
            _id: ticket_id,
            user_id: user_id.to_owned(),
            vehicle_license_number: body.vehicle_license_number.to_owned(),
            parking_spot_id: parking_space._id.to_hex(),
            issue_timestamp: chrono::Utc::now().timestamp(),
            end_timestamp: 0,
            amount_paid: 0.0,
            level: parking_space.location.no_level,
            parking_lot_id: body.parking_lot_id.to_owned(),
            code: ticket_id.to_hex().chars().take(8).collect(),
        };

        match self.ticket_collection.insert_one(ticket, None).await {
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

        Ok(ticket_id.to_hex().chars().take(8).collect())
    }
}