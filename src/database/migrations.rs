// This module handles database migration utilities
// The actual migrations are stored in the migrations/ directory
// and are run automatically when the database is initialized

use crate::database::DbPool;
use sqlx::migrate::MigrateError;

/// Run all pending migrations
#[allow(dead_code)]
pub async fn migrate(pool: &DbPool) -> Result<(), MigrateError> {
    sqlx::migrate!("./migrations").run(pool).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::create_test_db;

    #[tokio::test]
    async fn test_migrations() {
        let pool = create_test_db()
            .await
            .expect("Failed to create test database");

        // Test that migrations can be run without error
        let result = migrate(&pool).await;
        assert!(result.is_ok(), "Migrations should run successfully");
    }
}
