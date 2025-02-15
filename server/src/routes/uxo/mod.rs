pub mod handler;

use axum::{
    routing::{delete, get, post},
    Router,
};

use crate::state::object_state::ObjectState;

pub fn routes() -> Router<ObjectState> {
    Router::new()
        .route("/access/list/{object_id}", get(handler::list_access))
        .route("/access/give/{object_id}", post(handler::post_give_access))
        .route("/access/close/{object_id}", delete(handler::close_access))
}
