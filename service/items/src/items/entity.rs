use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use schema::schema::items::{self};
use chrono::NaiveDateTime;

#[derive(Queryable, Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub max_capacity_day: String,
    pub most_liked: Option<rust_decimal::Decimal>,
    pub enabled_for_menu: Option<i8>,
    pub is_active: Option<i32>,
    pub restricted_18: Option<i8>,
    pub estimated_minutes_item_preparation: Option<i32>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize,AsChangeset)]
#[table_name = "items"]
pub struct ItemDTO {
    pub name: String,
    pub description: String,
    pub max_capacity_day: String,
    pub most_liked: Option<rust_decimal::Decimal>,
    pub enabled_for_menu: Option<i8>,
    pub is_active: Option<i32>,
    pub restricted_18: Option<i8>,
    pub estimated_minutes_item_preparation: Option<i32>,
    //pub created_at: Option<NaiveDateTime>,
    //pub updated_at: Option<NaiveDateTime>,
}