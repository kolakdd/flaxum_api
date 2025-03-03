use crate::config::database::{Database, DatabaseTrait};
use crate::dto::object::{DeleteObjectDto, GetObjectListDto};
use crate::entity::object::{
    DownloadFileUrl, Object, ObjectCreateModel, ObjectType, ObjectsPaginated, UxOAccess,
};
use crate::entity::pagination::Pagination;
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::repository::object_repository::{ObjectRepository, ObjectRepositoryTrait};
use crate::repository::s3_repository::{S3Repository, S3RepositoryTrait};
use crate::repository::uxo_repository::{UxoRepository, UxoRepositoryTrait};
use crate::scalar::Id;
use aws_sdk_s3::Client as S3Client;
use axum::extract::Multipart;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct ObjectService {
    db_conn: Arc<Database>,
    object_repo: ObjectRepository,
    uxo_repo: UxoRepository,
    s3_repo: S3Repository,
}

// todo: add trait
impl ObjectService {
    pub fn new(db_conn: &Arc<Database>, s3_conn: &Arc<S3Client>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
            object_repo: ObjectRepository::new(db_conn),
            uxo_repo: UxoRepository::new(db_conn),
            s3_repo: S3Repository::new(s3_conn),
        }
    }

    pub async fn get_own_list(
        &self,
        pagination: Pagination,
        current_user: User,
        body: GetObjectListDto,
    ) -> Result<ObjectsPaginated, ApiError> {
        // todo: add check cur user have access to body.parent if Some(body.parent)
        let objects_paginated = self
            .object_repo
            .select_own_list(pagination, body, current_user.id)
            .await?;
        Ok(objects_paginated)
    }
    pub async fn get_shared_list(
        &self,
        pagination: Pagination,
        current_user: User,
        body: GetObjectListDto,
    ) -> Result<ObjectsPaginated, ApiError> {
        // todo: add check cur user have access to body.parent if Some(body.parent)
        let objects_paginated = self
            .object_repo
            .select_shared_list(pagination, body, current_user.id)
            .await?;
        Ok(objects_paginated)
    }

    pub async fn get_trash_list(
        &self,
        pagination: Pagination,
        current_user: User,
    ) -> Result<ObjectsPaginated, ApiError> {
        // todo: add check cur user have access to body.parent if Some(body.parent)
        let objects_paginated = self
            .object_repo
            .select_trash_list(pagination, current_user.id)
            .await?;
        Ok(objects_paginated)
    }

    pub async fn create_own_folder(
        &self,
        obj_constructor: ObjectCreateModel,
    ) -> Result<Object, ApiError> {
        let mut tx = self.db_conn.get_pool().begin().await?;
        let new_obj: Object = self
            .object_repo
            .insert_object(&mut tx, obj_constructor)
            .await?;
        let new_uxo = self
            .uxo_repo
            .insert_uxo(&mut tx, new_obj.owner_id, new_obj.id, UxOAccess::owner())
            .await?;
        tx.commit().await.unwrap();
        Ok(new_obj)
    }

    pub async fn upload_own_file(
        &self,
        mut multipart: Multipart,
        object_parent: Option<Id>,
        user_id: Id,
    ) -> Result<Object, ApiError> {
        while let Some(multipart_field) = multipart.next_field().await? {
            let field_name = multipart_field.name().unwrap().to_string();
            if field_name.as_str() == "file" {
                let mimetype = multipart_field.content_type().unwrap_or("null").to_string();
                let file_name = multipart_field.file_name().unwrap_or("null").to_string();
                let data = multipart_field.bytes().await.unwrap();

                let mut obj_constructor = ObjectCreateModel::default();

                obj_constructor.id = Id::new_v4();
                obj_constructor.parent_id = object_parent;
                obj_constructor.owner_id = user_id;
                obj_constructor.creator_id = user_id;
                obj_constructor.name = file_name;
                obj_constructor.size = Some(data.len() as i64);
                obj_constructor.type_ = ObjectType::File;
                obj_constructor.mimetype = Some(mimetype);

                let mut tx = self.db_conn.get_pool().begin().await?;

                let new_obj: Object = self
                    .object_repo
                    .insert_object(&mut tx, obj_constructor)
                    .await?;
                let new_uxo = self
                    .uxo_repo
                    .insert_uxo(&mut tx, new_obj.owner_id, new_obj.id, UxOAccess::owner())
                    .await?;

                let file_path = &format!("tmp/{}.{}", new_obj.creator_id, new_obj.id);
                let mut file = fs::File::create(Path::new(file_path))?;
                file.write_all(&data)?;

                tx.commit().await?;

                return Ok(new_obj);
            }
        }
        //todo: change to bad req
        Err(ApiError::UserError(
            crate::error::user_error::UserError::InvalidPassword,
        ))
    }

    pub async fn delete_own_object(&self, dto: DeleteObjectDto) -> Result<Object, ApiError> {
        //todo: add check access
        let res = match dto.hard_delete {
            true => self.object_repo.mark_as_eliminated(dto.file_id).await?,
            false => match dto.delete_mark {
                true => self.object_repo.mark_as_deleted(dto.file_id).await?,
                false => self.object_repo.mark_as_restored(dto.file_id).await?,
            },
        };
        Ok(res)
    }

    pub async fn download_own_file(&self, id: Id) -> Result<DownloadFileUrl, ApiError> {
        let obj = self.object_repo.select_by_id(id).await?;
        let res = self.s3_repo.generate_presigned_url(obj).await?;
        Ok(res)
    }



    pub async fn admin_get_object_list(&self, pagination: Pagination) -> Result<ObjectsPaginated, ApiError>{
        let objects_paginated = self
        .object_repo
        .select_list(pagination)
        .await?;
        Ok(objects_paginated)
    }
}
