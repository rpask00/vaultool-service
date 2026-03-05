use crate::domain::dto::item::CreateItem;
use crate::domain::models::item::Item;

#[async_trait::async_trait]
pub trait ItemsStore: Send + Sync {
    async fn get_items(&self, page: u32, per_page: u32, name_filter: String) -> Vec<Item>;
    // async fn create_item(&mut self, item: CreateItem) -> Result<Item, String>;
}
