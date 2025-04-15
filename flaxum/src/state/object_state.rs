use crate::config::database::Database;
use crate::config::parameter;

use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};

use crate::service::object_service::ObjectService;
use crate::service::token_service::{TokenService, TokenServiceTrait};
use crate::service::user_service::UserService;
use crate::service::uxo_service::UxoService;

use amqprs::connection::Connection as RMQConn;
use aws_sdk_s3::Client as S3Client;
use std::sync::Arc;

#[derive(Clone)]
pub struct ObjectState {
    pub(crate) user_repo: UserRepository,
    pub(crate) token_service: TokenService,
    pub(crate) user_service: UserService,
    pub(crate) object_service: ObjectService,
    pub(crate) uxo_service: UxoService,
}

impl ObjectState {
    pub fn new(
        db_conn: &Arc<Database>,
        s3_client: &Arc<S3Client>,
        rmq_conn: &Arc<RMQConn>,
    ) -> ObjectState {
        Self {
            user_repo: UserRepository::new(db_conn),
            token_service: TokenService::new(parameter::get("JWT_SECRET")),
            user_service: UserService::new(db_conn),
            object_service: ObjectService::new(db_conn, s3_client, rmq_conn),
            uxo_service: UxoService::new(db_conn),
        }
    }
}
