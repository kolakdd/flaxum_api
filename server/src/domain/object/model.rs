use crate::scalar::Id;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Clone, Debug, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "objectType", rename_all = "lowercase")]
pub enum ObjectType {
    Dir,
    File,
}

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Object {
    pub id: Id,
    pub parent_id: Option<Uuid>,
    pub owner_id: Id,
    pub creator_id: Id,
    pub name: String,
    pub size: Option<i64>,
    #[serde(rename = "type")]
    pub type_: ObjectType,
    pub mimetype: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub in_trash: bool,
    pub eliminated: bool,
}

pub struct ObjectCreateModel {
    pub id: Id,
    pub parent_id: Option<Uuid>,
    pub owner_id: Id,
    pub creator_id: Id,
    pub name: String,
    pub size: Option<i64>,
    pub type_: ObjectType,
    pub mimetype: Option<String>,
}

#[allow(clippy::too_many_arguments)]
impl Object {
    pub fn new(
        id: Id,
        parent_id: Option<Uuid>,
        owner_id: Id,
        creator_id: Id,
        name: String,
        size: Option<i64>,
        type_: ObjectType,
        mimetype: Option<String>,
        created_at: chrono::NaiveDateTime,
        updated_at: Option<chrono::NaiveDateTime>,
        in_trash: bool,
        eliminated: bool,
    ) -> Object {
        Object {
            id,
            parent_id,
            owner_id,
            creator_id,
            name,
            size,
            type_,
            mimetype,
            created_at,
            updated_at,
            in_trash,
            eliminated,
        }
    }
}

impl From<PgRow> for Object {
    fn from(value: PgRow) -> Self {
        Object::new(
            value.get("id"),
            value.get("parent_id"),
            value.get("owner_id"),
            value.get("creator_id"),
            value.get("name"),
            value.get("size"),
            value.get("type"),
            value.get("mimetype"),
            value.get("created_at"),
            value.get("updated_at"),
            value.get("in_trash"),
            value.get("eliminated"),
        )
    }
}

/// UxOAccess (can_read, can_edit, can_delete);
pub struct UxOAccess(pub bool, pub bool, pub bool);

#[derive(Debug, sqlx::Type, FromRow, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct UserXObject {
    pub user_id: Id,
    pub object_id: Id,
    pub can_read: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[allow(clippy::too_many_arguments)]
impl UserXObject {
    pub fn new(
        user_id: Id,
        object_id: Id,
        can_read: bool,
        can_edit: bool,
        can_delete: bool,
        created_at: chrono::NaiveDateTime,
        updated_at: Option<chrono::NaiveDateTime>,
    ) -> UserXObject {
        UserXObject {
            user_id,
            object_id,
            can_edit,
            can_read,
            can_delete,
            created_at,
            updated_at,
        }
    }
}

impl From<PgRow> for UserXObject {
    fn from(value: PgRow) -> Self {
        UserXObject::new(
            value.get("user_id"),
            value.get("object_id"),
            value.get("can_read"),
            value.get("can_edit"),
            value.get("can_delete"),
            value.get("created_at"),
            value.get("updated_at"),
        )
    }
}

#[derive(Debug, sqlx::Type, FromRow, Serialize, Deserialize, Clone)]
pub struct PublicUserObject {
    owner_id: Id,
    owner_email: String,
}

impl PublicUserObject {
    pub fn new(owner_id: Id, owner_email: String) -> PublicUserObject {
        PublicUserObject {
            owner_id,
            owner_email,
        }
    }
}

/// Для передачи с дополнительными данными в .../access/...
#[derive(sqlx::Type, Debug, FromRow, Serialize, Deserialize, Clone)]
#[allow(non_snake_case)]
pub struct PublicUserXObject {
    #[sqlx(flatten)]
    pub uxo: UserXObject,
    #[sqlx(flatten)]
    pub owner_user: PublicUserObject,
}

#[allow(clippy::too_many_arguments)]
impl PublicUserXObject {
    pub fn new(uxo: UserXObject, owner_user: PublicUserObject) -> PublicUserXObject {
        PublicUserXObject { uxo, owner_user }
    }
}

impl From<PgRow> for PublicUserXObject {
    fn from(value: PgRow) -> Self {
        PublicUserXObject::new(
            UserXObject::new(
                value.get("user_id"),
                value.get("object_id"),
                value.get("can_read"),
                value.get("can_edit"),
                value.get("can_delete"),
                value.get("created_at"),
                value.get("updated_at"),
            ),
            PublicUserObject::new(value.get("owner_id"), value.get("owner_email")),
        )
    }
}
