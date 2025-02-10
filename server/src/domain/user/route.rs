use std::sync::Arc;

use axum::{middleware, routing::get, Router};

use crate::{domain::user, state::AppState, utils::jwt};

pub struct UserRouter {}

// Роуты для взаимодействия с пользователями
/// требуют авторизации

impl UserRouter {
    pub async fn init(app_state: Arc<AppState>) -> Router {
        Router::new()
            .route(
                "/user/list",
                get(user::handler::admin_get_users).with_state(app_state.clone()),
            )
            .route(
                "/user/me",
                get(user::handler::get_user)
                    .with_state(app_state.clone())
                    .put(user::handler::update_user)
                    .with_state(app_state.clone())
                    .delete(user::handler::delete_user)
                    .with_state(app_state.clone()),
            )
            .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
    }
}
