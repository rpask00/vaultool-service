use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub tags: Vec<String>,
}
