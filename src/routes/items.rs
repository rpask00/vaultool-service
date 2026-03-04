use crate::app_state::AppState;
use crate::domain::error::ApiError;
use crate::domain::item::Item;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct ItemsResponse {
    items: Vec<Item>,
}

pub async fn items(State(state): State<AppState>) -> Result<impl IntoResponse, ApiError> {
    let items_store = state.items_store.write().await;
    let items = items_store.get_items().await;

    Ok((StatusCode::OK, Json(ItemsResponse { items })))
}
