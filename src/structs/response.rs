use serde::Serialize;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Debug)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub surname: String,
    pub account_balance: f64,
    pub blocked: bool,
}

#[derive(Serialize, Debug)]
pub struct UserListResponse {
    pub status: &'static str,
    pub users: Vec<UserResponse>,
}