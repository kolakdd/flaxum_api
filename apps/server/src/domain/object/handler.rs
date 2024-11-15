use crate::domain::object::model::Object;
use crate::utils::jwt::USER;
use crate::{route::AppState, scalar::Id};
use axum::extract::{Multipart, Query, State};
use axum::response::IntoResponse;
use axum::Json;
use bytes::Buf;
use futures::TryStreamExt;
use http::StatusCode;
use minio::s3::args::PutObjectArgs;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;
use std::fs;
use tokio::io::{AsyncRead, AsyncWrite, BufWriter};
use tokio_util::io::StreamReader;
use std::io;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct UploadFileDto {
    parent_id: Option<Uuid>,
}

// pub async fn upload_file(
//     State(state): State<Arc<AppState>>,
//     query: Query<UploadFileDto>,
//     mut multipart: Multipart,
// ) -> impl IntoResponse {
//     let current_user = USER.with(|u| u.clone());
//
//     while let Some(field) = multipart.next_field().await.unwrap() {
//         let file_name = field.file_name().unwrap().to_string();
//         let data = field.bytes().await.unwrap();
//
//         let mut tx = state.db.begin().await.unwrap();
//         let new_id = Id::new_v4();
//         let res = sqlx::query_as!(
//         Object,
//         "INSERT INTO objects (id, parent_id, name, size, owner_id) VALUES ($1, $2, $3, $4, $5) RETURNING *",
//             new_id,
//             query.parent_id,
//             &file_name,
//             data.len() as i64,
//             &current_user.id
//     )
//             .fetch_one(&mut *tx)
//             .await;
//         let mut data_buffer = data.reader();
//         let object_name = format!("{}/{}", &current_user.id, new_id);
//         let mut args = PutObjectArgs::new(
//             &state.upload_bucket,
//             &object_name,
//             &mut data_buffer,
//             None,
//             None,
//         )
//         .unwrap();
//         let a = state.s3.put_object(&mut args).await.unwrap();
//         println!("{:#?}", a);
//
//         tx.commit().await.unwrap();
//
//         match res {
//             Ok(object) => {
//                 return Ok((
//                     StatusCode::CREATED,
//                     Json(json!({ "status": "success", "data": object})),
//                 ))
//             }
//             Err(_) => return Err((StatusCode::BAD_REQUEST, "bad req".to_string())),
//         };
//     }
//     Err((StatusCode::BAD_REQUEST, "bad req".to_string()))
// }

pub async fn download_file() {}

pub async fn upload_file(
    // State(state): State<Arc<AppState>>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string(); // check "file"
        let file_name = field.file_name().unwrap().to_string(); 
        let content_type = field.content_type().unwrap().to_string();
        println!("{} {} {}", field_name, file_name, content_type);
        let data = field.bytes().await.unwrap();

        let path_1 = &format!("tmp/{}", Id::new_v4());
        let path = Path::new(path_1);

        let mut file = fs::File::create(path).unwrap();
        let _ = file.write_all(&data);
        
        if true{
            return Ok(StatusCode::CREATED);
        }
    }
    Err((StatusCode::BAD_REQUEST, "bad req".to_string()))
}
