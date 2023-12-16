use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserSchema {
    pub name: String,
    pub surname: String,
    pub account_balance: f64,
    pub blocked: bool,
}