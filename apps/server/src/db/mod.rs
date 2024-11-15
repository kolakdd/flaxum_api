use std::time::Duration;

use sqlx::{self, postgres::PgPoolOptions, Executor, Pool, Postgres};

use crate::{config, Error};

pub type DB = Pool<Postgres>;
pub trait Queryer<'c>: Executor<'c, Database = sqlx::Postgres> {}

impl<'c> Queryer<'c> for &Pool<Postgres> {}

pub async fn connect(database: &config::Database) -> Result<DB, Error> {
    PgPoolOptions::new()
        .max_connections(database.pool_size)
        .max_lifetime(Duration::from_secs(30 * 60)) // 30 mins
        .connect(&database.url)
        .await
        .map_err(|err| {
            tracing::error!("{}", err);
            err.into()
        })
}

pub async fn migrate(db: &DB) -> Result<(), Error> {
    println!("Migrating database...");
    match sqlx::migrate!("db/migrations").run(db).await {
        Ok(()) => Ok(()),
        Err(err) => {
            tracing::error!("{}", &err);
            println!("Migrate Error... {err}");
            Err(err)
        }
    }?;
    println!("Successfully migrated!...");

    Ok(())
}
