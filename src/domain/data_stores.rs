use crate::domain::dto::item::{CreateItem, UpdateItem};
use crate::domain::error::StoreError;
use crate::domain::models::item::Item;

#[async_trait::async_trait]
pub trait ItemsStore: Send + Sync {
    async fn get_items(&self, page: u32, per_page: u32, name_filter: String) -> Result<(usize, Vec<Item>), StoreError>;
    async fn get_item(&self, id: u32) -> Result<Item, StoreError>;
    async fn create_item(&mut self, item: CreateItem) -> Result<Item, StoreError>;
    async fn update_item(&mut self, id: u32, item: UpdateItem) -> Result<Item, StoreError>;
    async fn delete_item(&mut self, id: u32) -> Result<(), StoreError>;
}
