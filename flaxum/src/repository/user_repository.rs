use crate::config::database::{Database, DatabaseTrait};
use crate::db::pagination_query_builder;
use crate::dto::user::{CreateUserDto, CreateUserOut, UpdateUserMeDto};
use crate::entity::pagination::Pagination;
use crate::entity::user::{AdminUser, AdminUsersPaginated, PublicUser, User, UserRole};
use crate::scalar::Id;
use sqlx::Error as SqlxError;
use sqlx::Row;
use sqlx::{self, Postgres, QueryBuilder};
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Database>,
}

pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn insert(&self, payload: User) -> Result<(), SqlxError>;
    async fn create_user(&self, payload: CreateUserDto) -> Result<CreateUserOut, SqlxError>;
    // async fn delete_user;

    // async fn select_by_id(&self, id: u64) -> Result<User, SqlxError>;
    async fn select_by_email(&self, email: String) -> Option<User>;
    async fn update_user_me(
        &self,
        payload: UpdateUserMeDto,
        id: Id,
    ) -> Result<PublicUser, SqlxError>;
    async fn update_password(&self, hash_password: String, id: Id) -> Result<(), SqlxError>;

    async fn select_user_list(
        &self,
        pagination: Pagination,
    ) -> Result<AdminUsersPaginated, SqlxError>;
}

impl UserRepositoryTrait for UserRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    async fn insert(&self, payload: User) -> Result<(), SqlxError> {
        let q = r#"
        INSERT INTO "User" (id, name_1, email, hash_password, role_type)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING email, created_at
    "#;
        let _ = sqlx::query_as::<_, CreateUserOut>(q)
            .bind(payload.id)
            .bind(payload.name_1)
            .bind(payload.email)
            .bind(payload.hash_password)
            .bind(payload.role_type)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(())
    }

    async fn create_user(&self, payload: CreateUserDto) -> Result<CreateUserOut, SqlxError> {
        let q = r#"
        INSERT INTO "User" (id, name_1, email, hash_password, role_type)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING email, created_at
    "#;
        let user = sqlx::query_as::<_, CreateUserOut>(q)
            .bind(Id::new_v4())
            .bind("name_1_mock")
            .bind(payload.email.clone())
            .bind(payload.password)
            .bind(UserRole::User)
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(user)
    }

    async fn select_by_email(&self, email: String) -> Option<User> {
        let q = r#" SELECT * FROM "User" WHERE email = $1"#;
        sqlx::query_as::<_, User>(q)
            .bind(email)
            .fetch_optional(self.db_conn.get_pool())
            .await
            .unwrap_or(None)
    }

    async fn select_user_list(
        &self,
        pagination: Pagination,
    ) -> Result<AdminUsersPaginated, SqlxError> {
        let q = QueryBuilder::new(
            r#"
            SELECT *, COUNT(*) OVER() as total_count
            FROM "User" WHERE role_type != 'superuser'
            ORDER BY created_at desc 
            "#,
        );
        let mut q = pagination_query_builder(q, &pagination);
        let res = q.build().fetch_all(self.db_conn.get_pool()).await?;
        let mut total_count = 0;
        let users: Vec<AdminUser> = res
            .into_iter()
            .map(|row| {
                total_count = row.get::<i64, _>("total_count");
                AdminUser::from(row)
            })
            .collect();
        Ok(AdminUsersPaginated::build(
            users,
            pagination.limit,
            pagination.offset,
            total_count,
        ))
    }

    async fn update_user_me(
        &self,
        payload: UpdateUserMeDto,
        id: Id,
    ) -> Result<PublicUser, SqlxError> {
        let mut q: QueryBuilder<'_, Postgres> = QueryBuilder::new(r#"UPDATE "User" SET "#);

        let mut its_first = true;
        if let Some(name_1) = payload.name_1 {
            q.push(" name_1 = ");
            q.push_bind(name_1);
            its_first = false;
        }

        if let Some(name_2) = payload.name_2 {
            if !its_first {
                q.push(", ");
            }
            q.push(" name_2 = ");
            q.push_bind(name_2);
            its_first = false;
        }
        if let Some(name_3) = payload.name_3 {
            if !its_first {
                q.push(", ");
            }
            q.push(" name_3 = ");
            q.push_bind(name_3);
        }
        q.push(r#" WHERE "User".id = "#);
        q.push_bind(id);
        q.push(r#" RETURNING *"#);

        let user = q
            .build_query_as::<User>()
            .fetch_one(self.db_conn.get_pool())
            .await?;

        Ok(PublicUser::from(user))
    }

    async fn update_password(&self, hash_password: String, id: Id) -> Result<(), SqlxError> {
        let mut q: QueryBuilder<'_, Postgres> = QueryBuilder::new(r#"UPDATE "User" SET "#);
        q.push(" hash_password = ");
        q.push_bind(hash_password);
        q.push(r#" WHERE "User".id = "#);
        q.push_bind(id);
        q.push(r#" RETURNING *"#);

        let _ = q
            .build_query_as::<User>()
            .fetch_one(self.db_conn.get_pool())
            .await?;
        Ok(())
    }
}
