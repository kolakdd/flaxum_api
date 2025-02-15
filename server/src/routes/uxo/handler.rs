use crate::dto::uxo::GiveAccessDto;
use crate::entity::object::PublicUserXObject;
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
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
) -> Result<Json<Vec<PublicUserXObject>>, ApiError> {
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
pub async fn close_access() {}

///Список объектов которыми поделились
pub async fn shared_object_list() {}
