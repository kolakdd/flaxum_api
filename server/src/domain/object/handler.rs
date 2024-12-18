use crate::common::Pagination;
use crate::db::crud::commons::pagination::pagination_query_builder;
use crate::db::crud::objects::{create_object, object_change_delete};
use crate::db::crud::userxobjects::create_uxo;
use crate::domain::object::model::{Object, ObjectCreateModel, ObjectType, UxOAccess};
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

fn get_own_list_query(
    current_user: User,
    body: GetOwnListDto,
    pagination: &Pagination,
) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        r#"SELECT *
        FROM "Object"
        where eliminated is false and in_trash is false and owner_id = "#,
    );
    query.push_bind(current_user.id);

    if let Some(parent_id) = body.parent_id {
        query.push("AND parent_id = ");
        query.push_bind(parent_id);
    };
    pagination_query_builder(query, pagination)
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

    let objects: Result<Vec<Object>, sqlx::Error> =
        res.map(|rows| rows.into_iter().map(Object::from).collect());

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


fn get_shared_list_query(
    current_user: User,
    body: GetSharedListDto,
    pagination: &Pagination,
) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        r#"
        SELECT * FROM "Object" 
        JOIN "UserXObject" ON "Object".id = "UserXObject".object_id
        where "Object".eliminated is false and "Object".in_trash is false 
        and 
        "Object".owner_id != "#);
    query.push_bind(current_user.id);
    query.push(r#" AND "UserXObject".user_id = "#);
    query.push_bind(current_user.id);

    if let Some(parent_id) = body.parent_id {
        query.push(r#" AND "Object"parent_id = "#);
        query.push_bind(parent_id);
    };
    pagination_query_builder(query, pagination)
}

#[derive(Serialize, Deserialize, Default)]
pub struct GetSharedListDto {
    /// None при получении из корневой папки.
    /// Иначе из любой собственной папки
    pub parent_id: Option<Uuid>,
}


/// Получение доступных объектов
pub async fn get_shared_list(
    State(state): State<Arc<AppState>>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
    payload: Option<Json<GetSharedListDto>>,
) -> impl IntoResponse {
    let pagination = pagination.unwrap_or_default();
    if let Err(e) = pagination.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }
    let body: GetSharedListDto = payload.unwrap_or_default().0;
    let current_user = USER.with(|u| u.clone());

    let mut q = get_shared_list_query(current_user, body, &pagination);

    let res = q.build().fetch_all(&state.db).await;

    let objects: Result<Vec<Object>, sqlx::Error> =
        res.map(|rows| rows.into_iter().map(Object::from).collect());

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

fn get_trash_query(current_user: User, pagination: &Pagination) -> QueryBuilder<Postgres> {
    let mut query = QueryBuilder::new(
        r#"SELECT *
        FROM "Object"
        where eliminated is false and in_trash is true and owner_id = "#,
    );
    query.push_bind(current_user.id);
    pagination_query_builder(query, pagination)
}

/// Получение корзины
pub async fn get_trash_list(
    State(state): State<Arc<AppState>>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
) -> impl IntoResponse {
    let pagination = pagination.unwrap_or_default();
    if let Err(e) = pagination.validate() {
        return Err((StatusCode::BAD_REQUEST, e.to_string()));
    }
    let current_user = USER.with(|u| u.clone());
    let mut q = get_trash_query(current_user, &pagination);
    let res = q.build().fetch_all(&state.db).await;
    let objects: Result<Vec<Object>, sqlx::Error> =
        res.map(|rows| rows.into_iter().map(Object::from).collect());

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

    let mut tx = state.db.begin().await.unwrap();

    let object_constructor = ObjectCreateModel {
        id: Id::new_v4(),
        parent_id: body.parent_id,
        owner_id: current_user.id,
        creator_id: current_user.id,
        name: body.name,
        size: Some(0i64),
        type_: ObjectType::Dir,
        mimetype: None,
    };
    match create_object(object_constructor, &mut tx).await {
        Ok(object) => {
            let uxo = create_uxo(
                current_user.id,
                object.id,
                UxOAccess(true, true, true),
                &mut tx,
            )
            .await;
            match uxo {
                Ok(_) => {}
                Err(e) => {
                    println!("{:?}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Internal error".to_string(),
                    ));
                }
            }
            tx.commit().await.unwrap();
            Ok((
                StatusCode::CREATED,
                Json(json!({ "status": "success", "data": object})),
            ))
        }
        Err(e) => {
            println!("{:?}", e);
            Err((StatusCode::BAD_REQUEST, "bad req".to_string()))
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
        let field_name = field.name().unwrap().to_string();
        if field_name != "file" {
            continue;
        }
        let file_name = field.file_name().unwrap().to_string();
        let mimetype = field.content_type().unwrap().to_string();

        let data = field.bytes().await.unwrap();
        let file_length = data.len();

        let file_id = Id::new_v4();
        let file_path = &format!("tmp/{}.{}", current_user.id, file_id);

        let mut tx = state.db.begin().await.unwrap();

        let object_constructor = ObjectCreateModel {
            id: file_id,
            parent_id: query.parent_id,
            owner_id: current_user.id,
            creator_id: current_user.id,
            name: file_name,
            size: Some(file_length as i64),
            type_: ObjectType::File,
            mimetype: Some(mimetype),
        };

        let ans = match create_object(object_constructor, &mut tx).await {
            Ok(object) => {
                let _ = create_uxo(
                    current_user.id,
                    object.id,
                    UxOAccess(true, true, true),
                    &mut tx,
                )
                .await
                .unwrap();
                tx.commit().await.unwrap();
                Ok((
                    StatusCode::OK,
                    Json(json!({ "status": "success", "data": object})),
                ))
            }
            Err(e) => {
                println!("{:?}", e);
                return Err((StatusCode::BAD_REQUEST, "bad req".to_string()));
            }
        };

        let mut file = fs::File::create(Path::new(file_path)).unwrap();
        let _ = file.write_all(&data);

        return ans;
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
    let _ = USER.with(|u| u.clone());

    let q = r#"
    SELECT id, parent_id, owner_id, creator_id, name, size, type AS "type_",
     mimetype, created_at, updated_at, in_trash, eliminated 
     FROM "Object"
    WHERE eliminated is false and id = $1 "#;
    let res = sqlx::query_as::<_, Object>(q)
        .bind(query.file_id)
        .fetch_one(&state.db)
        .await;

    match res {
        Ok(object) => {
            let expires_in: u64 = 900; // 15 min
            let expires_in = Duration::from_secs(expires_in);
            let presigned_request = state
                .s3
                .get_object()
                .bucket("objects")
                .response_content_disposition(format!("attachment; filename=\"{}\"", object.name))
                .key(format!("{}/{}", object.owner_id, object.id))
                .presigned(PresigningConfig::expires_in(expires_in).unwrap())
                .await
                .unwrap();

            let valid_until = chrono::offset::Local::now() + expires_in;

            Ok((
                StatusCode::OK,
                Json(
                    json!({ "status": "success", "data": presigned_request.uri(), "valid_until": valid_until }),
                ),
            ))
        }
        Err(err) => {
            eprintln!("Database error: {err}");
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to download file".to_string(),
            ))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DeleteObjectDto {
    pub file_id: Id,
    pub delete_mark: bool,
    pub hard_delete: bool,
}

pub async fn delete_object(
    State(state): State<Arc<AppState>>,
    axum::extract::Json(dto): axum::extract::Json<DeleteObjectDto>,
) -> impl IntoResponse {
    let _ = USER.with(|u| u.clone());

    let object_res: Result<Object, sqlx::Error> = object_change_delete(dto, &state.db).await;
    // let object_res = get_object(dto.file_id, false, &state.db).await;

    match object_res {
        Ok(object) => Ok((
            StatusCode::OK,
            Json(json!({ "status": "success", "data": object})),
        )),
        Err(err) => match err {
            sqlx::Error::RowNotFound => Err((StatusCode::NOT_FOUND, "Not found")),
            _ => Err((StatusCode::INTERNAL_SERVER_ERROR, "Internal error")),
        },
    }
}
