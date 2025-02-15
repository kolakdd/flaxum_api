use std::sync::Arc;
use std::time::Duration;

use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::Client as S3Client;
use aws_sdk_s3::Error as S3Error;

use crate::config::parameter;
use crate::entity::object::{DownloadFileUrl, Object};

#[derive(Clone)]
pub struct S3Repository {
    pub(crate) s3_conn: Arc<S3Client>,
}

pub trait S3RepositoryTrait {
    fn new(s3_conn: &Arc<S3Client>) -> Self;

    async fn generate_presigned_url(&self, obj: Object) -> Result<DownloadFileUrl, S3Error>;
}

impl S3RepositoryTrait for S3Repository {
    fn new(s3_conn: &Arc<S3Client>) -> Self {
        Self {
            s3_conn: Arc::clone(s3_conn),
        }
    }

    async fn generate_presigned_url(&self, obj: Object) -> Result<DownloadFileUrl, S3Error> {
        let expires_in: u64 = 900; // 15 min
        let expires_in = Duration::from_secs(expires_in);

        // todo: add presignerError handler
        let presigned_request = &self
            .s3_conn
            .get_object()
            .bucket(parameter::get("UPLOAD_MAIN_BUCKET"))
            .response_content_disposition(format!("attachment; filename=\"{}\"", obj.name))
            .key(format!("{}/{}", obj.owner_id, obj.id))
            .presigned(PresigningConfig::expires_in(expires_in).unwrap())
            .await?;

        let valid_until = chrono::offset::Local::now() + expires_in;
        let url = presigned_request.uri().to_string();

        let res = DownloadFileUrl::new(url, valid_until);
        Ok(res)
    }
}
