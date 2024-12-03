use crate::common::Pagination;
use crate::domain::object::model::{Object, ObjectType};
use crate::domain::user::model::User;
use crate::utils::jwt::USER;
use crate::{route::AppState, scalar::Id};
use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::OptionalQuery;
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::Postgres;
use sqlx::query_builder::QueryBuilder;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;
use validator::Validate;

#[derive(Serialize, Deserialize, Default)]
pub struct GetOwnListDto {
    /// None при получении из корневой папки.
    /// Иначе из любой собственной папки
    pub parent_id: Option<Uuid>,
}

fn get_own_list_query<'a>(
    current_user: User,
    body: GetOwnListDto,
    pagination: &Pagination,
) -> QueryBuilder<'a, Postgres> {
    let mut query = QueryBuilder::new(
        r#"SELECT *
        FROM "Object"
        where owner_id = "#,
    );
    query.push_bind(current_user.id);

    if let Some(parent_id) = body.parent_id {
        query.push("AND parent_id = ");
        query.push_bind(parent_id);
    };

    query.push(" limit ");
    query.push_bind(pagination.limit);

    query.push(" offset ");
    query.push_bind(pagination.offset);

    query
}

/// Получение собственных объектов
pub async fn get_own_list(
    State(state): State<Arc<AppState>>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
    payload: Option<Json<GetOwnListDto>>,
) -> impl IntoResponse {
    let pagination = pagination.unwrap_or_default();
    if let Err(e) = pagination.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }
    let body: GetOwnListDto = payload.unwrap_or_default().0;
    let current_user = USER.with(|u| u.clone());

    let mut q = get_own_list_query(current_user, body, &pagination);

    let res = q.build().fetch_all(&state.db).await;

    let objects: Result<Vec<Object>, sqlx::Error> = res.map(|rows| {
        rows.into_iter()
            .map(Object::from) // Используем реализацию From<PgRow> for Object
            .collect()
    });

    match objects {
        Ok(object_list) => Ok((
            StatusCode::OK,
            Json(
                json!({ "status": "success", "data": object_list, "limit": pagination.limit , "offset": pagination.offset }),
            ),
        )),
        Err(e) => {
            println!("{e}");
            Err((StatusCode::NOT_FOUND, "Fail get list".to_string()))
        }
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

    // if body.parent_id.is_none() {}

    let q = r#"INSERT INTO "Object" (id, parent_id, owner_id, creator_id, name, size, type) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING id, parent_id, owner_id, creator_id, name, size, type AS "type_", mimetype, created_at, updated_at, in_trash, eliminated"#;
    let res = sqlx::query_as::<_, Object>(q)
        .bind(Id::new_v4())
        .bind(body.parent_id)
        .bind(current_user.id)
        .bind(current_user.id)
        .bind(body.name)
        .bind(0)
        .bind(ObjectType::Dir)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(object) => Ok((
            StatusCode::CREATED,
            Json(json!({ "status": "success", "data": object})),
        )),
        Err(e) => {
            println!("{e}");
            Err((StatusCode::NOT_FOUND, "Failed to create folder".to_string()))
        }
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

        if true {
            return Ok(StatusCode::CREATED);
        }
    }
    Err((StatusCode::BAD_REQUEST, "bad req".to_string()))
}
