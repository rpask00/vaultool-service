use crate::app_state::AppState;
use crate::domain::dto::file::CreateFile;
use crate::domain::error::ApiError;
use crate::domain::models::file::FileCategory;
use axum::Json;
use axum::body::Bytes;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum_extra::extract::Multipart;
use color_eyre::eyre::eyre;
use serde_json::json;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct FilesQuery {
    item_id: u32,
}

pub async fn list(
    State(state): State<AppState>,
    Query(query): Query<FilesQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let files_store = state.files_store.read().await;
    let files = files_store.get_files(query.item_id).await?;

    Ok((StatusCode::OK, Json(files)))
}

pub async fn create(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, ApiError> {
    let mut form_data = HashMap::new();
    let mut files: Vec<(String, Bytes)> = Vec::new();

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| ApiError::UnexpectedError(eyre!(e)))?
    {
        if let Some(file_name) = field.file_name() {
            let file_name = file_name.to_owned();

            let file_data = field
                .bytes()
                .await
                .map_err(|e| ApiError::UnexpectedError(eyre!(e)))?;

            files.push((file_name.to_string(), file_data));
        } else {
            let field_name = field.name().unwrap_or("unknown").to_owned();

            let value = field
                .text()
                .await
                .map_err(|e| ApiError::UnexpectedError(eyre!(e)))?;
            form_data.insert(field_name, value);
        }
    }

    // Extract specific fields
    let category = form_data
        .get(&"category".to_string())
        .cloned()
        .map(|cat| cat.into())
        .unwrap_or(FileCategory::OTHER);

    let item_id = form_data
        .get("item_id")
        .ok_or_else(|| ApiError::UnexpectedError(eyre!("Missing item_id in form data")))?
        .parse::<u32>()
        .map_err(|e| ApiError::UnexpectedError(eyre!(e)))?;

    let mut files_store = state.files_store.write().await;

    let mut output  = vec![];

    for (file_name, file_data) in files {
        let file = files_store
            .create_file(
                CreateFile {
                    item_id,
                    name: file_name,
                    category,
                },
                file_data,
            )
            .await?;

        output.push(file);
    }

    Ok((
        StatusCode::CREATED,
        Json(json!(output)),
    ))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(file_id): Path<u32>,
) -> Result<impl IntoResponse, ApiError> {
    let mut files_store = state.files_store.write().await;
    files_store.delete_file(file_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
