use crate::items::entity::{Item, ItemDTO};
use crate::items::repository::ItemRepository;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tonic::async_trait]
pub trait ItemInteractor {
    async fn get_item_by_id(&self, id: i32) -> Result<Item, Error>;
    async fn create_item(&self, new_item: ItemDTO) -> Result<i32, Error>;
    async fn update_item(&self, id: i32, updated_item: ItemDTO) -> Result<bool, Error>;
    async fn delete_item(&self, id: i32) -> Result<bool, Error>;
    async fn get_all_items(&self) -> Result<Vec<Item>, Error>;
}

pub struct ItemInteractorImpl {
    repository: Box<dyn ItemRepository + Send + Sync>,
}

impl ItemInteractorImpl {
    pub fn new(repository: Box<dyn ItemRepository + Send + Sync>) -> Box<dyn ItemInteractor + Send + Sync> {
        Box::new(ItemInteractorImpl { repository })
    }
}

#[tonic::async_trait]
impl ItemInteractor for ItemInteractorImpl {
    async fn get_item_by_id(&self, id: i32) -> Result<Item, Error> {
        self.repository.get_item_by_id(id).await
    }

    async fn create_item(&self, new_item: ItemDTO) -> Result<i32, Error> {
        self.repository.create_item(new_item).await
    }

    async fn update_item(&self, id: i32, updated_item: ItemDTO) -> Result<bool, Error> {
        self.repository.update_item(id, updated_item).await
    }

    async fn delete_item(&self, id: i32) -> Result<bool, Error> {
        self.repository.delete_item(id).await
    }

    async fn get_all_items(&self) -> Result<Vec<Item>, Error> {
        self.repository.get_all_items().await
    }
}