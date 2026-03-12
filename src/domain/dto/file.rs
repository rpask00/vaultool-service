use serde::{Deserialize, Serialize};
use crate::domain::models::file::FileCategory;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateFile {
    pub item_id: Option<u32>,
    pub name: String,
    pub category: FileCategory,
    pub priority: u32
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UpdateFile {
    pub item_id: Option<u32>,
    pub name: Option<String>,
    pub category: Option<FileCategory>,
    pub priority: Option<u32>
}
