use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum FileCategory {
    PHOTO = 1,
    OTHER = 999,
}

impl FileCategory {
    pub fn from(value: i32) -> Self {
        match value {
            1 => Self::PHOTO,
            _ => Self::OTHER,
        }
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct File {
    pub id: u32,
    pub item_id: Option<u32>,
    pub name: String,
    pub ext: String,
    pub category: FileCategory,
    pub created_at: String,
    pub size: u32,
}