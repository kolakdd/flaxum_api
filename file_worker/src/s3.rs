use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, config::Region};

use super::env::EnvironmentVariables;

const AWS_REGION: &str = "eu-central-1";

pub(crate) struct S3Client {
    pub(crate) s3_client: Client,
}

impl S3Client {
    pub async fn init(env_var: &EnvironmentVariables) -> Client {
        let region_provider = RegionProviderChain::first_try(Region::new(AWS_REGION));
        let shared_config = aws_config::from_env()
            .endpoint_url(env_var.minio_url.clone())
            .region(region_provider)
            .load()
            .await;

        Client::new(&shared_config)
    }
}
