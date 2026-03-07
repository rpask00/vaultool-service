use crate::app_state::AppState;
use crate::domain::dto::file::CreateFile;
use crate::domain::error::ApiError;
use axum::body::Bytes;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::Multipart;
use color_eyre::eyre::eyre;
use serde_json::json;
use std::collections::HashMap;


pub async fn list(
    State(state): State<AppState>,
    Path(item_id): Path<u32>,
) -> Result<impl IntoResponse, ApiError> {
    let files_store = state.files_store.read().await;
    let files = files_store.get_files(item_id).await?;

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
        let field_name = &field.name().unwrap_or("unknown").to_string();

        if let Some(filename) = field.file_name() {
            // File field
            let filename = filename.to_string();
            let data = field
                .bytes()
                .await
                .map_err(|e| ApiError::UnexpectedError(eyre!(e)))?;
            files.push((filename, data));
        } else {
            // Form field
            let value = field
                .text()
                .await
                .map_err(|e| ApiError::UnexpectedError(eyre!(e)))?;
            form_data.insert(field_name.clone(), value);
        }
    }

    // Extract specific fields
    let category = form_data.get(&"category".to_string()).cloned();
    let description = form_data.get(&"description".to_string()).cloned();
    let item_id = form_data.get(&"item_id".to_string()).cloned();

    let mut files_store = state.files_store.write().await;

    for (filename, file_data) in files {
        files_store
            .create_file(
                CreateFile {
                    item_id: item_id
                        .clone()
                        .unwrap()
                        .parse::<u32>()
                        .map_err(|e| ApiError::UnexpectedError(eyre!(e)))?,
                    name: filename,
                    category: form_data
                        .get(&"category".to_string())
                        .cloned()
                        .unwrap()
                        .into(),
                },
                file_data,
            )
            .await?;
    }

    Ok((
        StatusCode::CREATED,
        Json(json!({
            "category": category.unwrap_or("unknown".to_string()),
            "description": description.unwrap_or("".to_string()),
            "item_id": item_id.unwrap_or("unknown".to_string())
        })),
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
