use crate::Error;
use anyhow::Result;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{config::Region, Client};
use dotenv;
use url::Url;

pub const SIZE_1GB: usize = 1024 * 1024 * 1024;

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_DATABASE_POOL_SIZE: &str = "DATABASE_POOL_SIZE";
const POSTGRES_SCHEME: &str = "postgres";

const MINIO_URL: &str = "MINIO_URL";
const UPLOAD_MAIN_BUCKET: &str = "UPLOAD_MAIN_BUCKET";

const DEFAULT_DATABASE_POOL_SIZE: u32 = 100;

#[derive(Debug, Clone)]
pub struct Config {
    pub database: Database,
    pub s3_client: Client,
}

#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}

impl Config {
    pub async fn load() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        let database = Self::build_database();
        let s3_client = Self::build_s3().await;

        let mut config = Self {
            database,
            s3_client,
        };
        config.clean_and_validate()?;
        Ok(config)
    }

    fn clean_and_validate(&mut self) -> Result<(), Error> {
        let database_url = Url::parse(&self.database.url).unwrap();
        if database_url.scheme() != POSTGRES_SCHEME {
            return Err(Error::InvalidArgument(String::from(
                "config: database_url is not a valid postgres URL",
            )));
        }
        Ok(())
    }

    fn build_database() -> Database {
        let url = std::env::var(ENV_DATABASE_URL)
            .map_err(|_| env_not_found(ENV_DATABASE_URL))
            .unwrap();
        let pool_size = std::env::var(ENV_DATABASE_POOL_SIZE)
            .ok()
            .map_or(Ok(DEFAULT_DATABASE_POOL_SIZE), |pool_size_str| {
                pool_size_str.parse::<u32>()
            })
            .unwrap();

        Database { url, pool_size }
    }

    async fn build_s3() -> Client {
        let region_provider = RegionProviderChain::first_try(Region::new("eu-central-1"));
        let region = region_provider.region().await.unwrap();

        let url: String = std::env::var(MINIO_URL)
            .map_err(|_| env_not_found(MINIO_URL))
            .unwrap()
            .parse()
            .unwrap();

        let shared_config = aws_config::from_env()
            .endpoint_url(url)
            .region(region_provider)
            .load()
            .await;
        let client = Client::new(&shared_config);
        let bucket_name = std::env::var(UPLOAD_MAIN_BUCKET).unwrap();

        let constraint =
            aws_sdk_s3::types::BucketLocationConstraint::from(region.to_string().as_str());
        let cfg = aws_sdk_s3::types::CreateBucketConfiguration::builder()
            .location_constraint(constraint)
            .build();

        let _ = client
            .create_bucket()
            .create_bucket_configuration(cfg)
            .bucket(bucket_name)
            .send()
            .await;

        client
    }
}

fn env_not_found(var: &str) -> Error {
    Error::NotFound(format!("config: {var} env var not found"))
}
