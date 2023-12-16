// /users:
// get:
//   tags:
//     - users
//   summary: Get info on all users
//   description: Provides array with all registered users <br> Allowed roles<span>&#58;</span>  ```ADMIN```
//   operationId: adminGetUsers
//   responses:
//     '200':
//       description: successful operation
//       content:
//         application/json:
//           schema:
//             type: array
//             items:
//               $ref: '#/components/schemas/User'
// post:
//   tags:
//     - users
//   summary: Add a new user
//   description: Add a new user (development only) <br> Allowed roles<span>&#58;</span>  ```ADMIN```
//   operationId: addUser
//   requestBody:
//     description: Create a new user
//     content:
//       application/json:
//         schema:
//           $ref: '#/components/schemas/User'
//     required: true
//   responses:
//     '200':
//       description: Successful operation
//     '400':
//       description: Invalid input

// async fn get_users() -> impl axum::response::IntoResponse {
//     // Retrieve users data from MongoDB
//     let collection = db.collection::<User>("users");
//     let users = collection.find(None, None).await.unwrap().collect::<Vec<_>>().await;

//     // Convert users data to JSON
//     Json(users)
// }

// async fn add_user(user: Json<User>) -> impl axum::response::IntoResponse {
//     // Insert user data into MongoDB
//     let collection = db.collection::<User>("users");
//     collection.insert_one(user.0, None).await.unwrap();

//     "User added successfully"
// }

use std::sync::Arc;

use axum::{response::IntoResponse, http::StatusCode, extract::State, Json};

use crate::AppState;
use crate::structs::error::MyError;

pub async fn get_users(
    State(app_state): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> 
{
    match app_state
        .db
        .fetch_users()
        .await
        .map_err(MyError::from)
    {
        Ok(res) => Ok(Json(res)),
        Err(e) => Err(e.into()),
    }
}