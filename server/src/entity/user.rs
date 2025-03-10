use crate::{config::env::EnvironmentVariables, scalar::Id, utils::crypto};
use chrono::{NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::Row;

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
#[serde(rename_all = "camelCase")]
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


#[allow(clippy::too_many_arguments)]
impl User {
    pub fn new(
        id: Id,
        name_1: String,
        name_2: Option<String>,
        name_3: Option<String>,
        email: String,
        hash_password: String,
        role_type: UserRole,
        created_at: NaiveDateTime,
        updated_at: Option<NaiveDateTime>,
        is_deleted: bool,
        deleted_at: Option<NaiveDateTime>,
        is_blocked: bool,
        blocked_at: Option<NaiveDateTime>,
        storage_size: i64,
    ) -> User {
        User {
            id,
            name_1,
            name_2,
            name_3,
            email,
            hash_password,
            role_type,
            created_at,
            updated_at,
            is_deleted,
            deleted_at,
            is_blocked,
            blocked_at,
            storage_size,
        }
    }
}


impl From<PgRow> for User {
    fn from(value: PgRow) -> Self {
        User::new(
            value.get("id"),
            value.get("name_1"),
            value.get("name_2"),
            value.get("name_3"),
            value.get("email"),
            value.get("hash_password"),
            value.get("role_type"),
            value.get("created_at"),
            value.get("updated_at"),
            value.get("is_deleted"),
            value.get("deleted_at"),
            value.get("is_blocked"),
            value.get("blocked_at"),
            value.get("storage_size"),
        )
    }
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

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
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



/// Ифнормация о пользователях для админа
#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct AdminUser {
    id: Id,
    name_1: String,
    name_2: Option<String>,
    name_3: Option<String>,
    email: String,
    role_type: UserRole,
    created_at: NaiveDateTime,
    updated_at: Option<NaiveDateTime>,
    is_deleted: bool,
    deleted_at: Option<NaiveDateTime>,
    is_blocked: bool,
    blocked_at: Option<NaiveDateTime>,
    storage_size: i64,
}

#[allow(clippy::too_many_arguments)]
impl AdminUser {
    pub fn new(
        id: Id,
        name_1: String,
        name_2: Option<String>,
        name_3: Option<String>,
        email: String,
        role_type: UserRole,
        created_at: NaiveDateTime,
        updated_at: Option<NaiveDateTime>,
        is_deleted: bool,
        deleted_at: Option<NaiveDateTime>,
        is_blocked: bool,
        blocked_at: Option<NaiveDateTime>,
        storage_size: i64,
    ) -> AdminUser {
        AdminUser {
            id,
            name_1,
            name_2,
            name_3,
            email,
            role_type,
            created_at,
            updated_at,
            is_deleted,
            deleted_at,
            is_blocked,
            blocked_at,
            storage_size,
        }
    }
}


impl From<PgRow> for AdminUser {
    fn from(value: PgRow) -> Self {
        AdminUser::new(
            value.get("id"),
            value.get("name_1"),
            value.get("name_2"),
            value.get("name_3"),
            value.get("email"),
            value.get("role_type"),
            value.get("created_at"),
            value.get("updated_at"),
            value.get("is_deleted"),
            value.get("deleted_at"),
            value.get("is_blocked"),
            value.get("blocked_at"),
            value.get("storage_size"),
        )
    }
}

impl From<User> for AdminUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name_1: user.name_1,
            name_2: user.name_2,
            name_3: user.name_3,
            email: user.email,
            role_type: user.role_type,
            created_at: user.created_at,
            updated_at: user.updated_at,
            is_deleted: user.is_deleted,
            deleted_at: user.deleted_at,
            is_blocked: user.is_blocked,
            blocked_at: user.blocked_at,
            storage_size: user.storage_size,
        }
    }
}

/// Ифнормация о пользователях для админа
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AdminUsersPaginated {
    items: Vec<AdminUser>,
    limit: i64,
    offset: i64,
    total: i64,
}




impl AdminUsersPaginated {
    pub fn build(items: Vec<AdminUser>, limit: i64, offset: i64, total: i64) -> Self {
        Self {
            items,
            limit,
            offset,
            total,
        }
    }
}