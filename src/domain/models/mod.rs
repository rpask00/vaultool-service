use serde::{Deserialize, Serialize};

pub mod item;

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResponse<T> {
    pub items: Vec<T>,
    pub total: usize,
    pub page: u32,
    pub per_page: u32,
}
