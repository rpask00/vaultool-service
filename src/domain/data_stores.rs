use crate::domain::dto::file::CreateFile;
use crate::domain::dto::item::{CreateItem, UpdateItem};
use crate::domain::error::StoreError;
use crate::domain::models::file::File;
use crate::domain::models::item::Item;
use axum::body::Bytes;

#[async_trait::async_trait]
pub trait ItemsStore: Send + Sync {
    async fn get_items(
        &self,
        page: u32,
        per_page: u32,
        name_filter: String,
    ) -> Result<(usize, Vec<Item>), StoreError>;
    async fn get_item(&self, id: u32) -> Result<Item, StoreError>;
    async fn create_item(&mut self, item: CreateItem) -> Result<Item, StoreError>;
    async fn update_item(&mut self, id: u32, item: UpdateItem) -> Result<Item, StoreError>;
    async fn delete_item(&mut self, id: u32) -> Result<(), StoreError>;
}

#[async_trait::async_trait]
pub trait FilesStore: Send + Sync {
    async fn get_files(&self, item_id: u32) -> Result<Vec<File>, StoreError>;
    async fn create_file(
        &mut self,
        file: CreateFile,
        file_data: Bytes,
    ) -> Result<File, StoreError>;
    async fn delete_file(&mut self, id: u32) -> Result<(), StoreError>;
    async fn delete_files_from_fs(&mut self, files: Vec<File>) -> Result<(), StoreError>;
}
