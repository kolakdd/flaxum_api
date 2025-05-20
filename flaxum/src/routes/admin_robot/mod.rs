mod handler;

use crate::state::robot_state::RobotState;
use axum::{routing::post, Router};

pub fn routes() -> Router<RobotState> {
    Router::new()
        .route("/admin/robot/register", post(handler::admin_register_robot))
        .route("/admin/robot/delete",post(handler::admin_delete_robot))
        .route("/admin/robot/list", post(handler::admin_get_robot_list))
}
