use crate::scalar::Id;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "userRoleType", rename_all = "lowercase")]
pub enum UserRole {
    Superuser,
    Admin,
    User,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct ExistsOut {
    pub exists: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
pub struct User {
    pub id: Id,
    pub name_1: String,
    pub name_2: Option<String>,
    pub name_3: Option<String>,
    pub email: String,
    pub hash_password: String,
    pub role_type: UserRole,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub is_deleted: bool,
    pub deleted_at: Option<chrono::NaiveDateTime>,
    pub is_blocked: bool,
    pub blocked_at: Option<chrono::NaiveDateTime>,
    pub storage_size: i64,
}
