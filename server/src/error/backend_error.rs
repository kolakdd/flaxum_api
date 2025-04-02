use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Internal server error")]
    InternalError(String),
    #[error("Can't close access your self")]
    CloseAccessYourSelf,
}

impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        let status_code = match self {
            BackendError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            BackendError::CloseAccessYourSelf => StatusCode::BAD_REQUEST,
        };
        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
