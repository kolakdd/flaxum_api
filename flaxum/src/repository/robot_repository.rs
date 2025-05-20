use crate::config::database::{Database, DatabaseTrait};
use crate::db::pagination_query_builder;
use crate::dto::robot::CreateRobotDto;
use crate::dto::user::{CreateUserDto, CreateUserOut, UpdateUserMeDto};
use crate::entity::pagination::Pagination;
use crate::entity::robot::Robot;
use crate::entity::user::{AdminUser, AdminUsersPaginated, PublicUser, User, UserRole};
use crate::scalar::Id;
use sqlx::Error as SqlxError;
use sqlx::Row;
use sqlx::{self, Postgres, QueryBuilder};
use std::sync::Arc;



#[derive(Clone)]
pub struct RobotRepository {
    pub(crate) db_conn: Arc<Database>,
}

pub trait RobotRepositoryTrait {
    fn new(db_conn: &Arc<Database>) -> Self;
    // async fn create_robot(&self, payload: CreateRobotDto) -> Result<Robot, SqlxError>;
}

impl RobotRepositoryTrait for RobotRepository {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
        }
    }

    
}
