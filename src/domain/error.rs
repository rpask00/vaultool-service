use std::error::Error;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("Not found")]
    NotFound,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}


impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        log_error_chain(&self);

        let status = match &self {
            ApiError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound => StatusCode::NOT_FOUND,
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}



#[derive(Debug, Error)]
pub enum StoreError {
    #[error("Item not found")]
    NotFound,
    #[error("Item with the same name already exists")]
    DuplicateName,
    #[error("Unexpected error")]
    UnexpectedError(#[source] Report),
}

impl IntoResponse for StoreError {
    fn into_response(self) -> Response {
        log_error_chain(&self);
        
        let status = match &self {
            StoreError::NotFound => StatusCode::NOT_FOUND,
            StoreError::DuplicateName => StatusCode::CONFLICT,
            StoreError::UnexpectedError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = Json(ErrorResponse {
            error: self.to_string(),
        });

        (status, body).into_response()
    }
}

impl From<StoreError> for ApiError {
    fn from(error: StoreError) -> Self {
        match error {
            StoreError::NotFound => ApiError::NotFound,
            StoreError::DuplicateName => ApiError::UnexpectedError(eyre!("Duplicate name error")),
            StoreError::UnexpectedError(e) => ApiError::UnexpectedError(e),
        }
    }
}

fn log_error_chain(e: &(dyn Error + 'static)) {
    let separator = "\n-----------------------------------------------------------------------------------\n";
    let mut report = format!("{}{:?}\n", separator, e);
    let mut current = e.source();
    while let Some(cause) = current {
        let str = format!("Caused by:\n\n{:?}", cause);
        report = format!("{}\n{}", report, str);
        current = cause.source();
    }
    report = format!("{}\n{}", report, separator);
    tracing::error!("{}", report);
}

