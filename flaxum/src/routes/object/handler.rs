use crate::dto::object::{
    CreateFolderDto, DeleteObjectDto, DownloadFileDto, GetObjectListDto, UploadFileDto,
};
use crate::entity::pagination::Pagination;
use crate::error::api_error::ApiError;
use crate::error::request_error::ValidatedRequest;
use crate::scalar::Id;
use crate::state::object_state::ObjectState;

use axum::{
    extract::{Multipart, State},
    Extension, Json,
};
use axum_extra::extract::{OptionalQuery, Query};

use validator::Validate;

use crate::entity::object::{
    DownloadFileUrl, Object, ObjectCreateModel, ObjectType, ObjectsPaginated,
};
use crate::entity::user::User;

/// Получение собственных объектов
pub async fn get_own_list(
    State(state): State<ObjectState>,
    Extension(current_user): Extension<User>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
    ValidatedRequest(payload): ValidatedRequest<GetObjectListDto>,
) -> Result<Json<ObjectsPaginated>, ApiError> {
    // todo:: вынести пагинацию
    let pagination = pagination.unwrap_or_default();
    pagination.validate().unwrap();

    let res = state
        .object_service
        .get_own_list(pagination, current_user, payload)
        .await?;
    Ok(Json(res))
}

/// Получение доступных объектов
pub async fn get_shared_list(
    State(state): State<ObjectState>,
    Extension(current_user): Extension<User>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
    ValidatedRequest(payload): ValidatedRequest<GetObjectListDto>,
) -> Result<Json<ObjectsPaginated>, ApiError> {
    // todo:: вынести пагинацию
    let pagination = pagination.unwrap_or_default();
    pagination.validate().unwrap();

    let res = state
        .object_service
        .get_shared_list(pagination, current_user, payload)
        .await?;
    Ok(Json(res))
}

/// Получение корзины
pub async fn get_trash_list(
    State(state): State<ObjectState>,
    Extension(current_user): Extension<User>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
) -> Result<Json<ObjectsPaginated>, ApiError> {
    let pagination = pagination.unwrap_or_default();
    pagination.validate().unwrap();

    let res = state
        .object_service
        .get_trash_list(pagination, current_user)
        .await?;
    Ok(Json(res))
}

pub async fn create_own_folder(
    State(state): State<ObjectState>,
    Extension(current_user): Extension<User>,
    ValidatedRequest(payload): ValidatedRequest<CreateFolderDto>,
) -> Result<Json<Object>, ApiError> {
    let object_constructor = ObjectCreateModel {
        id: Id::new_v4(),
        parent_id: payload.parent_id,
        owner_id: current_user.id,
        creator_id: current_user.id,
        name: payload.name,
        size: Some(0i64),
        type_: ObjectType::Dir,
        mimetype: None,
    };

    let res = state
        .object_service
        .create_own_folder(object_constructor)
        .await?;
    Ok(Json(res))
}

pub async fn upload_file(
    State(state): State<ObjectState>,
    Extension(current_user): Extension<User>,
    OptionalQuery(dto_param): OptionalQuery<UploadFileDto>,
    multipart: Multipart,
) -> Result<Json<Object>, ApiError> {
    let parent_id = match dto_param {
        Some(x) => x.parent_id,
        None => None,
    };

    let res = state
        .object_service
        .upload_own_file(multipart, parent_id, current_user.id)
        .await?;
    Ok(Json(res))
}

pub async fn download_file(
    State(state): State<ObjectState>,
    Extension(_): Extension<User>,
    Query(q): Query<DownloadFileDto>,
) -> Result<Json<DownloadFileUrl>, ApiError> {
    let res = state.object_service.download_own_file(q.file_id).await?;
    Ok(Json(res))
}

pub async fn delete_object(
    State(state): State<ObjectState>,
    Extension(_): Extension<User>,
    ValidatedRequest(dto): ValidatedRequest<DeleteObjectDto>,
) -> Result<Json<Object>, ApiError> {
    let res = state.object_service.delete_own_object(dto).await?;
    Ok(Json(res))
}

pub async fn get_info() {}
pub async fn update_info() {}
