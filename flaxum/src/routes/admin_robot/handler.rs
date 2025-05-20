use crate::dto::robot::{CreateRobotDto, CreateRobotOut, DeleteRobotDto};
use crate::entity::pagination::Pagination;
use crate::entity::robot::RobotsPaginated;
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
use crate::response::api_response::OkMessage;
use crate::service::robot_service::RobotServiceTrait;
use crate::state::robot_state::RobotState;
use axum::Extension;
use axum::{extract::State, Json};
use axum_extra::extract::OptionalQuery;
use validator::Validate;

pub async fn admin_register_robot(
    State(state): State<RobotState>,
    Extension(_): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<CreateRobotDto>,
) -> Result<Json<CreateRobotOut>, ApiError> {
    let created_robot = state.robot_service.admin_register_robot(payload).await?;
    Ok(Json(created_robot))
}

pub async fn admin_get_robot_list(
    State(state): State<RobotState>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
    Extension(_): Extension<User>,
) -> Result<Json<RobotsPaginated>, ApiError> {
    let pagination = pagination.unwrap_or_default();
    pagination.validate().unwrap();

    let res = state.robot_service.admin_get_robot_list(pagination).await?;
    Ok(Json(res))
}

pub async fn admin_delete_robot(
    State(state): State<RobotState>,
    Extension(_): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<DeleteRobotDto>,
) -> Result<Json<OkMessage>, ApiError> {
    state.robot_service.admin_delete_robot(payload).await?;
    Ok(Json(OkMessage::default()))
}
