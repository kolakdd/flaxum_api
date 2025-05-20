use crate::config::database::Database;
use crate::repository::robot_object_repository::{RobotObjectRepository, RobotObjectRepositoryTrait};
use crate::repository::s3_repository::{S3Repository, S3RepositoryTrait};

use std::sync::Arc;

use amqprs::connection::Connection as RMQConn;
use aws_sdk_s3::Client as S3Client;



#[derive(Clone)]
pub struct RobotObjectService {
    db_conn: Arc<Database>,
    rmq_conn: Arc<RMQConn>,
    robot_object_repo: RobotObjectRepository,
    s3_repo: S3Repository,
}

pub trait RobotObjectServiceTrait{
    fn new(db_conn: &Arc<Database>, s3_conn: &Arc<S3Client>, rmq_conn: &Arc<RMQConn>) -> Self;

    async fn upload_object();
    async fn download_object();
    async fn delete_object();

}


impl RobotObjectServiceTrait for RobotObjectService {
    fn new(db_conn: &Arc<Database>, s3_conn: &Arc<S3Client>, rmq_conn: &Arc<RMQConn>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
            rmq_conn: Arc::clone(rmq_conn),
            robot_object_repo: RobotObjectRepository::new(db_conn),
            s3_repo: S3Repository::new(s3_conn),
        }
    }
    async fn upload_object(){}
    async fn download_object(){}
    async fn delete_object(){}
}

