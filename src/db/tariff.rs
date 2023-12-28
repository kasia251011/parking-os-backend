use bson::{oid::ObjectId, doc};
use futures::StreamExt;
use mongodb::Cursor;

use crate::structs::{
    error::MyError::{*, self}, 
    model::Tariff,
    response::TariffResponse,
    schema::CreateTariffSchema
};


use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    // pub async fn fetch_tariffs(&self) -> Result<Vec<TariffResponse>> {
    //     let mut cursor = self
    //         .tariff_collection
    //         .find(None, None)
    //         .await
    //         .map_err(MongoQueryError)?;

    //     Ok(self.cursor_to_vec(&mut cursor).await?)
    // }

    pub async fn create_tariff(&self, body: &CreateTariffSchema, parking_lot_id: &str) -> Result<String> {
        let tariff = Tariff {
            _id: ObjectId::new(),
            parking_lot_id: parking_lot_id.to_owned(),
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

    pub async fn get_tariffs_by_parking_lot_id_ascending(&self, parking_lot_id: &str) -> Result<Vec<TariffResponse>> {
        let filter = doc! { "parking_lot_id": parking_lot_id };
        let options = mongodb::options::FindOptions::builder().sort(doc! { "min_time": 1 }).build();

        let mut cursor = self
            .tariff_collection
            .find(filter, options)
            .await
            .map_err(MongoQueryError)?;

        Ok(self.cursor_to_vec(&mut cursor).await?)
    }

    fn doc_to_tariff(&self, tariff: &Tariff) -> Result<TariffResponse> {
        Ok(TariffResponse {
            parking_lot_id: tariff.parking_lot_id.to_owned(),
            min_time: tariff.min_time,
            max_time: tariff.max_time,
            price_per_hour: tariff.price_per_hour,
        })
    }

    async fn cursor_to_vec(&self, cursor: &mut Cursor<Tariff>) -> Result<Vec<TariffResponse>> {
        let mut json_result: Vec<TariffResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_tariff(&doc.unwrap())?);
        }

        Ok(json_result)
    }
}