use crate::{config::env::EnvironmentVariables, scalar::Id, utils::crypto};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Copy, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
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
    pub created_at: NaiveDateTime,
    pub updated_at: Option<NaiveDateTime>,
    pub is_deleted: bool,
    pub deleted_at: Option<NaiveDateTime>,
    pub is_blocked: bool,
    pub blocked_at: Option<NaiveDateTime>,
    pub storage_size: i64,
}

impl User {
    pub async fn build_superuser(env_var: &EnvironmentVariables) -> Self {
        Self {
            id: Id::new_v4(),
            name_1: "Admin".to_string(),
            name_2: None,
            name_3: None,
            email: env_var.flaxum_super_user_email.to_string(),
            hash_password: crypto::hash(env_var.flaxum_super_user_password.to_string())
                .await
                .unwrap(),
            role_type: UserRole::Superuser,
            created_at: Utc::now().naive_utc(),
            updated_at: None,
            is_deleted: false,
            deleted_at: None,
            is_blocked: false,
            blocked_at: None,
            storage_size: 0,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicUser {
    pub id: Id,
    pub name_1: String,
    pub name_2: Option<String>,
    pub name_3: Option<String>,
    pub email: String,
    pub role_type: UserRole,
    pub storage_size: i64,
}

impl From<User> for PublicUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name_1: user.name_1,
            name_2: user.name_2,
            name_3: user.name_3,
            email: user.email,
            role_type: user.role_type,
            storage_size: user.storage_size,
        }
    }
}
