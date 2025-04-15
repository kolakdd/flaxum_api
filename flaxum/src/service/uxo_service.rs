use std::sync::Arc;

use crate::config::database::Database;
use crate::dto::uxo::{DeleteAccessDto, DeleteAccessDtoIn, GiveAccessDto};
use crate::entity::object::{GetUxoListOut, PublicUserXObject};
use crate::error::api_error::ApiError;
use crate::error::backend_error::BackendError;
use crate::repository::uxo_repository::{UxoRepository, UxoRepositoryTrait};
use crate::scalar::Id;

// todo: add trait
#[derive(Clone)]
pub struct UxoService {
    uxo_repo: UxoRepository,
}

impl UxoService {
    pub fn new(db_conn: &Arc<Database>) -> Self {
        Self {
            uxo_repo: UxoRepository::new(db_conn),
        }
    }

    pub async fn get_object_uxo_list(&self, obj_id: Id) -> Result<GetUxoListOut, ApiError> {
        let res = self.uxo_repo.select_object_uxo_list(obj_id).await?;
        let res = GetUxoListOut { items: res };
        Ok(res)
    }

    pub async fn give_access_by_email(
        &self,
        obj_id: Id,
        dto: GiveAccessDto,
    ) -> Result<PublicUserXObject, ApiError> {
        let res = self.uxo_repo.insert_access_by_email(obj_id, dto).await?;
        Ok(res)
    }

    pub async fn remove_access_by_user_id(
        &self,
        owner_id: Id,
        obj_id: Id,
        dto: DeleteAccessDtoIn,
    ) -> Result<(), ApiError> {
        if owner_id == dto.recipient_id {
            return Err(BackendError::CloseAccessYourSelf)?;
        }
        let dto = DeleteAccessDto {
            obj_id,
            recipient_id: dto.recipient_id,
        };
        self.uxo_repo.delete_access_by_user_id(dto).await?;
        Ok(())
    }
}
