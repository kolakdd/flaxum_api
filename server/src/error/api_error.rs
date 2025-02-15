use std::io;

use crate::error::{
    db_error::DbError, id_error::IdError, io_error::WriteReadError, s3_error::ApiS3Error,
    token_error::TokenError, user_error::UserError,
};
use aws_sdk_s3;
use axum::{
    extract::multipart::MultipartError,
    response::{IntoResponse, Response},
};
use sqlx;
use thiserror::Error;
use uuid;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    TokenError(#[from] TokenError),
    #[error(transparent)]
    UserError(#[from] UserError),
    #[error(transparent)]
    DbError(#[from] DbError),
    #[error(transparent)]
    IdError(#[from] IdError),
    #[error(transparent)]
    MultipartError(#[from] MultipartError),
    #[error(transparent)]
    WriteReadError(#[from] WriteReadError),
    #[error(transparent)]
    ApiS3Error(#[from] ApiS3Error),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::TokenError(error) => error.into_response(),
            ApiError::UserError(error) => error.into_response(),
            ApiError::DbError(error) => error.into_response(),
            ApiError::MultipartError(error) => error.into_response(),
            ApiError::IdError(error) => error.into_response(),
            ApiError::WriteReadError(error) => error.into_response(),
            ApiError::ApiS3Error(error) => error.into_response(),
        }
    }
}

impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        ApiError::DbError(DbError::from(error))
    }
}

impl From<uuid::Error> for ApiError {
    fn from(error: uuid::Error) -> Self {
        ApiError::IdError(IdError::from(error))
    }
}

impl From<io::Error> for ApiError {
    fn from(error: io::Error) -> Self {
        ApiError::WriteReadError(WriteReadError::from(error))
    }
}

impl From<aws_sdk_s3::Error> for ApiError {
    fn from(error: aws_sdk_s3::Error) -> Self {
        ApiError::ApiS3Error(ApiS3Error::from(error))
    }
}
