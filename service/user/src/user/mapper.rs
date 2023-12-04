use crate::user::grpc_user::User as UserMessage;
use crate::user::entity::User;

impl From<UserMessage> for User {
  fn from(value: UserMessage) -> User {
    User {
      id: value.id,
      username: value.username,
      email: value.email,
      password: value.password,
      store_id: value.store_id,
      branch_id: value.branch_id,
      login_session: value.login_session,
    }
  }
}

impl From<User> for UserMessage {
  fn from(value: User) -> UserMessage {
    UserMessage {
      id: value.id,
      username: value.username,
      email: value.email,
      password: value.password,
      store_id: value.store_id,
      branch_id: value.branch_id,
      login_session: value.login_session,
    }
  }
}
