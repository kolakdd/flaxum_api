mod handler;

use crate::state::user_state::UserState;
use axum::{routing::post, Router};

pub fn routes() -> Router<UserState> {
    Router::new()
    .route("/admin/user/register", post(handler::admin_register_user))
    .route("/admin/user/password", post(handler::admin_change_user_password))
    .route("/admin/user/list", post(handler::admin_get_user_list))

}
