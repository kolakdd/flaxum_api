mod handler;

use crate::{config, state::object_state::ObjectState};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::limit::RequestBodyLimitLayer;

pub fn routes() -> Router<ObjectState> {
    Router::new()
        .route("/folder", post(handler::create_own_folder))
        .route(
            "/upload",
            post(handler::upload_file).layer(RequestBodyLimitLayer::new(config::SIZE_1GB)),
        )
        .route("/download", get(handler::download_file))
        .route(
            "/object",
            get(handler::get_info)
                .put(handler::update_info)
                .delete(handler::delete_object),
        )
        .route("/object/own/list", post(handler::get_own_list))
        .route("/object/trash/list", post(handler::get_trash_list))
        .route("/object/shared/list", post(handler::get_shared_list))
}
