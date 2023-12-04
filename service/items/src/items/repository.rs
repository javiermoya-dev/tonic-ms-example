use crate::items::entity::{Item, ItemDTO};
use db::DbPool;
use diesel::prelude::*;
use schema::schema::items;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tonic::async_trait]
pub trait ItemRepository {
    async fn get_item_by_id(&self, id: i32) -> Result<Item, Error>;
    async fn create_item(&self, new_item: ItemDTO) -> Result<i32, Error>;
    async fn update_item(&self, id: i32, updated_item: ItemDTO) -> Result<bool, Error>;
    async fn delete_item(&self, id: i32) -> Result<bool, Error>;
    async fn get_all_items(&self) -> Result<Vec<Item>, Error>;
}

pub struct ItemRepositoryImpl {
    db_pool: DbPool,
}

impl ItemRepositoryImpl {
    pub fn new(db_pool: DbPool) -> Box<dyn ItemRepository + Send + Sync> {
        Box::new(ItemRepositoryImpl { db_pool })
    }
}

#[tonic::async_trait]
impl ItemRepository for ItemRepositoryImpl {
    async fn get_item_by_id(&self, i: i32) -> Result<Item, Error> {
        let conn = &mut self.db_pool.get().expect("Failed to get a database connection from the pool");

        let item_data = items::table
        .filter(items::id.eq(i))
        .first::<Item>(conn)?;

        Ok(item_data)
    }

    async fn create_item(&self, new_item: ItemDTO) -> Result<i32, Error> {
        let conn = &mut self.db_pool.get().expect("Failed to get a database connection from the pool");

        diesel::insert_into(items::table)
            .values(&new_item)
            .execute(conn)?;

        let last_inserted_id: i32 = items::table
            .select(items::id)
            .order(items::id.desc())
            .first(conn)?;

        Ok(last_inserted_id)
    }

    async fn update_item(&self, i: i32, updated_item: ItemDTO) -> Result<bool, Error> {
        let conn = &mut self.db_pool.get().expect("Failed to get a database connection from the pool");

        let rows_affected = diesel::update(items::table.find(i))
            .set(&updated_item)
            .execute(conn)?;

        let updated_successfully = rows_affected > 0;

        Ok(updated_successfully)
    }

    async fn delete_item(&self, i: i32) -> Result<bool, Error> {
        let conn = &mut self.db_pool.get().expect("Failed to get a database connection from the pool");

        let rows_affected = diesel::delete(items::table.filter(items::id.eq(i)))
            .execute(conn)?;

        let deleted_successfully = rows_affected > 0;

        Ok(deleted_successfully)
    }

    async fn get_all_items(&self) -> Result<Vec<Item>, Error> {
        let conn = &mut self.db_pool.get().expect("Failed to get a database connection from the pool");
    
        let all_items = items::table.load::<Item>(conn)?;
    
        Ok(all_items)
    }
}