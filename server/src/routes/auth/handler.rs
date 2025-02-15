use crate::dto::token::TokenReadDto;
use crate::dto::user::{CreateUserDto, CreateUserOut, UserLoginDto};
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
use crate::error::user_error::UserError;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::TokenServiceTrait;
use crate::state::auth_state::AuthState;
use axum::{extract::State, Json};

/// Регистрация пользователя по Логину и Паролю
pub async fn register_user(
    State(state): State<AuthState>,
    ValidatedRequest(payload): ValidatedRequest<CreateUserDto>,
) -> Result<Json<CreateUserOut>, ApiError> {
    let created_user = state.user_service.register_user(payload).await?;
    Ok(Json(created_user))
}

/// Авторизация пользователя по Логину и Паролю
pub async fn access_token(
    State(state): State<AuthState>,
    ValidatedRequest(payload): ValidatedRequest<UserLoginDto>,
) -> Result<Json<TokenReadDto>, ApiError> {
    // todo: вынести всё в сервис
    let user = state
        .user_repo
        .select_by_email(payload.email)
        .await
        .ok_or(UserError::UserNotFound)?;
    return match state
        .user_service
        .verify_password(&user, &payload.password)
        .await
    {
        true => Ok(Json(state.token_service.generate_token(user)?)),
        false => Err(UserError::InvalidPassword)?,
    };
}

pub async fn refresh_token() {}
