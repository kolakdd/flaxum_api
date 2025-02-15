mod handler;

use crate::state::auth_state::AuthState;
use axum::{routing::post, Router};

pub fn routes() -> Router<AuthState> {
    Router::new()
        .route("/user/login", post(handler::access_token))
        .route("/user/register", post(handler::register_user))
        .route("/refresh_token", post(handler::refresh_token))
}
