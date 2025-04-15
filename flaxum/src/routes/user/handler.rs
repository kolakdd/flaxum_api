use crate::dto::user::{ChangePasswordDto, UpdateUserMeDto};
use crate::entity::user::PublicUser;
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
use crate::response::api_response::OkMessage;
use crate::{entity::user::User, state::user_state::UserState};

use axum::Json;
use axum::{extract::State, Extension};

pub async fn get_me(
    State(_): State<UserState>,
    Extension(current_user): Extension<User>,
) -> Result<Json<PublicUser>, ApiError> {
    let public_user = PublicUser::from(current_user);
    Ok(Json(public_user))
}

pub async fn update_me(
    State(state): State<UserState>,
    Extension(current_user): Extension<User>,
    ValidatedRequest(dto): ValidatedRequest<UpdateUserMeDto>,
) -> Result<Json<PublicUser>, ApiError> {
    let public_user = state
        .user_service
        .update_user_me(dto, current_user.id)
        .await?;
    Ok(Json(public_user))
}

pub async fn change_password(
    State(state): State<UserState>,
    Extension(current_user): Extension<User>,
    ValidatedRequest(dto): ValidatedRequest<ChangePasswordDto>,
) -> Result<Json<OkMessage>, ApiError> {
    state
        .user_service
        .change_password(dto, current_user.id)
        .await?;
    Ok(Json(OkMessage::default()))
}
