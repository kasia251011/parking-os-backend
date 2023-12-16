use bson::oid::ObjectId;
use futures::StreamExt;

use crate::structs::{
    error::MyError::{*, self}, 
    model::User,
    response::{UserListResponse, UserResponse, GenericResponse}, 
    schema::CreateUserSchema
};

use super::common::DB;

type Result<T> = std::result::Result<T, MyError>;

impl DB {
    pub async fn fetch_users(&self) -> Result<UserListResponse> {
        let mut cursor = self
            .user_collection
            .find(None, None)
            .await
            .map_err(MongoQueryError)?;
    
        let mut json_result: Vec<UserResponse> = Vec::new();
        while let Some(doc) = cursor.next().await {
            json_result.push(self.doc_to_user(&doc.unwrap())?);
        }
    
        Ok(UserListResponse {
            status: "200",
            users: json_result,
        })
    }

    pub async fn create_user(&self, body: &CreateUserSchema) -> Result<GenericResponse> {
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

        Ok(GenericResponse {
            status: "200".to_string(),
            message: "Successful operation".to_string(),
        })
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