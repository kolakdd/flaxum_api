use crate::dto::uxo::DeleteAccessDtoIn;
use crate::dto::uxo::GiveAccessDto;
use crate::entity::object::GetUxoListOut;
use crate::entity::object::PublicUserXObject;
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
use crate::response::api_response::OkMessage;
use crate::scalar::Id;
use crate::state::object_state::ObjectState;

use axum::extract::Path;
use axum::extract::State;
use axum::Extension;
use axum::Json;

/// Список доступов к файлу
pub async fn list_access(
    State(state): State<ObjectState>,
    Extension(_): Extension<User>,
    Path(object_id): Path<Id>,
) -> Result<Json<GetUxoListOut>, ApiError> {
    let res = state.uxo_service.get_object_uxo_list(object_id).await?;
    Ok(Json(res))
}

/// Дать доступ пользователю
pub async fn post_give_access(
    State(state): State<ObjectState>,
    Extension(_): Extension<User>,
    Path(object_id): Path<Id>,
    ValidatedRequest(payload): ValidatedRequest<GiveAccessDto>,
) -> Result<Json<PublicUserXObject>, ApiError> {
    let res = state
        .uxo_service
        .give_access_by_email(object_id, payload)
        .await?;
    Ok(Json(res))
}

/// Забрать доступ
pub async fn close_access(
    State(state): State<ObjectState>,
    Extension(current_user): Extension<User>,
    Path(object_id): Path<Id>,
    ValidatedRequest(dto): ValidatedRequest<DeleteAccessDtoIn>,
) -> Result<Json<OkMessage>, ApiError> {
    state
        .uxo_service
        .remove_access_by_user_id(current_user.id, object_id, dto)
        .await?;
    Ok(Json(OkMessage::default()))
}
