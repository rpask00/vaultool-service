use crate::domain::item::Item;

#[async_trait::async_trait]
pub trait ItemsStore: Send + Sync {
    async fn get_items(&self) -> Vec<Item>;
}
