use schema::schema::users::{self, dsl::*};
use db::DbPool;
use diesel::prelude::*;
use crate::user::entity::User;
use uuid::Uuid;

type Error = Box<dyn std::error::Error + Send + Sync>;
const POOL_MESSAGE: &str = "Failed database connection from the pool";

#[tonic::async_trait]
 pub trait UserRepository {
  async fn get_user_by_id(&self, i: i32) -> Result<User, Error>;
  async fn get_user_by_username_or_email(&self, username_or_email: &str) -> Result<User, Error>;
  async fn update_login_session_to_db(&self, username_or_email: &str, user_login_session: &str) -> Result<(), Error>;
  async fn generate_login_session(&self) -> Result<String, Error>;
}

pub struct UserRepositoryImpl {
  db_pool: DbPool,
}

impl UserRepositoryImpl {
  pub fn new(db_pool: DbPool) -> Box<dyn UserRepository + Send + Sync> {
     Box::new(UserRepositoryImpl { db_pool })    
  }
}

#[tonic::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_user_by_id(&self, i: i32) -> Result<User, Error> {
      let conn = &mut self.db_pool.get().expect(POOL_MESSAGE);
      
      let data = users::table
        .select((id, username, email, password, store_id, branch_id, login_session))
        .filter(users::id.eq(i))
        .first::<User>(conn)?;
      
      Ok(data)
    } 

    async fn get_user_by_username_or_email(&self, username_or_email: &str) -> Result<User, Error> {
      let conn = &mut self.db_pool.get().expect(POOL_MESSAGE);
      
      let data = users::table
        .select((id, username, email, password, store_id, branch_id, login_session))
        .filter(username.eq(username_or_email))
        .or_filter(email.eq(username_or_email))
        .first::<User>(conn)?;
      
      Ok(data)
    }    

    async fn update_login_session_to_db(&self, username_or_email: &str, user_login_session: &str) -> Result<(), Error> {
      let conn = &mut self.db_pool.get().expect(POOL_MESSAGE);
      
      diesel::update(users::table.filter(username.eq(username_or_email)))
        .set(login_session.eq(user_login_session.to_string()))
        .execute(conn);
      
      Ok(())
    } 
   
    async fn generate_login_session(&self) -> Result<String, Error> {
      let uuui_generated = Uuid::new_v4().to_string();
      Ok(uuui_generated)
    }
}
