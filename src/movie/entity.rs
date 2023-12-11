use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use schema::schema::movies::{self};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Movie {
    pub id: i32,
    pub title: String,
    pub year: i32,
    pub genre: String,
    pub rating: String,
    //pub starRating: String,
    pub cast: String,
    pub image: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name = "movies"]
pub struct MovieDTO {
    pub title: String,
    pub year: i32,
    pub genre: String,
    pub rating: String,
    //pub starRating: String,
    pub cast: String,
    pub image: String,
}