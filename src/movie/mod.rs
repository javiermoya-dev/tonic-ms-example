pub mod entity;
pub mod repository;
pub mod mapper;
pub mod interactor;
pub mod service;

pub mod grpc_movie {
  tonic::include_proto!("movie");
}
