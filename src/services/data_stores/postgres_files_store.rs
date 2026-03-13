use crate::domain::data_stores::FilesStore;
use crate::domain::dto::file::{CreateFile, UpdateFile};
use crate::domain::error::StoreError;
use crate::domain::models::file::{File, FileCategory};
use axum::body::Bytes;
use glob::glob;
use sqlx::PgPool;
use std::fs;
use tap::Pipe;

pub struct PostgresFilesStore {
    pool: PgPool,
}

impl PostgresFilesStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl FilesStore for PostgresFilesStore {
    async fn get_files(&self, item_ids: Vec<u32>) -> Result<Vec<File>, StoreError> {
        let item_ids: Vec<i32> = item_ids.into_iter().map(|id| id as i32).collect();

        sqlx::query!(
            r#"
            SELECT id, item_id, name, created_at, category, size, extension, priority, content_type
            FROM files
            WHERE item_id = ANY($1)
            LIMIT 1000
            "#,
            &item_ids
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(e.into()))?
        .into_iter()
        .map(|row| File {
            id: row.id as u32,
            item_id: row.item_id.map(|id| id as u32),
            name: row.name,
            content_type: row.content_type,
            priority: row.priority as u32,
            ext: row.extension,
            category: FileCategory::from(row.category),
            created_at: row.created_at.to_string(),
            size: row.size as u32,
        })
        .collect::<Vec<File>>()
        .pipe(Ok)
    }

    async fn create_file(
        &mut self,
        file: CreateFile,
        file_data: Bytes,
    ) -> Result<File, StoreError> {
        if fs::exists("uploads").unwrap() {
            fs::create_dir_all("uploads").map_err(|e| StoreError::UnexpectedError(e.into()))?;
        }

        let file = sqlx::query!(
            r#"
            INSERT INTO files (item_id, name, category, size, extension, priority, content_type)
            VALUES ($1, $2, $3, $4, $5, $6 , $7)
            RETURNING id, item_id, name, created_at, category, size, extension, priority, content_type
            "#,
            file.item_id.map(|id| id as i32),
            file.name,
            file.category as i32,
            file_data.len() as i64,
            file.name.split('.').last().unwrap_or("png").to_string(),
            file.priority as i64,
            file.content_type,
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(e.into()))
        .map(|row| File {
            id: row.id as u32,
            item_id: row.item_id.map(|id| id as u32),
            name: row.name,
            priority: row.priority as u32,
            content_type: row.content_type,
            ext: row.extension,
            category: FileCategory::from(row.category),
            created_at: row.created_at.to_string(),
            size: row.size as u32,
        })?;

        let file_clone = file.clone();

        tokio::task::spawn_blocking(move || {
            let file_path = format!("uploads/{}.{}", file_clone.id, file_clone.ext);
            fs::write(file_path, file_data).map_err(|e| StoreError::UnexpectedError(e.into()))
        })
        .await
        .map_err(|e| StoreError::UnexpectedError(e.into()))??;

        Ok(file)
    }

    async fn delete_file(&mut self, id: u32) -> Result<(), StoreError> {
        for path in glob(&format!("uploads/{}.*", id)).unwrap().flatten() {
            fs::remove_file(path).unwrap();
        }

        sqlx::query!(
            r#"
            DELETE FROM files
            WHERE id = $1
            "#,
            id as i64
        )
        .execute(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(e.into()))?;

        Ok(())
    }

    async fn update_file(&mut self, id: u32, file: UpdateFile) -> Result<File, StoreError> {
        sqlx::query!(
            r#"
            UPDATE files
            SET name = COALESCE($1, name),
                category = COALESCE($2, category),
                priority = COALESCE($3, priority),
                item_id = COALESCE($4, item_id)
            WHERE id = $5
            RETURNING id, item_id, name, created_at, category, size, extension, priority, content_type
            "#,
            file.name,
            file.category.as_ref().map(|c| *c as i32),
            file.priority.map(|p| p as i32),
            file.item_id.map(|p| p as i32),
            id as i64,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(e.into()))?
        .map(|row| File {
            id: row.id as u32,
            item_id: row.item_id.map(|id| id as u32),
            name: row.name,
            content_type: row.content_type,
            priority: row.priority as u32,
            ext: row.extension,
            category: FileCategory::from(row.category),
            created_at: row.created_at.to_string(),
            size: row.size as u32,
        })
        .ok_or(StoreError::NotFound)
    }

    async fn delete_files_from_fs(&mut self, files: Vec<File>) -> Result<(), StoreError> {
        for file in files {
            for path in glob(&format!("uploads/{}.*", file.id)).unwrap().flatten() {
                fs::remove_file(path).map_err(|e| StoreError::UnexpectedError(e.into()))?;
            }
        }

        Ok(())
    }
}
