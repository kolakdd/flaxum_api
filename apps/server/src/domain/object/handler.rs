use crate::domain::object::model::Object;
use crate::utils::jwt::USER;
use crate::{route::AppState, scalar::Id};
use axum::extract::{Multipart, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use actix_web::web::head;
use uuid::Uuid;
use validator::Validate;

#[derive(Deserialize)]
pub struct Pagination {
    limit: usize,
    offset: usize,
}

impl Default for Pagination {
    fn default() -> Self {
        Self {
            limit: 20,
            offset: 0,
        }
    }
}

pub async fn get_own_list(
    State(state): State<Arc<AppState>>,
    pagination: Option<Query<Pagination>>,
) -> impl IntoResponse {
    let Query(pagination) = pagination.unwrap_or_default();
    let current_user = USER.with(|u| u.clone());

    let res = sqlx::query_as!(
        Object,
        "SELECT *
        FROM public.objects
        where owner_id = $3
        limit $1
        offset $2;",
        pagination.limit as i64,
        pagination.offset as i64,
        current_user.id
    )
    .fetch_all(&state.db)
    .await;

    match res {
        Ok(object_list) => Ok((
            StatusCode::OK,
            Json(
                json!({ "status": "success", "data": object_list, "limit": pagination.limit , "offset": pagination.offset }),
            ),
        )),
        Err(_) => Err((StatusCode::NOT_FOUND, "Fail get list".to_string())),
    }
}

pub async fn get_info() {}

pub async fn update_info() {}

pub async fn delete_object() {}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateFolderDto {
    #[validate(length(min = 1, max = 128))]
    pub name: String,
    pub parent_id: Option<Uuid>,
}

pub async fn create_folder(
    State(state): State<Arc<AppState>>,
    Json(body): Json<CreateFolderDto>,
) -> impl IntoResponse {
    if let Err(e) = body.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }
    let current_user = USER.with(|u| u.clone());

    if body.parent_id.is_none() {}
    let res = sqlx::query_as!(
        Object,
        "INSERT INTO objects (id, parent_id, name, size, owner_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        Id::new_v4(),
        body.parent_id,
        body.name,
        0,
        current_user.id,
    )
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(object) => Ok((
            StatusCode::CREATED,
            Json(json!({ "status": "success", "data": object})),
        )),
        Err(_) => Err((StatusCode::NOT_FOUND, "Failed to create folder".to_string())),
    }
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UploadFileDto {
    #[validate(length(min = 1, max = 128))]
    pub parent_id: Option<Uuid>,
}

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    todo!();
    let current_user = USER.with(|u| u.clone());

    let mut default_dto = UploadFileDto{parent_id: None};

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        println!("Length of {} is {} bytes", file_name ,data.len());
    }

    let res = sqlx::query_as!(
        Object,
        "INSERT INTO objects (id, parent_id, name, size, owner_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        Id::new_v4(),
        Id::new_v4(),
        &"test",
        1,
        current_user.id,
    )
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(object) => Ok((
            StatusCode::CREATED,
            Json(json!({ "status": "success", "data": object})),
        )),
        Err(_) => Err((StatusCode::NOT_FOUND, "Failed to create folder".to_string())),
    }
}

pub async fn download_file() {}
