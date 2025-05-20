use crate::config::database::Database;

use crate::repository::user_repository::UserRepositoryTrait;
use crate::repository::robot_repository::RobotRepositoryTrait;
use crate::repository::robot_object_repository::RobotObjectRepositoryTrait;
use crate::service::robot_object_service::{RobotObjectService, RobotObjectServiceTrait};
use crate::service::robot_service::{RobotService, RobotServiceTrait};
use crate::service::token_service::TokenServiceTrait;

use amqprs::connection::Connection as RMQConn;
use aws_sdk_s3::Client as S3Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct RobotState {
    pub(crate) robot_service: RobotService,
    pub(crate) robot_object_service: RobotObjectService,
}

impl RobotState {
    pub fn new(
        db_conn: &Arc<Database>,
        s3_client: &Arc<S3Client>,
        rmq_conn: &Arc<RMQConn>,
    ) -> Self {
        Self {
            robot_service: RobotService::new(db_conn),
            robot_object_service: RobotObjectService::new(db_conn, s3_client, rmq_conn),
        }
    }
}
