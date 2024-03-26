use serde::{Serialize, Deserialize};
use bson::oid::ObjectId;
use sqlx::prelude::FromRow;


#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct User {
    pub id: i32,
    pub company_id: Option<i32>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateUser {
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeleteUser {
    id: ObjectId,
}

