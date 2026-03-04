use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::data_stores::ItemsStore;

pub type ItemsStoreType = Arc<RwLock<dyn ItemsStore>>;

#[derive(Clone)]
pub struct AppState {
    pub items_store: ItemsStoreType,
}

impl AppState {
    pub fn new(
        items_store: ItemsStoreType,
    ) -> Self {
        Self {
            items_store,
        }
    }
}
