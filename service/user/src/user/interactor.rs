use crate::user::entity::User;
use crate::user::repository::UserRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;
use error::make_error;

#[tonic::async_trait]
 pub trait UserInteractor {
  async fn get_user_by_id(&self, i: i32) -> Result<User, Error>;
  async fn get_user_by_username(&self, username_or_email: &str) -> Result<User, Error>;
  async fn update_login_session_to_db(&self, username_or_email: &str, ) -> Result<(), Error>;
}

pub struct UserInteractorImpl {
  repository: Box<dyn UserRepository + Send + Sync>,
}

impl UserInteractorImpl {
  pub fn new(repository: Box<dyn UserRepository + Send + Sync>) -> Box<dyn UserInteractor + Send + Sync> {
    Box::new(UserInteractorImpl { repository })
  }
}

#[tonic::async_trait]
impl UserInteractor for UserInteractorImpl {
  async fn get_user_by_id(&self, i: i32)-> Result<User, Error> {
    self.repository.get_user_by_id(i).await
  }

  async fn get_user_by_username(&self, username_or_email: &str)-> Result<User, Error> {
    self.repository.get_user_by_username_or_email(username_or_email).await
  }

  async fn update_login_session_to_db(&self, username_or_email: &str) -> Result<(), Error> {
    let user_login_session = self.repository.generate_login_session().await?;
    
    self.repository.update_login_session_to_db(username_or_email, &user_login_session).await?;
    Ok(())
  }
}
