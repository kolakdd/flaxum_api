use crate::config::database::Database;
use crate::config::parameter;
use crate::repository::user_repository::UserRepository;
use crate::repository::user_repository::UserRepositoryTrait;
use crate::service::token_service::{TokenService, TokenServiceTrait};
use crate::service::user_service::UserService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AuthState {
    pub(crate) token_service: TokenService,
    pub(crate) user_repo: UserRepository,
    pub(crate) user_service: UserService,
}

impl AuthState {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            token_service: TokenService::new(parameter::get("JWT_SECRET")),
            user_service: UserService::new(db_conn),
            user_repo: UserRepository::new(db_conn),
        }
    }
}
