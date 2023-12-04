use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Error};
use r2d2::{self, PooledConnection};
type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type DbConnection = PooledConnection<ConnectionManager<MysqlConnection>>;
pub type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection() -> MysqlConnection {
    dotenv().ok();
    println!("from db");
    let database_url = env::var("DATABASE_URL")
      .expect("DATABASE_URL must be set");
      MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url)) 
   
}

pub fn db_connection() -> DbPool {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not found in .env file");
  let manager = ConnectionManager::<MysqlConnection>::new(database_url);
  Pool::builder()
      .build(manager)
      .expect("Failed to create connection pool")
} 




/* lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<MysqlConnection>::new(db_url);
        //Pool::new(manager).expect("Failed to create db pool")
        Pool::builder().build(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    // let conn = connection().expect("Failed to get db connection");
}

pub fn connection() -> Result<DbConnection, Error> {
    POOL.get()
        .map_err(|e| Error::ConnectionError(ConnectionError::InvalidConnectionUrl(e.to_string())))
}

pub fn connection2() -> DbConnection { 
    return POOL.get().unwrap();
}    */