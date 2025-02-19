use crate::config::database::{Database, DatabaseTrait};
use crate::dto::user::{CreateUserDto, CreateUserOut};
use crate::entity::user::{User, UserRole};
use crate::scalar::Id;
use sqlx;
use sqlx::Error as SqlxError;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserRepository {
    pub(crate) db_conn: Arc<Database>,
}

pub trait UserRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn insert(&self, payload: User) -> Result<(), SqlxError>;
    async fn create_user(&self, payload: CreateUserDto) -> Result<CreateUserOut, SqlxError>;
    // async fn update_user;
    // async fn delete_user;

    // async fn select_by_id(&self, id: u64) -> Result<User, SqlxError>;
    async fn select_by_email(&self, email: String) -> Option<User>;
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
}
