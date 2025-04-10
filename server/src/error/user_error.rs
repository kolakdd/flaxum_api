use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserError {
    #[error("User not found")]
    UserNotFound,
    #[error("User is not super User")]
    NotSuperUser,
    #[error("User deleted")]
    UserDeleted,
    #[error("User blocked")]
    UserBlocked,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid password")]
    InvalidPassword,
}

impl IntoResponse for UserError {
    fn into_response(self) -> Response {
        let status_code = match self {
            UserError::UserNotFound => StatusCode::NOT_FOUND,
            UserError::NotSuperUser => StatusCode::FORBIDDEN,
            UserError::UserBlocked => StatusCode::UNAUTHORIZED,
            UserError::UserDeleted => StatusCode::UNAUTHORIZED,
            UserError::UserAlreadyExists => StatusCode::BAD_REQUEST,
            UserError::InvalidPassword => StatusCode::BAD_REQUEST,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
