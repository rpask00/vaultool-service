use serde::{Deserialize, Serialize};
use crate::domain::models::item::Item;

#[derive(Debug, Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: String,
    pub quantity: u32,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub description: Option<String>,
    pub quantity: Option<u32>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize)]
pub struct ItemsResponse {
    pub items: Vec<Item>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
}