use crate::config::database::Database;
use crate::dto::user::{
    AdminCreateUserDto, AdminCreateUserOut, ChangePasswordDto, CreateUserDto, CreateUserOut,
    UpdateUserMeDto,
};
use crate::entity::user::{PublicUser, User};
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::user_error::UserError;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
use crate::scalar::Id;
use crate::utils::crypto;

use sqlx::Error as SqlxError;
use std::sync::Arc;

// todo: add trait
#[derive(Clone)]
pub struct UserService {
    user_repo: UserRepository,
}

impl UserService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            user_repo: UserRepository::new(db_conn),
        }
    }

    pub async fn admin_register_user(
        &self,
        payload: AdminCreateUserDto,
    ) -> Result<AdminCreateUserOut, ApiError> {
        return match self
            .user_repo
            .select_by_email(payload.email.to_owned())
            .await
        {
            Some(_) => Err(UserError::UserAlreadyExists)?,

            None => {
                let raw_password = crypto::generate_password().await;
                let hash_password = crypto::hash(raw_password.clone()).await.unwrap();
                let creating_user = CreateUserDto {
                    email: payload.email,
                    password: hash_password,
                };
                let user = self.user_repo.create_user(creating_user).await?;

                let user = AdminCreateUserOut {
                    email: user.email,
                    password: raw_password,
                    created_at: user.created_at,
                };
                return Ok(user);
            }
        };
    }

    #[deprecated]
    pub async fn register_user(
        &self,
        mut payload: CreateUserDto,
    ) -> Result<CreateUserOut, ApiError> {
        return match self
            .user_repo
            .select_by_email(payload.email.to_owned())
            .await
        {
            Some(_) => Err(UserError::UserAlreadyExists)?,
            None => {
                //todo: change unwrap
                let hash_password = crypto::hash(payload.password.to_string()).await.unwrap();
                payload.password = hash_password;
                let user = self.user_repo.create_user(payload).await?;
                return Ok(user);
            }
        };
    }

    pub async fn verify_password(&self, user: &User, password: &str) -> bool {
        crypto::verify(password.to_string(), user.hash_password.to_string())
            .await
            .unwrap()
    }

    pub async fn update_user_me(
        &self,
        payload: UpdateUserMeDto,
        user_id: Id,
    ) -> Result<PublicUser, ApiError> {
        Ok(self.user_repo.update_user_me(payload, user_id).await?)
    }

    pub async fn change_password(
        &self,
        payload: ChangePasswordDto,
        user_id: Id,
    ) -> Result<(), ApiError> {
        let hash_password = crypto::hash(payload.new_password).await.unwrap();
        Ok(self
            .user_repo
            .update_password(hash_password, user_id)
            .await?)
    }
}
