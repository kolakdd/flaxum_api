use sqlx::{Error, Pool, Postgres};

use super::env::EnvironmentVariables;
use std::time::Duration;

use sqlx::{self, postgres::PgPoolOptions};

#[derive(Debug, Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

pub trait DatabaseTrait {
    fn init(
        env_var: &EnvironmentVariables,
    ) -> impl std::future::Future<Output = Result<Self, Error>> + Send
    where
        Self: Sized;

    fn get_pool(&self) -> &Pool<Postgres>;
}

impl DatabaseTrait for Database {
    /// Init database
    async fn init(env_var: &EnvironmentVariables) -> Result<Self, Error> {
        let pool = Self::connect(&env_var.database_url, env_var.database_pool_size).await?;
        let _ = Self::migrate(&pool).await;
        Ok(Database { pool })
    }
    /// Get pool
    fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

impl Database {
    /// Return database pool
    async fn connect(url: &str, pool_size: u32) -> Result<Pool<Postgres>, Error> {
        PgPoolOptions::new()
            .max_connections(pool_size)
            .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
            .connect(url)
            .await
            .map_err(|err| {
                tracing::error!("{}", err);
                err.into()
            })
    }

    /// Migrate database
    async fn migrate(db: &Pool<Postgres>) -> Result<(), Error> {
        tracing::info!("Migrating database...");
        match sqlx::migrate!("db/migrations").run(db).await {
            Ok(()) => Ok(()),
            Err(err) => {
                tracing::error!("{}", &err);
                println!("Migrate Error... {err}");
                Err(err)
            }
        }?;
        tracing::info!("Successfully migrated!...");
        Ok(())
    }
}
