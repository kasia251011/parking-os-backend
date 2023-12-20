use bson::oid::ObjectId;
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::Ticket,
    response::TicketResponse, 
    schema::CreateTicketSchema
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_tickets(&self) -> Result<Vec<TicketResponse>> {
        let mut cursor = self
            .ticket_collection
            .find(None, None)
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
            .get_new_parking_space_by_licence_number(&body.vehicle_licence_number, &parking_lot.id)
            .await?;

        println!("parking_space: {:?}", parking_space);

        let ticket_id = ObjectId::new();
        let ticket = Ticket {
            _id: ticket_id,
            user_id: body.user_id.to_owned(),
            vehicle_licence_number: body.vehicle_licence_number.to_owned(),
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

        Ok(ticket_id.to_hex())
    }

    fn doc_to_ticket(&self, ticket: &Ticket) -> Result<TicketResponse> {
        let ticket_response = TicketResponse {
            id: ticket._id.to_hex(),
            user_id: ticket.user_id.to_owned(),
            vehicle_licence_number: ticket.vehicle_licence_number.to_owned(),
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