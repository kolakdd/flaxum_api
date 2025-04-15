use crate::scalar::Id;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgRow, FromRow, Row};
use uuid::Uuid;

#[derive(Clone, Debug, sqlx::Type, Deserialize, Serialize, Default)]
#[sqlx(type_name = "objectType", rename_all = "lowercase")]
pub enum ObjectType {
    Dir,
    #[default]
    File,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ObjectsPaginated {
    items: Vec<Object>,
    limit: i64,
    offset: i64,
    total: i64,
}

impl ObjectsPaginated {
    pub fn build(items: Vec<Object>, limit: i64, offset: i64, total: i64) -> Self {
        Self {
            items,
            limit,
            offset,
            total,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
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

#[allow(clippy::too_many_arguments)]
impl Object {
    pub fn new(
        id: Id,
        parent_id: Option<Id>,
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

#[derive(Debug, Clone, Default)]
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

#[derive(Debug, Clone)]
pub struct UxOAccess {
    pub can_read: bool,
    pub can_edit: bool,
    pub can_delete: bool,
}

impl UxOAccess {
    pub fn owner() -> Self {
        Self {
            can_read: true,
            can_edit: true,
            can_delete: true,
        }
    }
}

#[derive(Debug, sqlx::Type, sqlx::FromRow, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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

#[derive(Debug, sqlx::Type, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
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
#[derive(sqlx::Type, Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PublicUserXObject {
    #[sqlx(flatten)]
    pub uxo: UserXObject,
    #[sqlx(flatten)]
    pub owner_user: PublicUserObject,
}

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

impl<'r> FromRow<'r, PgRow> for PublicUserXObject {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        Ok(PublicUserXObject::new(
            UserXObject::new(
                row.get("user_id"),
                row.get("object_id"),
                row.get("can_read"),
                row.get("can_edit"),
                row.get("can_delete"),
                row.get("created_at"),
                row.get("updated_at"),
            ),
            PublicUserObject::new(row.get("owner_id"), row.get("owner_email")),
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetUxoListOut {
    pub items: Vec<PublicUserXObject>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DownloadFileUrl {
    url: String,
    valid_until: DateTime<Local>,
}

impl DownloadFileUrl {
    pub fn new(url: String, valid_until: DateTime<Local>) -> Self {
        DownloadFileUrl { url, valid_until }
    }
}
