use serde::{Deserialize, Serialize};
use serde_repr::Deserialize_repr;

#[derive(Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u32)]
pub enum FileCategory {
    PHOTO = 1,
    OTHER = 999,
}

impl Serialize for FileCategory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_u32(*self as u32)
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
    pub priority: u32,
    pub ext: String,
    pub category: FileCategory,
    pub created_at: String,
    pub size: u32,
}

// add test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_category_from_string() {
        assert_eq!(FileCategory::from("1".to_string()), FileCategory::PHOTO);
        assert_eq!(FileCategory::from("asdf".to_string()), FileCategory::OTHER);
        assert_eq!(FileCategory::from("other".to_string()), FileCategory::OTHER);
    }

    #[test]
    fn test_file_category_from_i32() {
        assert_eq!(FileCategory::from(1), FileCategory::PHOTO);
        assert_eq!(FileCategory::from(9999999), FileCategory::OTHER);
        assert_eq!(FileCategory::from(999), FileCategory::OTHER);
    }
}
