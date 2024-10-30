use crate::scalar::Id;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::{postgres::PgRow, Row};
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize, Clone)]
#[allow(non_snake_case)]
pub struct Object {
    pub id: Id,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub size: Option<i64>,
    #[serde(rename = "ownerId")]
    pub owner_id: String,
    #[serde(rename = "createDate")]
    pub create_date: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updateDate")]
    pub update_date: Option<chrono::DateTime<chrono::Utc>>,
}

impl Object {
    pub fn new(
        id: Id,
        parent_id: Option<Uuid>,
        name: String,
        size: Option<i64>,
        owner_id: String,
        create_date: Option<chrono::DateTime<chrono::Utc>>,
        update_date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Object {
        Object {
            id,
            parent_id,
            name,
            size,
            owner_id,
            create_date,
            update_date,
        }
    }
}

impl From<PgRow> for Object {
    fn from(value: PgRow) -> Self {
        Object::new(
            value.get("id"),
            value.get("parent_id"),
            value.get("name"),
            value.get("size"),
            value.get("owner_id"),
            value.get("create_date"),
            value.get("update_date"),
        )
    }
}
