use crate::dto::user::{AdminChangePasswordDto, AdminCreateUserDto, AdminCreateUserOut, ChangePasswordDto};
use crate::entity::pagination::Pagination;
use crate::entity::user::{User, AdminUsersPaginated};
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
use crate::response::api_response::OkMessage;
use crate::state::user_state::UserState;
use axum::Extension;
use axum::{extract::State, Json};
use axum_extra::extract::OptionalQuery;
use validator::Validate;

pub async fn admin_register_user(
    State(state): State<UserState>,
    Extension(_): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<AdminCreateUserDto>,
) -> Result<Json<AdminCreateUserOut>, ApiError> {
    let created_user = state.user_service.admin_register_user(payload).await?;
    Ok(Json(created_user))
}


pub async fn admin_change_user_password(
    State(state): State<UserState>,
    Extension(_): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<AdminChangePasswordDto>,
) -> Result<Json<OkMessage>, ApiError> {
    let change_pass = ChangePasswordDto{new_password: payload.new_password};
    let _ = state.user_service.change_password(change_pass, payload.id).await?;
    Ok(Json(OkMessage::default()))
}


pub async fn admin_get_user_list(
    State(state): State<UserState>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
    Extension(_): Extension<User>,
) -> Result<Json<AdminUsersPaginated>, ApiError> {
    let pagination = pagination.unwrap_or_default();
    pagination.validate().unwrap();

    let res: AdminUsersPaginated = state.user_service.admin_get_user_list(pagination).await?;
    Ok(Json(res))
}
