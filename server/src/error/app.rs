use axum::response::IntoResponse;
use http::StatusCode;
use sqlx;

#[derive(Debug)]
pub enum Error {
    // Other
    Internal,
    MissingFirstAndLastPaginationArguments,
    PassedFirstAndLastPaginationArguments,
}

impl std::convert::From<Error> for crate::Error {
    fn from(err: Error) -> Self {
        match err {
            // Other
            Error::Internal => crate::Error::Internal(String::new()),
            Error::MissingFirstAndLastPaginationArguments => crate::Error::InvalidArgument(
                "You must provide a `first` or `last` value to properly paginate the entity."
                    .to_string(),
            ),
            Error::PassedFirstAndLastPaginationArguments => crate::Error::InvalidArgument(
                "Passing both `first` and `last` for pagination is not supported.".to_string(),
            ),
        }
    }
}

impl std::convert::From<sqlx::Error> for Error {
    fn from(_: sqlx::Error) -> Self {
        Error::Internal
    }
}

/// DB ERRORS
struct DbError(sqlx::Error);

impl From<sqlx::Error> for DbError {
    fn from(error: sqlx::Error) -> Self {
        Self(error)
    }
}

impl IntoResponse for DbError {
    fn into_response(self) -> axum::response::Response {
        println!("ERROR: {}", self.0);
        (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
    }
}
