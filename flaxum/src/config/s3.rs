use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, Client};

use super::env::EnvironmentVariables;

const AWS_REGION: &str = "eu-central-1";

pub struct S3Client {
    pub s3_client: Client,
}

pub trait S3ClientTrait {}

impl S3Client {
    pub async fn init(env_var: &EnvironmentVariables) -> Client {
        let region_provider = RegionProviderChain::first_try(Region::new(AWS_REGION));
        let shared_config = aws_config::from_env()
            .endpoint_url(env_var.minio_url.clone())
            .region(region_provider)
            .load()
            .await;

        let client = Client::new(&shared_config);
        println!("addres s3 = {:?}", shared_config.endpoint_url());
        let _ = Self::init_buckets(&client, env_var).await;
        client
    }

    // Инициализация бакетов
    async fn init_buckets(client: &Client, env_var: &EnvironmentVariables) -> anyhow::Result<()> {
        let region = client.config().region().unwrap();
        let constraint =
            aws_sdk_s3::types::BucketLocationConstraint::from(region.to_string().as_str());
        let cfg = aws_sdk_s3::types::CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        tracing::info!("Start init buckets");

        // Создание бакета objects
        let created_bucket = client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .bucket(env_var.upload_main_bucket.clone())
            .send()
            .await;

        match created_bucket {
            Ok(bucket) => tracing::info!("Finish with {:?}", bucket.location),
            Err(err) => tracing::warn!("Err create bucket, {:?}", err.into_source()),
        }

        // Создание бакета tmp upload
        let region = client.config().region().unwrap();
        let constraint =
            aws_sdk_s3::types::BucketLocationConstraint::from(region.to_string().as_str());
        let cfg = aws_sdk_s3::types::CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();
        let created_bucket = client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .bucket(env_var.download_tmp_bucket.clone())
            .send()
            .await;

        match created_bucket {
            Ok(bucket) => tracing::info!("Finish with {:?}", bucket.location),
            Err(err) => tracing::warn!("Err create bucket, {:?}", err.into_source()),
        }
        Ok(())
    }
}
