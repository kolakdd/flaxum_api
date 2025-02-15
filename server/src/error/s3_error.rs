use aws_sdk_s3;

use crate::response::api_response::ApiErrorResponse;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiS3Error {
    #[error("S3Error: {0}")]
    ApiS3Error(#[from] aws_sdk_s3::Error),
}

impl IntoResponse for ApiS3Error {
    fn into_response(self) -> Response {
        let status_code = match self {
            ApiS3Error::ApiS3Error(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        ApiErrorResponse::send(status_code.as_u16(), Some(self.to_string()))
    }
}
