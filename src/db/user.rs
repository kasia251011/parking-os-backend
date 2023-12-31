use std::str::FromStr;

use bson::{oid::ObjectId, doc};
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::User,
    response::UserResponse, 
    schema::CreateUserSchema
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_users(&self) -> Result<Vec<UserResponse>> {
        let mut cursor = self
            .user_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<UserResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_user(&doc.unwrap())?);
        }
    
        Ok(json_result)
    }

    pub async fn create_user(&self, body: &CreateUserSchema) -> Result<String> {
        let user = User {
            _id: ObjectId::new(),
            name: body.name.to_owned(),
            surname: body.surname.to_owned(),
            account_balance: body.account_balance,
            blocked: body.blocked,
        };

        match self.user_collection.insert_one(user, None).await {
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

    pub async fn get_user_by_id(&self, user_id: &str) -> Result<User> {
        let filter = doc! { "_id": ObjectId::from_str(user_id).unwrap() };
        let user = self
            .user_collection
            .find_one(filter, None)
            .await
            .map_err(MongoQueryError)?
            .ok_or(MongoNotFound("User not found".to_string()))?;

        Ok(user)
    }

    pub async fn transfer_balance(&self, user_id: &str, amount: f64) -> Result<String> {
        let user = self.get_user_by_id(user_id).await?;
        let admin = self.get_user_by_id("6581ca3eee6bbb6e8aca7fc4").await?;
        let new_balance_admin = admin.account_balance + amount;
        let new_balance = match user.account_balance - amount {
            x if x < 0.0 => return Err(NotEnoughBalanceError("Not enough balance".to_string())),
            x => x,
        };

        let filter = doc! { "_id": ObjectId::from_str(user_id).unwrap() };
        let update = doc! { "$set": { "account_balance": new_balance } };
        self.user_collection
            .update_one(filter, update, None)
            .await
            .map_err(MongoQueryError)?;

        let filter = doc! { "_id": ObjectId::from_str("6581ca3eee6bbb6e8aca7fc4").unwrap() };
        let update = doc! { "$set": { "account_balance": new_balance_admin } };
        self.user_collection
            .update_one(filter, update, None)
            .await
            .map_err(MongoQueryError)?;

        Ok("Successful operation".to_string())
    }
    
    fn doc_to_user(&self, user: &User) -> Result<UserResponse> {
        let user_response = UserResponse {
            id: user._id.to_hex(),
            name: user.name.to_owned(),
            surname: user.surname.to_owned(),
            account_balance: user.account_balance.to_owned(),
            blocked: user.blocked.to_owned(),
        };

        Ok(user_response)
    }
}