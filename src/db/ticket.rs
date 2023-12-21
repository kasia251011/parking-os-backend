use bson::{oid::ObjectId, doc};
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::{Ticket, ParkingSpace},
    response::TicketResponse, 
    schema::CreateTicketSchema
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_tickets(&self, user_id: &str, active: bool) -> Result<Vec<TicketResponse>> {
        let user_id_query = if user_id.is_empty() {
            doc! { "$ne": "" }
        } else {
            doc! { "$eq": user_id }
        };
    
        let end_timestamp_query = if active {
            doc! { "$eq": 0 }
        } else {
            doc! { "$ne": 0 }
        };
    
        let filter = doc! {
            "user_id": user_id_query,
            "end_timestamp": end_timestamp_query,
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
            issue_timestamp: chrono::Utc::now().timestamp(),
            end_timestamp: 0,
            amount_paid: 0.0,
            spot_name: parking_space.location.no_space.to_string(),
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
            .get_parking_space_by_location(&ticket.parking_lot_id, &ticket.spot_name, ticket.level)
            .await?;

        let ticket = self
            .update_ticket(&ticket, &parking_space)
            .await?;

        Ok(ticket)
    }

    pub async fn update_ticket(&self, ticket: &Ticket, parking_space: &ParkingSpace) -> Result<TicketResponse> {
        let filter = doc! { "_id": ticket._id.to_owned() };

        // TODO get ticket's tariff
        // TODO calculate amount paid
        let amount_paid = 1.0;
        let update = doc! { 
            "$set": { 
                "end_timestamp": chrono::Utc::now().timestamp(), 
                "amount_paid": amount_paid + parking_space.price_modifier 
        }};

        let ticket = self
            .ticket_collection
            .find_one_and_update(filter, update, None)
            .await
            .map_err(MongoQueryError)?;

        println!("ticket: {:?}", ticket);

        self.toggle_occupied_parking_space(&parking_space._id.to_hex()).await?;

        self.transfer_balance(&ticket.clone().unwrap().user_id, amount_paid).await?;

        match ticket {
            Some(ticket) => Ok(self.doc_to_ticket(&ticket)?),
            None => Err(MongoNotFound(format!("ticket with code: {}", ticket.unwrap().code))),
        }
    }

    fn doc_to_ticket(&self, ticket: &Ticket) -> Result<TicketResponse> {
        let ticket_response = TicketResponse {
            id: ticket._id.to_hex(),
            user_id: ticket.user_id.to_owned(),
            vehicle_license_number: ticket.vehicle_license_number.to_owned(),
            issue_timestamp: ticket.issue_timestamp,
            end_timestamp: ticket.end_timestamp,
            amount_paid: ticket.amount_paid,
            spot_name: ticket.spot_name.to_owned(),
            level: ticket.level,
            parking_lot_id: ticket.parking_lot_id.to_owned(),
            code: ticket.code.to_owned(),
        };

        Ok(ticket_response)
    }
}