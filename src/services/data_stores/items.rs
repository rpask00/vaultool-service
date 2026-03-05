use crate::domain::data_stores::ItemsStore;
use crate::domain::item::Item;
use sqlx::{PgPool, Row};

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
    async fn get_items(&self, page: u32, per_page: u32, name_filter: String) -> Vec<Item> {
        sqlx::query!(
            r#"
            SELECT id, name, description, tags
            FROM items
             WHERE name ILIKE $1
             LIMIT $2
                OFFSET $3
            "#,
            format!("%{}%", name_filter),
            per_page as i64,
            ((page - 1) * per_page) as i64
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default()
        .into_iter()
        .map(|row| Item {
            id: row.id as u32,
            name: row.name,
            description: row.description,
            quantity: 0,
            tags: row.tags,
        })
        .collect()
    }
}
