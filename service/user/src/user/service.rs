use tonic::{Request, Response, Status};

use crate::user::interactor::UserInteractor;
use crate::user::grpc_user::{
  UserByIdRequest, UserByIdResponse, 
  UpdateUserLoginSessionRequest, EmptyResponse,
}; 

use crate::user::grpc_user::user_service_server::UserService; 

pub struct  UserServiceImpl{
  interactor: Box<dyn UserInteractor + Send + Sync>,
}

impl UserServiceImpl {
  pub fn new(interactor: Box<dyn UserInteractor + Send + Sync>) -> impl UserService {
      UserServiceImpl { interactor }
  }
}

#[tonic::async_trait]
impl UserService for UserServiceImpl {
    async fn get_user_by_id(&self, request: Request<UserByIdRequest>) -> Result<Response<UserByIdResponse>, Status> {
        let UserByIdRequest { id_user } = request.into_inner();
        if !id_user.is_positive() {
            //return Err(Status::invalid_argument("EMPTY_ID_ERR"));
            return status::Status::invalid_arguments(vec!["id_user"]);
        }

        match self.interactor.get_user_by_id(id_user).await {
          Ok(user) => Ok(
              Response::new(
                  UserByIdResponse { user: Some(user.into()) }
              )
          ),
          Err(error) => status::Status::not_found(error)
        }
    }

    async fn update_user_login_session(&self, request: Request<UpdateUserLoginSessionRequest>) -> Result<Response<EmptyResponse>, Status> {
      let UpdateUserLoginSessionRequest { username_or_email } = request.into_inner();
      if username_or_email.is_empty() {
        return status::Status::invalid_arguments(vec!["username_or_email"]);
      }

      match self.interactor.get_user_by_username(&username_or_email).await {
        Ok(_) => match self.interactor.update_login_session_to_db(&username_or_email).await {
          Ok(_) => Ok(
            Response::new(
              EmptyResponse { }
            )
          ),
          Err(error) => status::Status::internal( error)
        },
        Err(error) => status::Status::not_found( error )
      }
    }
}
