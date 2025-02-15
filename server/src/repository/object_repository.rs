use std::sync::Arc;

use crate::{
    config::database::{Database, DatabaseTrait},
    db::pagination_query_builder,
    dto::object::GetObjectListDto,
    entity::object::{Object, ObjectCreateModel, ObjectsPaginated},
    entity::pagination::Pagination,
    scalar::Id,
};
use chrono::Utc;

use sqlx::Error as SqlxError;
use sqlx::{self, Postgres, QueryBuilder, Row, Transaction};

#[derive(Clone)]
pub struct ObjectRepository {
    pub(crate) db_conn: Arc<Database>,
}

pub trait ObjectRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;

    async fn select_by_id(&self, id: Id) -> Result<Object, SqlxError>;
    async fn select_own_list(
        &self,
        pagination: Pagination,
        body: GetObjectListDto,
        owner_id: Id,
    ) -> Result<ObjectsPaginated, SqlxError>;
    async fn select_shared_list(
        &self,
        pagination: Pagination,
        body: GetObjectListDto,
        owner_id: Id,
    ) -> Result<ObjectsPaginated, SqlxError>;
    async fn select_trash_list(
        &self,
        pagination: Pagination,
        owner_id: Id,
    ) -> Result<ObjectsPaginated, SqlxError>;

    async fn insert_object(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        create_model: ObjectCreateModel,
    ) -> Result<Object, SqlxError>;

    async fn mark_as_deleted(&self, id: Id) -> Result<Object, SqlxError>;
    async fn mark_as_restored(&self, id: Id) -> Result<Object, SqlxError>;
    async fn mark_as_eliminated(&self, id: Id) -> Result<Object, SqlxError>;
}

impl ObjectRepositoryTrait for ObjectRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn select_by_id(&self, id: Id) -> Result<Object, SqlxError> {
        let q = r#"
        SELECT id, parent_id, owner_id, creator_id, name, size, type AS "type_",
         mimetype, created_at, updated_at, in_trash, eliminated 
         FROM "Object"
        WHERE eliminated is false and id = $1 "#;
        let res = sqlx::query_as::<_, Object>(q)
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await;

