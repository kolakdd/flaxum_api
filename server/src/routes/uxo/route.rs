use std::sync::Arc;

use crate::domain::access;
use crate::state::AppState;
use crate::utils::jwt;
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};

pub struct UserXObjectRouter {}

/// Роуты для контроля доступа над объектами
/// требуют авторизации
///
impl UserXObjectRouter {
    pub async fn init(app_state: Arc<AppState>) -> Router {
        Router::new()
            .route(
                "/list/:object_id",
                get(access::handler::list_access).with_state(app_state.clone()),
            )
            .route(
                "/give/:object_id",
                post(access::handler::post_give_access).with_state(app_state.clone()),
            )
            .route(
                "/close/:object_id",
                delete(access::handler::close_access).with_state(app_state.clone()),
            )
            .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
    }
}
