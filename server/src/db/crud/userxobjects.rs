use crate::domain::object::model::{UserXObject, UxOAccess};
use crate::scalar::Id;
use http::StatusCode;
use sqlx::{Postgres, Transaction};

/// Create UserXObject row.
pub async fn create_uxo(
    user_id: Id,
    object_id: Id,
    access: UxOAccess,
    tx: &mut Transaction<'static, Postgres>,
) -> Result<UserXObject, (StatusCode, String)> {
    let q = r#"
    INSERT INTO "UserXObject" 
    (id, user_id, object_id, can_read, can_edit, can_delete) 
    VALUES 
    ($1, $2, $3, $4, $5, $6) 
    RETURNING 
    id, user_id, object_id, can_read, can_edit, can_delete, created_at, updated_at
    "#;

    let uxo = sqlx::query_as::<_, UserXObject>(q)
        .bind(Id::new_v4())
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
