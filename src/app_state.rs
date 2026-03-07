use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::data_stores::{FilesStore, ItemsStore};

pub type ItemsStoreType = Arc<RwLock<dyn ItemsStore>>;
pub type FilesStoreType = Arc<RwLock<dyn FilesStore>>;

#[derive(Clone)]
pub struct AppState {
    pub items_store: ItemsStoreType,
    pub files_store: FilesStoreType,
}

impl AppState {
    pub fn new(
        items_store: ItemsStoreType,
        files_store: FilesStoreType,
    ) -> Self {
        Self {
            items_store,
            files_store,
        }
    }
}
