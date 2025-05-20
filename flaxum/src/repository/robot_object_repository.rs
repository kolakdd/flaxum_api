use std::sync::Arc;

use crate::{
    config::database::{Database, DatabaseTrait},
    db::pagination_query_builder,
    dto::object::GetObjectListDto,
    entity::object::{Object, ObjectCreateModel, ObjectsPaginated},
    entity::pagination::Pagination,
    scalar::Id,
};
use chrono::Utc;

use sqlx::Error as SqlxError;
use sqlx::{self, Postgres, QueryBuilder, Row, Transaction};


#[derive(Clone)]
pub struct RobotObjectRepository {
    pub(crate) db_conn: Arc<Database>,
}

pub trait RobotObjectRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;

}

impl RobotObjectRepositoryTrait for RobotObjectRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }
}
