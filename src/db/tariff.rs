use std::str::FromStr;

use bson::{oid::ObjectId, doc};
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::Tariff,
    response::TariffResponse,
    schema::CreateTariffSchema
};


use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_tariffs(&self) -> Result<Vec<TariffResponse>> {
        let mut cursor = self
            .tariff_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;

        let mut json_result: Vec<TariffResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_tariff(&doc.unwrap())?);
        }

        Ok(json_result)
    }

    pub async fn create_tariff(&self, body: &CreateTariffSchema) -> Result<String> {
        let tariff = Tariff {
            _id: ObjectId::new(),
            parking_lot_id: body.parking_lot_id.to_owned(),
            min_time: body.min_time,
            max_time: body.max_time,
            price_per_hour: body.price_per_hour,
        };

        match self.tariff_collection.insert_one(tariff, None).await {
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

    pub async fn get_tariff_by_id(&self, tariff_id: &str) -> Result<Tariff> {
        let filter = doc! { "_id": ObjectId::from_str(tariff_id).unwrap() };
        let tariff = self
            .tariff_collection
            .find_one(filter, None)
            .await
            .map_err(MongoQueryError)?;

        match tariff {
            Some(tariff) => Ok(tariff),
            None => Err(MongoNotFound("Tariff not found".to_string())),
        }
    }

    fn doc_to_tariff(&self, tariff: &Tariff) -> Result<TariffResponse> {
        Ok(TariffResponse {
            parking_lot_id: tariff.parking_lot_id.to_owned(),
            min_time: tariff.min_time,
            max_time: tariff.max_time,
            price_per_hour: tariff.price_per_hour,
        })
    }
}