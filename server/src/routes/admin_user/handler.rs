use crate::dto::user::{AdminChangePasswordDto, AdminCreateUserDto, AdminCreateUserOut, ChangePasswordDto};
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
use crate::response::api_response::OkMessage;
use crate::state::user_state::UserState;
use axum::Extension;
use axum::{extract::State, Json};

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
