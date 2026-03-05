use crate::domain::data_stores::ItemsStore;
use crate::domain::dto::item::{CreateItem, UpdateItem};
use crate::domain::error::StoreError;
use crate::domain::models::item::Item;
use color_eyre::eyre::eyre;
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
    async fn get_items(
        &self,
        page: u32,
        per_page: u32,
        name_filter: String,
    ) -> Result<(usize, Vec<Item>), StoreError> {
        let items = sqlx::query!(
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
        .map_err(|e| StoreError::UnexpectedError(eyre!(e)))?
        .into_iter()
        .map(|row| Item {
            id: row.id as u32,
            name: row.name,
            description: row.description,
            quantity: 0,
            tags: row.tags,
        })
        .collect::<Vec<Item>>();

        let total = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM items
             WHERE name ILIKE $1
            "#,
            format!("%{}%", name_filter)
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(eyre!(e)))?
        .count
        .unwrap_or(0) as usize;

        Ok((total, items))
    }

    async fn get_item(&self, id: u32) -> Result<Item, StoreError> {
        sqlx::query!(
            r#"
            SELECT id, name, description, tags
            FROM items
            WHERE id = $1
            "#,
            id as i64
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(eyre!(e)))?
        .map(|row| Item {
            id: row.id as u32,
            name: row.name,
            description: row.description,
            quantity: 0,
            tags: row.tags,
        })
        .ok_or(StoreError::NotFound)
    }

    async fn create_item(&mut self, item: CreateItem) -> Result<Item, StoreError> {
        sqlx::query!(
            r#"
            INSERT INTO items (name, description, tags, quantity)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, description, tags, quantity
            "#,
            item.name,
            item.description,
            &item.tags,
            item.quantity as i32
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(eyre!(e)))
        .map(|row| Item {
            id: row.id as u32,
            name: row.name,
            description: row.description,
            quantity: row.quantity as u32,
            tags: row.tags,
        })
    }

    async fn update_item(&mut self, id: u32, item: UpdateItem) -> Result<Item, StoreError> {
        sqlx::query!(
            r#"
            UPDATE items
            SET name = COALESCE($1, name),
                description = COALESCE($2, description),
                tags = COALESCE($3, tags)
            WHERE id = $4
            RETURNING id, name, description, tags
            "#,
            item.name,
            item.description,
            item.tags.as_deref(),
            id as i64
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(eyre!(e)))?
        .map(|row| Item {
            id: row.id as u32,
            name: row.name,
            description: row.description,
            quantity: 0,
            tags: row.tags,
        })
        .ok_or(StoreError::NotFound)
    }

    async fn delete_item(&mut self, id: u32) -> Result<(), StoreError> {
        sqlx::query!(
            r#"
            DELETE FROM items
            WHERE id = $1
            "#,
            id as i64
        )
        .execute(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(eyre!(e)))
        .map(|_| ())
    }
}
