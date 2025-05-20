use crate::config::database::Database;
use crate::dto::robot::{CreateRobotDto, CreateRobotOut, DeleteRobotDto};
use crate::dto::user::{
    AdminCreateUserDto, AdminCreateUserOut, ChangePasswordDto, CreateUserDto, CreateUserOut,
    UpdateUserMeDto,
};
use crate::entity::pagination::Pagination;
use crate::entity::robot::{Robot, RobotsPaginated};
use crate::entity::user::{AdminUsersPaginated, PublicUser, User};
use crate::error::api_error::ApiError;
use crate::error::user_error::UserError;
use crate::repository::robot_repository::{RobotRepository, RobotRepositoryTrait};
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
use crate::scalar::Id;
use crate::utils::crypto::{self, generate_secret};

use std::sync::Arc;

#[derive(Clone)]
pub struct RobotService {
    robot_repo: RobotRepository,
}

pub trait RobotServiceTrait{
    fn new(db_conn: &Arc<Database>) -> Self;
    async fn admin_register_robot(&self, dto: CreateRobotDto) ->  Result<CreateRobotOut, ApiError>;
    async fn admin_get_robot_list(&self, pagination: Pagination) -> Result<RobotsPaginated, ApiError>;
    async fn admin_delete_robot(&self, dto: DeleteRobotDto) -> Result<(), ApiError>;

} 

impl RobotServiceTrait for RobotService {
    fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            robot_repo: RobotRepository::new(db_conn),
        }
    }

    async fn admin_register_robot(&self, dto: CreateRobotDto) ->  Result<CreateRobotOut, ApiError> {
        // generate token
        let token = generate_secret(64).await;
        // create robot in db with token 
        // let robot = self.robot_repo;
        // return token, name, id 
        todo!();
    }
    
    async fn admin_get_robot_list(&self, pagination: Pagination) -> Result<RobotsPaginated, ApiError>{
        todo!()
    }

    async fn admin_delete_robot(&self, dto: DeleteRobotDto) -> Result<(), ApiError>{
        todo!()
    }
}
