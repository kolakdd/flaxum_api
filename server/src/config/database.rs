use sqlx::{Error, Pool, Postgres};

use super::env::EnvironmentVariables;
use std::time::Duration;

use sqlx::{self, postgres::PgPoolOptions};

// pub trait Queryer<'c>: Executor<'c, Database = sqlx::Postgres> {}
// impl<'c> Queryer<'c> for &Pool<Postgres> {}
pub type DB = Pool<Postgres>;

#[derive(Debug, Clone)]
pub struct Database {
    pub pool: DB,
}

impl Database {
    // Init database
    pub async fn init(env_var: &EnvironmentVariables) -> Result<Self, Error> {
        let pool = Self::connect(&env_var.database_url, env_var.database_pool_size).await?;
        let _ = Self::migrate(&pool).await;
        Ok(Database { pool })
    }

    /// Return database pool
    async fn connect(url: &str, pool_size: u32) -> Result<DB, Error> {
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
    async fn migrate(db: &DB) -> Result<(), Error> {
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
