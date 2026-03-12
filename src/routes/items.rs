use crate::app_state::AppState;
use crate::domain::dto::item::{CreateItem, UpdateItem};
use crate::domain::error::ApiError;
use crate::domain::models::ListResponse;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ItemsQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub name: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ItemsQuery>,
) -> Result<impl IntoResponse, ApiError> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(10).clamp(1, 100);
    let name_filter = params.name.unwrap_or_default().to_lowercase();

    let items_store = state.items_store.write().await;
    let (total, items) = items_store.get_items(page, per_page, name_filter).await?;

    Ok((
        StatusCode::OK,
        Json(ListResponse {
            total,
            items,
            page,
            per_page,
        }),
    ))
}

pub async fn get(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> Result<impl IntoResponse, ApiError> {
    let items_store = state.items_store.write().await;
    let item = items_store.get_item(id).await?;

    Ok((StatusCode::OK, Json(item)))
}

pub async fn create(
    State(state): State<AppState>,
    Json(payload): Json<CreateItem>,
) -> Result<impl IntoResponse, ApiError> {
    let mut items_store = state.items_store.write().await;
    let item = items_store.create_item(payload).await?;

    Ok((StatusCode::CREATED, Json(item)))
}

pub async fn update(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<u32>,
    Json(payload): Json<UpdateItem>,
) -> Result<impl IntoResponse, ApiError> {
    let mut items_store = state.items_store.write().await;
    let item = items_store.update_item(id, payload).await?;

    Ok((StatusCode::OK, Json(item)))
}


pub async fn delete(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<u32>,
) -> Result<impl IntoResponse, ApiError> {
    let mut items_store = state.items_store.write().await;
    let mut files_store = state.files_store.write().await;
    let files = files_store.get_files(vec![id]).await?;

    items_store.delete_item(id).await?;
    files_store.delete_files_from_fs(files).await?;

    Ok(StatusCode::NO_CONTENT)
}