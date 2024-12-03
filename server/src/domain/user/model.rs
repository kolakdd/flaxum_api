use crate::scalar::Id;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{postgres::PgRow, Row};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "userRoleType", rename_all = "lowercase")]
pub enum UserRole {
    Superuser,
    Admin,
    User,
}


#[derive(Serialize)]
pub struct ExistsOut {
    pub exists: bool,
}

impl<'r> FromRow<'r, PgRow> for ExistsOut {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(ExistsOut {
            exists: row.try_get("exists")?,
        })
    }
}


#[derive(Serialize)]
pub struct CreateUserOut {
    pub email: String,
    pub created_at: chrono::NaiveDateTime,
}

impl<'r> FromRow<'r, PgRow> for CreateUserOut {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(CreateUserOut {
            email: row.try_get("email")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[allow(clippy::too_many_arguments)]
impl CreateUserOut {
    pub fn new(
        email: String,
        created_at: chrono::NaiveDateTime,
    ) -> CreateUserOut {
        CreateUserOut {
            email,
            created_at,
        }
    }
}


#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
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
        created_at: chrono::NaiveDateTime,
        updated_at: Option<chrono::NaiveDateTime>,
        is_deleted: bool,
        deleted_at: Option<chrono::NaiveDateTime>,
        is_blocked: bool,
        blocked_at: Option<chrono::NaiveDateTime>,
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
