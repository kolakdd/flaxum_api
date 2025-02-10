use crate::domain::object::model::{PublicUserXObject, UserXObject, UxOAccess};
use crate::domain::uxo::handler::GiveAccessDto;
use crate::scalar::Id;
use http::StatusCode;
use sqlx::{Postgres, Transaction};

/// Создать UserXObject к объекту, для пользователя,
/// с заданными правами.
pub async fn create_uxo(
    user_id: Id,
    object_id: Id,
    access: UxOAccess,
    tx: &mut Transaction<'static, Postgres>,
) -> Result<UserXObject, (StatusCode, String)> {
    let q = r#"
    INSERT INTO "UserXObject" 
    (user_id, object_id, can_read, can_edit, can_delete) 
    VALUES 
    ($1, $2, $3, $4, $5) 
    RETURNING 
    user_id, object_id, can_read, can_edit, can_delete, created_at, updated_at
    "#;

    let uxo = sqlx::query_as::<_, UserXObject>(q)
        .bind(user_id)
        .bind(object_id)
        .bind(access.0)
        .bind(access.1)
        .bind(access.2)
        .fetch_one(&mut **tx)
        .await;
    match uxo {
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

/// Получить все UserXObject у объекта.
pub async fn get_all_uxo(
    object_id: Id,
    db_pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<Vec<PublicUserXObject>, String> {
    let q = r#"
        SELECT
            "UserXObject".user_id,
            "UserXObject".object_id,
            "UserXObject".can_read,
            "UserXObject".can_edit,
            "UserXObject".can_delete,
            "UserXObject".created_at,
            "UserXObject".updated_at,
            "User".id AS "owner_id",
            "User".email AS "owner_email"
        FROM "UserXObject"
        JOIN "User" ON "UserXObject".user_id = "User".id
        WHERE object_id = $1;
    "#;
    let uxo = sqlx::query_as::<_, PublicUserXObject>(q)
        .bind(object_id)
        .fetch_all(db_pool)
        .await;
    match uxo {
        Ok(uxo_ok) => Ok(uxo_ok),
        Err(e) => Err(e.to_string()),
    }
}

/// Дать доступ на объект по email.
pub async fn give_access(
    object_id: Id,
    access_dto: GiveAccessDto,
    db_pool: &sqlx::Pool<sqlx::Postgres>,
) -> Result<PublicUserXObject, String> {
    println!("lolka");
    let q = r#"
    WITH inserted AS (
        INSERT INTO "UserXObject" 
        (user_id, object_id, can_read, can_edit, can_delete) 
        VALUES 
        ((SELECT id FROM "User" WHERE email = $1), $2, $3, $4, $5) 
        RETURNING user_id, object_id, can_read, can_edit, can_delete, created_at, updated_at
    )
    SELECT 
        inserted.user_id,
        inserted.object_id,
        inserted.can_read,
        inserted.can_edit,
        inserted.can_delete,
        inserted.created_at,
        inserted.updated_at,
        "User".id AS "owner_id",
        "User".email AS "owner_email"
    FROM inserted
    JOIN "User" ON inserted.user_id = "User".id;
    "#;
    let uxo = sqlx::query_as::<_, PublicUserXObject>(q)
        .bind(access_dto.recipient_email)
        .bind(object_id)
        .bind(access_dto.can_read)
        .bind(access_dto.can_edit)
        .bind(access_dto.can_delete)
        .fetch_one(db_pool)
        .await;
    match uxo {
        Ok(uxo_ok) => Ok(uxo_ok),
        Err(e) => {
            println!("{:?}", e);
            Err(e.to_string())
        }
    }
}
