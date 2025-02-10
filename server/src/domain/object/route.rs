use std::sync::Arc;

use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

use crate::{config, domain::object, state::AppState, utils::jwt};

pub struct ObjectRouter {}

// Роуты для взаимодействия с хранилищем и объектами
/// требуют авторизации
impl ObjectRouter {
    pub async fn init(app_state: Arc<AppState>) -> Router {
        Router::new()
            .route(
                "/folder",
                // Создать папку
                post(object::handler::create_folder).with_state(app_state.clone()),
            )
            .route(
                "/upload",
                post(object::handler::upload_file)
                    .layer(RequestBodyLimitLayer::new(config::SIZE_1GB))
                    .with_state(app_state.clone()),
            )
            .route(
                "/download",
                get(object::handler::download_file).with_state(app_state.clone()),
            )
            .route(
                "/object",
                get(object::handler::get_info)
                    .with_state(app_state.clone())
                    .put(object::handler::update_info)
                    .with_state(app_state.clone())
                    .delete(object::handler::delete_object)
                    .with_state(app_state.clone()),
            )
            .route(
                "/object/own/list",
                post(object::handler::get_own_list).with_state(app_state.clone()),
            )
            .route(
                "/object/trash/list",
                post(object::handler::get_trash_list).with_state(app_state.clone()),
            )
            .route(
                "/object/shared/list",
                post(object::handler::get_shared_list).with_state(app_state.clone()),
            )
            .layer(middleware::from_fn_with_state(app_state.clone(), jwt::auth))
    }
}
