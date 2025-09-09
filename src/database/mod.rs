use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::Path;

pub mod migrations;

pub type DbPool = Pool<Sqlite>;

pub async fn create_connection_pool(database_url: &str) -> Result<DbPool, sqlx::Error> {
    // Create the database file if it doesn't exist
    if let Some(parent) = Path::new(
        database_url
            .strip_prefix("sqlite://")
            .unwrap_or(database_url),
    )
    .parent()
    {
        std::fs::create_dir_all(parent).map_err(sqlx::Error::Io)?;
    }

    let pool = SqlitePool::connect(database_url).await?;
    Ok(pool)
}

pub async fn run_migrations(pool: &DbPool) -> Result<(), sqlx::migrate::MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await?;
    Ok(())
}

pub async fn initialize_database(database_url: &str) -> Result<DbPool, sqlx::Error> {
    let pool = create_connection_pool(database_url).await?;
    run_migrations(&pool)
        .await
        .map_err(|e| sqlx::Error::Migrate(Box::new(e)))?;
    Ok(pool)
}

#[cfg(test)]
pub async fn create_test_db() -> Result<DbPool, sqlx::Error> {
    let pool = SqlitePool::connect(":memory:").await?;
    run_migrations(&pool)
        .await
        .map_err(|e| sqlx::Error::Migrate(Box::new(e)))?;
    Ok(pool)
}
