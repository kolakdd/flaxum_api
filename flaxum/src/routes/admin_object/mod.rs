mod handler;

use crate::state::object_state::ObjectState;
use axum::{routing::post, Router};

pub fn routes() -> Router<ObjectState> {
    Router::new().route("/admin/object/list", post(handler::admin_get_object_list))
}
