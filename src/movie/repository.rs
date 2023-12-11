use crate::movie::entity::{Movie, MovieDTO};
use db::DbPool;
use diesel::prelude::*;
use schema::schema::movies;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tonic::async_trait]
pub trait MovieRepository {
    async fn get_all_movies(&self) -> Result<Vec<Movie>, Error>;
}

pub struct MovieRepositoryImpl {
    db_pool: DbPool,
}

impl MovieRepositoryImpl {
    pub fn new(db_pool: DbPool) -> Box<dyn MovieRepository + Send + Sync> {
        Box::new(MovieRepositoryImpl { db_pool })
    }
}

#[tonic::async_trait]
impl MovieRepository for MovieRepositoryImpl {
    async fn get_all_movies(&self) -> Result<Vec<Movie>, Error> {
        let conn = &mut self.db_pool.get().expect("Failed to get a database connection from the pool");
        let all_movies = movies::table.load::<Movie>(conn)?;
        Ok(all_movies)
    }
}
