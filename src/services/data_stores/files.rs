use crate::domain::data_stores::FilesStore;
use crate::domain::dto::file::CreateFile;
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
    async fn get_files(&self, item_id: u32) -> Result<Vec<File>, StoreError> {
        sqlx::query!(
            r#"
            SELECT id, item_id, name, created_at, category, size, extension
            FROM files
            WHERE item_id = $1
            "#,
            item_id as i64
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(e.into()))?
        .into_iter()
        .map(|row| File {
            id: row.id as u32,
            item_id: Some(row.item_id as u32),
            name: row.name,
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
        if fs::exists("uploads".to_string()).unwrap() {
            fs::create_dir_all("uploads").map_err(|e| StoreError::UnexpectedError(e.into()))?;
        }

        let file = sqlx::query!(
            r#"
            INSERT INTO files (item_id, name, category, size, extension)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, item_id, name, created_at, category, size, extension
            "#,
            file.item_id as i64,
            file.name,
            file.category as i32,
            file_data.len() as i64,
            file.name.split('.').last().unwrap_or("png").to_string()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| StoreError::UnexpectedError(e.into()))
        .map(|row| File {
            id: row.id as u32,
            item_id: Some(row.item_id as u32),
            name: row.name,
            ext: row.extension,
            category: FileCategory::from(row.category),
            created_at: row.created_at.to_string(),
            size: row.size as u32,
        })?;

        let file_clone = file.clone();

        tokio::task::spawn_blocking(move || {
            let file_path = format!("uploads/{}.{}", file_clone.id.to_string(), file_clone.ext);
            fs::write(file_path, file_data).map_err(|e| StoreError::UnexpectedError(e.into()))
        })
        .await
        .map_err(|e| StoreError::UnexpectedError(e.into()))??;

        Ok(file)
    }

    async fn delete_file(&mut self, id: u32) -> Result<(), StoreError> {
        for entry in glob(&format!("uploads/{}.*", id)).unwrap() {
            if let Ok(path) = entry {
                fs::remove_file(path).unwrap();
            }
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

    async fn delete_files_from_fs(&mut self, files: Vec<File>) -> Result<(), StoreError> {
        for file in files {
            for entry in glob(&format!("uploads/{}.*", file.id)).unwrap() {
                if let Ok(path) = entry {
                    fs::remove_file(path).map_err(
                        |e| StoreError::UnexpectedError(e.into())
                    )?;
                }
            }
        }

        Ok(())
    }
}
