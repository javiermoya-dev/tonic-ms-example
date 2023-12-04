use tonic::{transport::Server};
use std::net::SocketAddr;

const SERVICE_NAME: &str = "items";
pub mod user;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = configuration::Config::new("service/.env", SERVICE_NAME)?;
    println!("{:?}", cfg.service_port);
    let conn = db::db_connection();
    
    let repository = user::repository::UserRepositoryImpl::new(conn.clone());
    let interactor = user::interactor::UserInteractorImpl::new(repository);
    let service = user::service::UserServiceImpl::new(interactor);
    
    
    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    Server::builder()
        .add_service(user::grpc_user::user_service_server::UserServiceServer::new(service))
        .serve(server_addr)
        .await?;
    
    Ok(())
}