        res
    }

    async fn select_own_list(
        &self,
        pagination: Pagination,
        body: GetObjectListDto,
        owner_id: Id,
    ) -> Result<ObjectsPaginated, SqlxError> {
        let mut q = QueryBuilder::new(
            r#"SELECT *, COUNT(*) OVER() as total_count
            FROM "Object"
            WHERE eliminated IS FALSE AND in_trash IS FALSE AND owner_id = "#,
        );
        q.push_bind(owner_id);

        if let Some(parent_id) = body.parent_id {
            q.push(" AND parent_id = ");
            q.push_bind(parent_id);
        } else {
            q.push(" AND parent_id IS NULL ");
        };

        let mut q = pagination_query_builder(q, &pagination);
        let res = q.build().fetch_all(self.db_conn.get_pool()).await?;

        let mut total_count = 0;
        let objects: Vec<Object> = res
            .into_iter()
            .map(|row| {
                total_count = row.get::<i64, _>("total_count");
                Object::from(row)
            })
            .collect();
        Ok(ObjectsPaginated::build(
            objects,
            pagination.limit,
            pagination.offset,
            total_count,
        ))
    }

    async fn select_shared_list(
        &self,
        pagination: Pagination,
        body: GetObjectListDto,
        uxo_owner: Id,
    ) -> Result<ObjectsPaginated, SqlxError> {
        let mut q = QueryBuilder::new(
            r#"
        SELECT * FROM "Object", COUNT(*) OVER() as total_count
        JOIN "UserXObject" ON "Object".id = "UserXObject".object_id
        where "Object".eliminated is false and "Object".in_trash is false 
        and "Object".owner_id != "#,
        );
        q.push_bind(uxo_owner);

        if let Some(parent_id) = body.parent_id {
            q.push(r#" AND "Object".parent_id = "#);
            q.push_bind(parent_id);
        } else {
            q.push(r#" AND "UserXObject".user_id = "#);
            q.push_bind(uxo_owner);
        };
        let mut q = pagination_query_builder(q, &pagination);
        let res = q.build().fetch_all(self.db_conn.get_pool()).await?;

        let mut total_count = 0;
        let objects: Vec<Object> = res
            .into_iter()
            .map(|row| {
                total_count = row.get::<i64, _>("total_count");
                Object::from(row)
            })
            .collect();
        Ok(ObjectsPaginated::build(
            objects,
            pagination.limit,
            pagination.offset,
            total_count,
        ))
    }

    async fn select_trash_list(
        &self,
        pagination: Pagination,
        owner_id: Id,
    ) -> Result<ObjectsPaginated, SqlxError> {
        let mut q = QueryBuilder::new(
            r#"SELECT * FROM "Object", COUNT(*) OVER() as total_count
            where eliminated is false and in_trash is true and owner_id = "#,
        );
        q.push_bind(owner_id);
        let mut q = pagination_query_builder(q, &pagination);
        let res = q.build().fetch_all(self.db_conn.get_pool()).await?;

        let mut total_count = 0;
        let objects: Vec<Object> = res
            .into_iter()
            .map(|row| {
                total_count = row.get::<i64, _>("total_count");
                Object::from(row)
            })
            .collect();
        Ok(ObjectsPaginated::build(
            objects,
            pagination.limit,
            pagination.offset,
            total_count,
        ))
    }

    async fn insert_object(
        &self,
        tx: &mut Transaction<'static, Postgres>,
        create_model: ObjectCreateModel,
    ) -> Result<Object, SqlxError> {
        let q = r#"
    INSERT INTO "Object" 
    (id, parent_id, owner_id, creator_id, name, size, type, mimetype) 
    VALUES 
    ($1, $2, $3, $4, $5, $6, $7, $8) 
    RETURNING 
    id, parent_id, owner_id, creator_id, name, size, type AS "type_", mimetype, created_at, updated_at, in_trash, eliminated
    "#;

        sqlx::query_as::<_, Object>(q)
            .bind(create_model.id)
            .bind(create_model.parent_id)
            .bind(create_model.owner_id)
            .bind(create_model.creator_id)
            .bind(create_model.name)
            .bind(create_model.size)
            .bind(create_model.type_)
            .bind(create_model.mimetype)
            .fetch_one(&mut **tx)
            .await
    }

    async fn mark_as_deleted(&self, id: Id) -> Result<Object, SqlxError> {
        let q = r#"
            UPDATE "Object" SET in_trash = $1, updated_at = $2  
            WHERE id = $3
            RETURNING id, parent_id, owner_id, creator_id, name, size, type AS "type_", 
            mimetype, created_at, updated_at, in_trash, eliminated 
                    "#;

        sqlx::query_as::<_, Object>(q)
            .bind(true)
            .bind(Utc::now().naive_utc())
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await
    }

    async fn mark_as_restored(&self, id: Id) -> Result<Object, SqlxError> {
        let q = r#"
        UPDATE "Object" SET in_trash = $1, updated_at = $2  
        WHERE id = $3
        RETURNING id, parent_id, owner_id, creator_id, name, size, type AS "type_", 
        mimetype, created_at, updated_at, in_trash, eliminated 
      "#;

        sqlx::query_as::<_, Object>(q)
            .bind(false)
            .bind(Utc::now().naive_utc())
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await
    }

    async fn mark_as_eliminated(&self, id: Id) -> Result<Object, SqlxError> {
        let q = r#"
        UPDATE "Object" SET eliminated = $1, updated_at = $2  
        WHERE id = $3
        RETURNING id, parent_id, owner_id, creator_id, name, size, type AS "type_", 
        mimetype, created_at, updated_at, in_trash, eliminated 
      "#;

        sqlx::query_as::<_, Object>(q)
            .bind(true)
            .bind(Utc::now().naive_utc())
            .bind(id)
            .fetch_one(self.db_conn.get_pool())
            .await
    }
}
