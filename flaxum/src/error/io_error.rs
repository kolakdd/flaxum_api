use std::io::Error as IoError;

use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WriteReadError {
    #[error("IoError: {0}")]
    IoError(#[from] IoError),
}

impl IntoResponse for WriteReadError {
    fn into_response(self) -> Response {
        let status_code = match self {
            WriteReadError::IoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
