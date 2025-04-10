mod handler;

use axum::{
    routing::{get, put},
    Router,
};

use crate::state::user_state::UserState;

pub fn routes() -> Router<UserState> {
    Router::new()
        .route("/user/password", put(handler::change_password))
        .route("/user/me", get(handler::get_me).put(handler::update_me))
}
