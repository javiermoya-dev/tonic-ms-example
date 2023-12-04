use tonic::transport::Server;
use std::net::SocketAddr;

const SERVICE_NAME: &str = "items";

pub mod items;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let cfg = configuration::Config::new("service/.env", SERVICE_NAME)?;
    println!("{:?}", cfg.service_port);
    let conn = db::db_connection();
    
    let repository = items::repository::ItemRepositoryImpl::new(conn);
    let interactor = items::interactor::ItemInteractorImpl::new(repository);
    let service = items::service::ItemServiceImpl::new(interactor);

    let server_addr = SocketAddr::new(cfg.service_hostname.unwrap().parse().unwrap(), cfg.service_port.unwrap().parse().unwrap());
    
    Server::builder()
         .add_service(items::grpc_items::item_service_server::ItemServiceServer::new(service))
         .serve(server_addr)
         .await?;

    Ok(())
}