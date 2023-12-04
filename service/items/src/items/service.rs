use tonic::{Request, Response, Status};
use crate::items::interactor::ItemInteractor;
use crate::items::grpc_items::{GetItemByIdRequest, GetItemByIdResponse, 
    GetAllItemsRequest, GetAllItemsResponse ,CreateItemRequest, CreateItemResponse, UpdateItemRequest, 
    UpdateItemResponse,DeleteItemRequest, DeleteItemResponse };  
use crate::items::grpc_items::item_service_server::ItemService;
use crate::items::grpc_items::Item as GrpcItem;
use crate::items::mapper::{map_create_item_request_to_item_dto, map_update_item_request_to_item_dto};

pub struct ItemServiceImpl {
    interactor: Box<dyn ItemInteractor + Send + Sync>,
}

impl ItemServiceImpl {
    pub fn new(interactor: Box<dyn ItemInteractor + Send + Sync>) -> impl ItemService {
        ItemServiceImpl { interactor }
    }
}

#[tonic::async_trait]
impl ItemService for ItemServiceImpl {
    async fn get_item_by_id(
        &self,
        request: Request<GetItemByIdRequest>,
    ) -> Result<Response<GetItemByIdResponse>, Status> {
        let id = request.into_inner().id;

        match self.interactor.get_item_by_id(id).await {
            Ok(item) => {
                let response = GetItemByIdResponse { item: Some(item.into()) };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::not_found("Item not found")),
        }
    }


    async fn create_item(
        &self,
        request: Request<CreateItemRequest>,
    ) -> Result<Response<CreateItemResponse>, Status> {
        let new_item_request = request.into_inner();
        
        let new_item_dto = map_create_item_request_to_item_dto(new_item_request);
    
        // Luego, utilizas new_item_dto en lugar de new_item
        match self.interactor.create_item(new_item_dto).await {
            Ok(id) => {
                let response = CreateItemResponse { id };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::internal("Failed to create item")),
        }
    }

    async fn update_item(
        &self,
        request: Request<UpdateItemRequest>,
    ) -> Result<Response<UpdateItemResponse>, Status> {
        let update_request = request.into_inner();
    
        let updated_item_dto = map_update_item_request_to_item_dto(update_request.clone());
    
        match self.interactor.update_item(update_request.id, updated_item_dto).await {
            Ok(updated_successfully) => {
                let response = UpdateItemResponse { status: updated_successfully };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::internal("Failed to update item")),
        }
    }

    async fn delete_item(
        &self,
        request: Request<DeleteItemRequest>,
    ) -> Result<Response<DeleteItemResponse>, Status> {
        let id = request.into_inner().id;
    
        match self.interactor.delete_item(id).await {
            Ok(deleted_successfully) => {
                let response = DeleteItemResponse { status: deleted_successfully };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::internal("Failed to delete item")),
        }
    }

    async fn get_all_items(
        &self,
        _request: Request<GetAllItemsRequest>,
    ) -> Result<Response<GetAllItemsResponse>, Status> {
        match self.interactor.get_all_items().await {
            Ok(items) => {
                let items_grpc: Vec<GrpcItem> = items.into_iter().map(|item| item.into()).collect();
                let response = GetAllItemsResponse { items: items_grpc.into() };
                Ok(Response::new(response))
            }
            Err(_) => Err(Status::internal("Failed to fetch items")),
        }
    }
}
