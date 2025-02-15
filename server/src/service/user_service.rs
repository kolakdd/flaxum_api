use crate::config::database::Database;
use crate::dto::user::{CreateUserDto, CreateUserOut};
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::error::db_error::DbError;
use crate::error::user_error::UserError;
use crate::repository::user_repository::{UserRepository, UserRepositoryTrait};
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
                let user = self.user_repo.create_user(payload).await;

                return match user {
                    // todo: change create_user out like this
                    // Ok(user) => Ok(RegisterUserOut::from(user)),
                    Ok(user) => Ok(user),
                    Err(e) => match e {
                        SqlxError::Database(e) => match e.code() {
                            Some(code) => {
                                if code == "23000" {
                                    Err(DbError::UniqueConstraintViolation(e.to_string()))?
                                } else {
                                    Err(DbError::SomethingWentWrong(e.to_string()))?
                                }
                            }
                            _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                        },
                        _ => Err(DbError::SomethingWentWrong(e.to_string()))?,
                    },
                };
            }
        };
    }

    pub async fn verify_password(&self, user: &User, password: &str) -> bool {
        crypto::verify(password.to_string(), user.hash_password.to_string())
            .await
            .unwrap()
    }
}
