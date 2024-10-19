use crate::Error;
use dotenv;
use serde::{Deserialize, Serialize};
use url::Url;

const ENV_DATABASE_URL: &str = "DATABASE_URL";
const ENV_DATABASE_POOL_SIZE: &str = "DATABASE_POOL_SIZE";
const POSTGRES_SCHEME: &str = "postgres";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub database: Database,
}

/// Database contains the data necessary to connect to a database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub url: String,
    pub pool_size: u32,
}
const DEFAULT_DATABASE_POOL_SIZE: u32 = 100;

impl Config {
    /// Load and validate the configuration from the environment.
    /// If an error is found while parsing the values, or validating the data, an error is returned.
    pub fn load() -> Result<Self, Error> {
        dotenv::dotenv().ok();

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

        let mut config = Self { database };

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
