use crate::domain::data_stores::ItemsStore;
use crate::domain::item::Item;
use sqlx::PgPool;

pub struct PostgresItemsStore {
    pool: PgPool,
}

impl PostgresItemsStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl ItemsStore for PostgresItemsStore {
    async fn get_items(&self) -> Vec<Item> {
        vec![Item {
            name: "Dźwig".to_string(),
        }]
    }
}
