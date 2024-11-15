use crate::Error;
use dotenv;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use url::Url;

pub const SIZE_1GB: usize = 1024 * 1024 * 1024;

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_DATABASE_POOL_SIZE: &str = "DATABASE_POOL_SIZE";
const POSTGRES_SCHEME: &str = "postgres";

const MINIO_ROOT_USER: &str = "MINIO_ROOT_USER";
const MINIO_ROOT_PASSWORD: &str = "MINIO_ROOT_PASSWORD";
const MINIO_URL: &str = "MINIO_URL";
const UPLOAD_MAIN_BUCKET: &str = "UPLOAD_MAIN_BUCKET";

#[derive(Debug, Clone)]
pub struct Config {
    pub database: Database,
    pub s3: S3Client,
}

/// Database contains the data necessary to connect to a database
#[derive(Debug, Clone)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}

const DEFAULT_DATABASE_POOL_SIZE: u32 = 100;

/// Database contains the data necessary to connect to a database
#[derive(Debug, Clone)]
pub struct S3Client {
    pub url: BaseUrl,
    pub static_provider: StaticProvider,
    pub upload_bucket: String,
}

impl Config {
    /// Load and validate the configuration from the environment.
    /// If an error is found while parsing the values, or validating the data, an error is returned.
    pub fn load() -> Result<Self, Error> {
        dotenv::dotenv().ok();
        // load DB
        let database = {
            let url =
                std::env::var(ENV_DATABASE_URL).map_err(|_| env_not_found(ENV_DATABASE_URL))?;
            let pool_size = std::env::var(ENV_DATABASE_POOL_SIZE)
                .ok()
                .map_or(Ok(DEFAULT_DATABASE_POOL_SIZE), |pool_size_str| {
                    pool_size_str.parse::<u32>()
                })?;

            Database { url, pool_size }
        };
        let s3 = {
            let url: BaseUrl = std::env::var(MINIO_URL)
                .map_err(|_| env_not_found(MINIO_URL))?
                .parse()
                .unwrap();
            let static_provider = StaticProvider::new(
                &std::env::var(MINIO_ROOT_USER).map_err(|_| env_not_found(MINIO_ROOT_USER))?,
                &std::env::var(MINIO_ROOT_PASSWORD)
                    .map_err(|_| env_not_found(MINIO_ROOT_PASSWORD))?,
                None,
            );
            let upload_bucket =
                std::env::var(UPLOAD_MAIN_BUCKET).map_err(|_| env_not_found(UPLOAD_MAIN_BUCKET))?;

            S3Client {
                url,
                static_provider,
                upload_bucket,
            }
        };

        // load config
        let mut config = Self { database, s3 };

        config.clean_and_validate()?;

        Ok(config)
    }

    /// Validate env
    fn clean_and_validate(&mut self) -> Result<(), Error> {
        // Database
        let database_url = Url::parse(&self.database.url).unwrap();
        if database_url.scheme() != POSTGRES_SCHEME {
            return Err(Error::InvalidArgument(String::from(
                "config: database_url is not a valid postgres URL",
            )));
        }
        Ok(())
    }
}

fn env_not_found(var: &str) -> Error {
    Error::NotFound(format!("config: {var} env var not found"))
}
