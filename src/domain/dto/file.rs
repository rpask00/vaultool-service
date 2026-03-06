use serde::{Deserialize, Serialize};
use crate::domain::models::file::FileCategory;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateFile {
    pub item_id: u32,
    pub name: String,
    pub category: FileCategory,
}
