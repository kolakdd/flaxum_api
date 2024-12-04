use crate::db::crud::userxobjects::get_all_uxo;
use crate::db::crud::userxobjects::give_access;
use crate::utils::jwt::USER;
use crate::{route::AppState, scalar::Id};
use axum::extract::Path;
use axum::extract::State;
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use validator::Validate;

/// Список доступов к файлу
pub async fn list_access(
    State(state): State<Arc<AppState>>,
    Path(object_id): Path<Id>,
) -> impl IntoResponse {
    let _ = USER.with(|u| u.clone());
    let object_res = get_all_uxo(object_id, &state.db).await;
    match object_res {
        Ok(object) => Ok((
            StatusCode::OK,
            Json(json!({ "status": "success", "data": object})),
        )),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

#[derive(Validate, Serialize, Deserialize)]
pub struct GiveAccessDto {
    pub can_read: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    #[validate(email)]
    pub recipient_email: String,
}
/// Дать доступ пользователю
pub async fn post_give_access(
    State(state): State<Arc<AppState>>,
    Path(object_id): Path<Id>,
    axum::extract::Json(dto): axum::extract::Json<GiveAccessDto>,
) -> impl IntoResponse {
    if let Err(e) = dto.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }
    let _ = USER.with(|u| u.clone());

    let object_res = give_access(object_id, dto, &state.db).await;
    match object_res {
        Ok(object) => Ok((
            StatusCode::OK,
            Json(json!({ "status": "success", "data": object})),
        )),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, err)),
    }
}

/// Забрать доступ
pub async fn close_access() {}

///Список объектов которыми поделились
pub async fn shared_object_list() {}
