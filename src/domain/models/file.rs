use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum FileCategory {
    PHOTO = 1,
    OTHER = 999,
}

impl Serialize for FileCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32( *self as u32)
    }
}


impl From<String> for FileCategory {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "1" => Self::PHOTO,
            _ => Self::OTHER,
        }
    }
}

impl From<i32> for FileCategory {
    fn from(value: i32) -> Self {
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
