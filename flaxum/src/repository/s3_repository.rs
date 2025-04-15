use std::sync::Arc;
use std::time::Duration;

use crate::config::parameter;
use crate::entity::object::{DownloadFileUrl, Object};
use aws_sdk_s3::operation::create_multipart_upload::CreateMultipartUploadOutput;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::types::CompletedPart;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::Error as S3Error;

use bytes::Bytes;

const CHUNK_SIZE: u64 = 1024 * 1024 * 8;

#[derive(Clone)]
pub struct S3Repository {
    pub(crate) s3_conn: Arc<S3Client>,
}

pub trait S3RepositoryTrait {
    fn new(s3_conn: &Arc<S3Client>) -> Self;

    async fn generate_presigned_url(&self, obj: Object) -> Result<DownloadFileUrl, S3Error>;
    async fn upload_bytes(&self, obj: &Object, bytes: Vec<u8>) -> Result<(), S3Error>;
    async fn get_bytes(&self, obj: &Object) -> Result<Vec<u8>, S3Error>;
}

impl S3RepositoryTrait for S3Repository {
    fn new(s3_conn: &Arc<S3Client>) -> Self {
        Self {
            s3_conn: Arc::clone(s3_conn),
        }
    }
    async fn get_bytes(&self, obj: &Object) -> Result<Vec<u8>, S3Error> {
        println!("key = {}", format!("{}/{}", obj.owner_id, obj.id));
        println!("bucket  = {}", parameter::get("UPLOAD_MAIN_BUCKET"));

        let obj = self
            .s3_conn
            .get_object()
            .bucket(parameter::get("UPLOAD_MAIN_BUCKET"))
            .key(format!("{}/{}", obj.owner_id, obj.id))
            .send()
            .await
            .unwrap();
        println!("obj = {:?}", obj);

        let bytes = obj.body.collect().await.unwrap().to_vec();
        Ok(bytes)
    }

    async fn upload_bytes(&self, obj: &Object, bytes: Vec<u8>) -> Result<(), S3Error> {
        let size = bytes.len() as u64;
        let chunk_size = CHUNK_SIZE;

        let multipart_upload_res: CreateMultipartUploadOutput = self
            .s3_conn
            .create_multipart_upload()
            .bucket(parameter::get("DOWNLOAD_TEMP_BUCKET"))
            .key(obj.id)
            .send()
            .await?;

        let upload_id = multipart_upload_res.upload_id().unwrap();
        let mut upload_parts: Vec<CompletedPart> = Vec::new();

        for (chunk_index, chunk) in bytes.chunks(chunk_size as usize).enumerate() {
            let part_number = (chunk_index + 1) as i32; // Нумерация с 1
            let chunk_bytes = Bytes::copy_from_slice(chunk); // Конвертируем в Bytes
            let byte_stream = ByteStream::from(chunk_bytes);

            let upload_part_res = self
                .s3_conn
                .upload_part()
                .bucket(parameter::get("DOWNLOAD_TEMP_BUCKET"))
                .key(obj.id)
                .upload_id(upload_id)
                .part_number(part_number)
                .body(byte_stream)
                .send()
                .await?;

            upload_parts.push(
                CompletedPart::builder()
                    .e_tag(upload_part_res.e_tag.unwrap_or_default())
                    .part_number(part_number)
                    .build(),
            );
        }

        self.s3_conn
            .complete_multipart_upload()
            .bucket(parameter::get("DOWNLOAD_TEMP_BUCKET"))
            .key(obj.id)
            .upload_id(upload_id)
            .multipart_upload(
                aws_sdk_s3::types::CompletedMultipartUpload::builder()
                    .set_parts(Some(upload_parts))
                    .build(),
            )
            .send()
            .await?;

        Ok(())
    }

    async fn generate_presigned_url(&self, obj: Object) -> Result<DownloadFileUrl, S3Error> {
        let expires_in: u64 = 900; // 15 min
        let expires_in = Duration::from_secs(expires_in);

        let presigned_request = &self
            .s3_conn
            .get_object()
            .bucket(parameter::get("DOWNLOAD_TEMP_BUCKET"))
            .response_content_disposition(format!("attachment; filename=\"{}\"", obj.name))
            .key(format!("{}", obj.id))
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await?;

        let valid_until = chrono::offset::Local::now() + expires_in;
        let url = presigned_request.uri().to_string();

        let res = DownloadFileUrl::new(url, valid_until);
        Ok(res)
    }
}

fn decode() {}
