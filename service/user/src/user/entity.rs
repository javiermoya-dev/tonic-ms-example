use schema::schema::{ users };
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use chrono::NaiveDateTime;

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub store_id: Option<i32>,
    pub branch_id: Option<i32>,
    pub login_session: String
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "users"]
pub struct UserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
    pub store_id: Option<i32>,
    pub branch_id: Option<i32>
}
