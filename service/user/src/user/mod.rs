pub mod entity;
pub mod mapper;
pub mod repository;
pub mod interactor;
pub mod service;

pub mod grpc_user {
  tonic::include_proto!("user");
}