use crate::config::database::{Database, DatabaseTrait};
use crate::config::rabbitmq::{send_upload_user_event, UploadUserEvent};
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
use aes::cipher::KeyIvInit;
use aws_sdk_s3::Client as S3Client;
use axum::extract::Multipart;
use tokio::fs;
use tokio::io::{AsyncSeekExt, AsyncWriteExt};
use std::io::{Seek, SeekFrom, Write};
use std::sync::Arc;
use tokio::time::Instant;

use aes::Aes256;
use ctr::cipher::StreamCipher;
use ctr::Ctr128BE;
use sha2::{digest::Digest, Sha256};

type Aes256Ctr = Ctr128BE<Aes256>;

use amqprs::connection::Connection as RMQConn;

#[derive(Clone)]
pub struct ObjectService {
    db_conn: Arc<Database>,
    rmq_conn: Arc<RMQConn>,
    object_repo: ObjectRepository,
    uxo_repo: UxoRepository,
    s3_repo: S3Repository,
}

// todo: add trait
impl ObjectService {
    pub fn new(db_conn: &Arc<Database>, s3_conn: &Arc<S3Client>, rmq_conn: &Arc<RMQConn>) -> Self {
        Self {
            db_conn: Arc::clone(db_conn),
            rmq_conn: Arc::clone(rmq_conn),
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
        let _ = self
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
        while let Some(multipart_field) = multipart.next_field().await.map_err(|e| {
            ApiError::BackendError(crate::error::backend_error::BackendError::InternalError(
                format!("Failed to read multipart field: {}", e),
            ))
        })? {
            let field_name = multipart_field.name().ok_or(ApiError::BackendError(
                crate::error::backend_error::BackendError::InternalError(
                    "No field name".to_string(),
                ),
            ))?;

            if field_name == "file" {
                let mimetype = multipart_field
                    .content_type()
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| "application/octet-stream".to_string());

                let file_name = multipart_field
                    .file_name()
                    .ok_or(ApiError::BackendError(
                        crate::error::backend_error::BackendError::InternalError(
                            "No filename".to_string(),
                        ),
                    ))?
                    .to_string();

                let file_id = Id::new_v4();

                let file_path = format!("tmp/{}.{}", user_id, file_id);
                let mut file = fs::File::create(&file_path).await?;
                let mut total_size: usize = 0;

                let mut hasher = Sha256::new();
                let mut stream = multipart_field;
                loop {
                    match stream.chunk().await {
                        Ok(Some(chunk)) => {
                            total_size += chunk.len();
                            file.write_all(&chunk).await?;
                            hasher.update(&chunk); 
                        }
                        Ok(None) => break,
                        Err(e) => {
                            return Err(ApiError::BackendError(
                                crate::error::backend_error::BackendError::InternalError(format!(
                                    "Failed to read file chunk: {}",
                                    e
                                )),
                            ));
                        }
                    }
                }
                tracing::debug!("file uploaded.");

                let mut obj_constructor = ObjectCreateModel::default();
                obj_constructor.id = file_id;
                obj_constructor.parent_id = object_parent;
                obj_constructor.owner_id = user_id;
                obj_constructor.creator_id = user_id;
                obj_constructor.name = file_name;
                obj_constructor.size = Some(total_size as i64);
                obj_constructor.type_ = ObjectType::File;
                obj_constructor.mimetype = Some(mimetype);
                obj_constructor.upload_s3 = Some(false);

                let key = UploadUserEvent::generate_key(32);
                obj_constructor.decode_key = Some(key.clone());
                
                file.seek(SeekFrom::Start(0)).await?;
                println!("seeked");
                let hash_res = hasher.finalize();
                let hash_sha256 = hex::encode(hash_res).to_string();
                println!("hash_sha256 = {}", hash_sha256);
                obj_constructor.hash_sha256 = Some(hash_sha256);


                let mut tx = self.db_conn.get_pool().begin().await?;
                println!("start tx = ");

                let new_obj: Object = self
                    .object_repo
                    .insert_object(&mut tx, obj_constructor)
                    .await?;
                println!("new_obj created ");

                self.uxo_repo
                    .insert_uxo(&mut tx, new_obj.owner_id, new_obj.id, UxOAccess::owner())
                    .await?;
                println!("uxo created ");
        
                let event = UploadUserEvent {
                    user_id: user_id.to_string(),
                    object_id: file_id.to_string(),
                    key: key,
                };
                println!("event registred ");

                tracing::debug!("transaction ready");

                let rmq_con = self.rmq_conn.clone();
                tokio::spawn(async move {
                    send_upload_user_event(event, &rmq_con).await;
                });
                
                tracing::debug!("send_upload_user_event finished");
                tx.commit().await?;
                println!("ALL GOOD");

                return Ok(new_obj);
            }
        }

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
        let data: Vec<u8> = self.s3_repo.get_bytes(&obj).await.unwrap();

        let mut key = [0u8; 32];
        let nonce: [u8; 16] = {
            hex::decode_to_slice(obj.clone().decode_key.unwrap(), &mut key).unwrap();    
            let mut hasher = Sha256::new();
            Digest::update(&mut hasher, key);
            let result = hasher.finalize();
            result[..16].try_into().unwrap()
        };
        let mut cipher = Aes256Ctr::new(&key.into(), &nonce.into());

        let now = Instant::now();
        let mut decoded_data = data;
        cipher.apply_keystream(&mut decoded_data);
        tracing::debug!("Decrupted elapsed: {:.2?}", now.elapsed());

        let mut hasher = Sha256::new();
        hasher.update(&decoded_data);
        let hash_res = hasher.finalize();
        let hash_sha256 = hex::encode(hash_res).to_string();
        println!("new hash = {}, old hash = {}", hash_sha256, &obj.hash_sha256.clone().unwrap().to_string());

        let _ = self.s3_repo.upload_bytes(&obj, decoded_data).await.unwrap();
        let res = self.s3_repo.generate_presigned_url(obj).await?;
        Ok(res)
    }

    pub async fn admin_get_object_list(
        &self,
        pagination: Pagination,
    ) -> Result<ObjectsPaginated, ApiError> {
        let objects_paginated = self.object_repo.select_list(pagination).await?;
        Ok(objects_paginated)
    }
}
