use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use uuid::Error as UuidError;

#[derive(Error, Debug)]
pub enum IdError {
    #[error("IdError: {0}")]
    UuidError(#[from] UuidError),
}

impl IntoResponse for IdError {
    fn into_response(self) -> Response {
        let status_code = match self {
            IdError::UuidError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
