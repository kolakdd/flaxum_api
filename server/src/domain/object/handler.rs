use crate::common::Pagination;
use crate::domain::object::model::{Object, ObjectType};
use crate::domain::user::model::User;
use crate::utils::jwt::USER;
use crate::{route::AppState, scalar::Id};
use aws_sdk_s3::presigning::PresigningConfig;
use axum::extract::{Multipart, State};
use axum::response::IntoResponse;
use axum::Json;
use axum_extra::extract::{OptionalQuery, Query};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::postgres::Postgres;
use sqlx::query_builder::QueryBuilder;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
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

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UploadFileDto {
    parent_id: Option<Id>,
}

pub async fn upload_file(
    State(state): State<Arc<AppState>>,
    OptionalQuery(query): OptionalQuery<UploadFileDto>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let current_user = USER.with(|u| u.clone());
    let query = query.unwrap_or_default();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string(); // check "file"
        if field_name != "file" {
            continue;
        }
        let file_name = field.file_name().unwrap().to_string();
        let content_type = field.content_type().unwrap().to_string();
        println!("{} {} {}", field_name, file_name, content_type);

        let data = field.bytes().await.unwrap();
        let file_length = data.len();

        let file_id = Id::new_v4();
        let file_path = &format!("tmp/{}.{}", current_user.id, file_id);

        let mut tx = state.db.begin().await.unwrap();

        let q = r#"
        INSERT INTO "Object" 
        (id, parent_id, owner_id, creator_id, name, size, type) 
        VALUES 
        ($1, $2, $3, $4, $5, $6, $7) 
        RETURNING 
        id, parent_id, owner_id, creator_id, name, size, type AS "type_", mimetype, created_at, updated_at, in_trash, eliminated
        "#;
        let res = sqlx::query_as::<_, Object>(q)
            .bind(file_id)
            .bind(query.parent_id)
            .bind(current_user.id)
            .bind(current_user.id)
            .bind(file_name)
            .bind(file_length as i64)
            .bind(ObjectType::File)
            .fetch_one(&mut *tx)
            .await;

        let mut file = fs::File::create(Path::new(file_path)).unwrap();
        let ans = file.write_all(&data);
        println!("{:?}", ans);

        tx.commit().await.unwrap();

        match res {
            Ok(object) => {
                return Ok((
                    StatusCode::CREATED,
                    Json(json!({ "status": "success", "data": object})),
                ));
            }
            Err(e) => {
                println!("{:?}", e);
                return Err((StatusCode::BAD_REQUEST, "bad req".to_string()));
            }
        };
    }
    Err((
        StatusCode::INTERNAL_SERVER_ERROR,
        "internal error".to_string(),
    ))
}

#[derive(Serialize, Deserialize)]
pub struct DownloadFileDto {
    file_id: Id,
}

pub async fn download_file(
    State(state): State<Arc<AppState>>,
    Query(query): Query<DownloadFileDto>,
) -> impl IntoResponse {
    let current_user = USER.with(|u| u.clone());
    println!("1");
    let q = r#"
    SELECT id, parent_id, owner_id, creator_id, name, size, type AS "type_", mimetype, created_at, updated_at, in_trash, eliminated FROM "Object"
    WHERE id = $1 "#;
    let res = sqlx::query_as::<_, Object>(q)
        .bind(query.file_id)
        .fetch_one(&state.db)
        .await;
    println!("2");

    match res {
        Ok(object) => {
            println!("3");

            let expires_in: u64 = 900; // 15 min
            let expires_in = Duration::from_secs(expires_in);
            let presigned_request = state
                .s3
                .get_object()
                .bucket("objects")
                .key(format!("{}/{}", object.owner_id, object.id))
                .presigned(PresigningConfig::expires_in(expires_in).unwrap())
                .await
                .unwrap();

            let valid_until = chrono::offset::Local::now() + expires_in;
            println!("6");

            Ok((
                StatusCode::CREATED,
                Json(
                    json!({ "status": "success", "data": presigned_request.uri(), "valid_until": valid_until }),
                ),
            ))
        }
        Err(err) => {
        println!("4");

            eprintln!("Database error: {err}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to download file".to_string(),
            ))
        }
    }
}