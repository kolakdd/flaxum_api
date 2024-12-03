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
