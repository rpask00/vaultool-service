use crate::app_state::AppState;
use crate::domain::error::ApiError;
use crate::domain::models::item::Item;
use axum::Json;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ItemsResponse {
    items: Vec<Item>,
    #[serde(rename = "totalItems")]
    total_items: usize,
}

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
    let items = items_store.get_items(page, per_page, name_filter).await;

    Ok((
        StatusCode::OK,
        Json(ItemsResponse {
            items,
            total_items: 100,
        }),
    ))
}

pub async fn create() -> impl IntoResponse {
    (StatusCode::CREATED, Json("Item created successfully"))
}
