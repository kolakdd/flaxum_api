use std::sync::Arc;

use crate::{domain::auth, state::AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub struct AuthRouter {}

// Роуты для авторизации
impl AuthRouter {
    pub async fn init(app_state: Arc<AppState>) -> Router {
        Router::new()
            .route(
                "/user/register",
                post(auth::handler::register_user).with_state(app_state.clone()),
            )
            .route(
                "/user/login",
                post(auth::handler::access_token).with_state(app_state.clone()),
            )
            .route(
                "/refresh_token",
                get(auth::handler::refresh_token).with_state(app_state.clone()),
            )
    }
}
