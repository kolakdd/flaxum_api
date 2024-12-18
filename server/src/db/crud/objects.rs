use crate::domain::object::handler::DeleteObjectDto;
use crate::domain::object::model::{Object, ObjectCreateModel};
use crate::scalar::Id;
use chrono::Utc;
use http::StatusCode;
use sqlx::Error;
use sqlx::{Pool, Postgres, Transaction};

/// Create Object row.
pub async fn create_object(
    object_create_model: ObjectCreateModel,
    tx: &mut Transaction<'static, Postgres>,
) -> Result<Object, (StatusCode, String)> {
    let q = r#"
    INSERT INTO "Object" 
    (id, parent_id, owner_id, creator_id, name, size, type, mimetype) 
    VALUES 
    ($1, $2, $3, $4, $5, $6, $7, $8) 
    RETURNING 
    id, parent_id, owner_id, creator_id, name, size, type AS "type_", mimetype, created_at, updated_at, in_trash, eliminated
    "#;

    let new_file = sqlx::query_as::<_, Object>(q)
        .bind(object_create_model.id)
        .bind(object_create_model.parent_id)
        .bind(object_create_model.owner_id)
        .bind(object_create_model.creator_id)
        .bind(object_create_model.name)
        .bind(object_create_model.size)
        .bind(object_create_model.type_)
        .bind(object_create_model.mimetype)
        .fetch_one(&mut **tx)
        .await;
    match new_file {
        Ok(uxo_ok) => Ok(uxo_ok),
        Err(e) => {
            println!("{:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal error".to_string(),
            ))
        }
    }
}

// todo: add geter with check access
pub async fn get_object(
    file_id: Id,
    is_deleted: bool,
    db_pool: &Pool<Postgres>,
) -> Result<Object, Error> {
    let q = r#"
    SELECT id, parent_id, owner_id, creator_id, name, size, type AS "type_",
     mimetype, created_at, updated_at, in_trash, eliminated 
     FROM "Object"
    WHERE id = $1 and is_deleted is $2 where eliminated is false"#;

    sqlx::query_as::<_, Object>(q)
        .bind(file_id)
        .bind(is_deleted)
        .fetch_one(db_pool)
        .await
}

/// Change object's delete flag
pub async fn object_change_delete(
    dto: DeleteObjectDto,
    db_pool: &Pool<Postgres>,
) -> Result<Object, Error> {
    if !dto.hard_delete {
        let q = r#"
        UPDATE "Object" SET in_trash = $1, updated_at = $2  
        WHERE id = $3
        RETURNING id, parent_id, owner_id, creator_id, name, size, type AS "type_", 
        mimetype, created_at, updated_at, in_trash, eliminated 
      "#;

        sqlx::query_as::<_, Object>(q)
            .bind(dto.delete_mark)
            .bind(Utc::now().naive_utc())
            .bind(dto.file_id)
            .fetch_one(db_pool)
            .await
    } else {
        let q = r#"
        UPDATE "Object" SET eliminated = $1, updated_at = $2  
        WHERE id = $3
        RETURNING id, parent_id, owner_id, creator_id, name, size, type AS "type_", 
        mimetype, created_at, updated_at, in_trash, eliminated 
      "#;

        sqlx::query_as::<_, Object>(q)
            .bind(dto.hard_delete)
            .bind(Utc::now().naive_utc())
            .bind(dto.file_id)
            .fetch_one(db_pool)
            .await
    }
}
