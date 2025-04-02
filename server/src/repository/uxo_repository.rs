use crate::{
    config::database::{Database, DatabaseTrait},
    dto::uxo::{DeleteAccessDto, GiveAccessDto},
    entity::object::{PublicUserXObject, UserXObject, UxOAccess},
    scalar::Id,
};
use sqlx::Error as SqlxError;
use sqlx::{self, Postgres, Transaction};
use std::sync::Arc;

#[derive(Clone)]
pub struct UxoRepository {
    pub(crate) db_conn: Arc<Database>,
}

pub trait UxoRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;

    async fn insert_uxo(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        user_id: Id,
        object_id: Id,
        access: UxOAccess,
    ) -> Result<UserXObject, SqlxError>;

    async fn select_object_uxo_list(&self, obj_id: Id)
        -> Result<Vec<PublicUserXObject>, SqlxError>;
    async fn insert_access_by_email(
        &self,
        obj_id: Id,
        access_dto: GiveAccessDto,
    ) -> Result<PublicUserXObject, SqlxError>;

    async fn delete_access_by_user_id(
        &self,
        access_dto: DeleteAccessDto,
    ) -> Result<(), SqlxError>;
}

impl UxoRepositoryTrait for UxoRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn insert_uxo(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        user_id: Id,
        object_id: Id,
        access: UxOAccess,
    ) -> Result<UserXObject, SqlxError> {
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
            .bind(access.can_read)
            .bind(access.can_edit)
            .bind(access.can_delete)
            .fetch_one(&mut **tx)
            .await?;
        Ok(uxo)
    }

    async fn select_object_uxo_list(
        &self,
        obj_id: Id,
    ) -> Result<Vec<PublicUserXObject>, SqlxError> {
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

        sqlx::query_as::<_, PublicUserXObject>(q)
            .bind(obj_id)
            .fetch_all(self.db_conn.get_pool())
            .await
    }

    async fn insert_access_by_email(
        &self,
        obj_id: Id,
        access_dto: GiveAccessDto,
    ) -> Result<PublicUserXObject, SqlxError> {
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

        sqlx::query_as::<_, PublicUserXObject>(q)
            .bind(access_dto.recipient_email)
            .bind(obj_id)
            .bind(access_dto.can_read)
            .bind(access_dto.can_edit)
            .bind(access_dto.can_delete)
            .fetch_one(self.db_conn.get_pool())
            .await
    }


    async fn delete_access_by_user_id(
        &self,
        access_dto: DeleteAccessDto,
    ) -> Result<(), SqlxError>{
        let q = r#"
        DELETE FROM "UserXObject" WHERE user_id = $1 AND object_id = $2
        "#;
        sqlx::query(q)
            .bind(access_dto.recipient_id)
            .bind(access_dto.obj_id)
            .execute(self.db_conn.get_pool())
            .await?;
        Ok(())
    }
}
