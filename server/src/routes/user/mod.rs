pub mod handler;

use axum::{routing::get, Router};

use crate::state::user_state::UserState;

pub fn routes() -> Router<UserState> {
    Router::new()
        .route("/user/list", get(handler::admin_get_users))
        .route(
            "/user/me",
            get(handler::get_user)
                .put(handler::update_user)
                .delete(handler::delete_user),
        )
}
