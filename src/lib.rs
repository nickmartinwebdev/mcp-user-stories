//! # User Stories Management System
//!
//! A comprehensive library for managing user stories and their acceptance criteria
//! using SQLx with compile-time checked queries and Cloudflare D1 database support.
//!
//! ## Features
//!
//! - CRUD operations for user stories and acceptance criteria
//! - Compile-time checked SQL queries with SQLx
//! - Database migrations support
//! - Business logic validation
//! - Search and filtering capabilities
//! - Statistics and reporting
//! - Full async/await support
//!
//! ## Example Usage
//!
//! ```rust,no_run
//! use mcp_user_stories::{
//!     database::initialize_database,
//!     models::{CreateUserStoryRequest, CreateAcceptanceCriteriaRequest},
//!     repositories::Repositories,
//!     services::Services,
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Initialize database
//!     let pool = initialize_database("sqlite://./user_stories.db").await?;
//!
//!     // Setup repositories and services
//!     let repositories = Repositories::new(pool);
//!     let services = Services::new(repositories);
//!
//!     // Create a user story
//!     let user_story_request = CreateUserStoryRequest {
//!         id: "US-001".to_string(),
//!         title: "User Login".to_string(),
//!         description: "As a user, I want to login to access my account".to_string(),
//!         persona: "End User".to_string(),
//!     };
//!
//!     let user_story = services.user_stories.create(user_story_request).await?;
//!     println!("Created user story: {}", user_story.title);
//!
//!     Ok(())
//! }
//! ```

pub mod database;
pub mod models;
pub mod repositories;
pub mod services;

// Re-export commonly used types for convenience
pub use database::{initialize_database, DbPool};
pub use models::*;
pub use repositories::Repositories;
pub use services::Services;

/// Result type alias for the library
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::create_test_db;

    #[tokio::test]
    async fn test_full_integration() {
        // Create test database
        let pool = create_test_db()
            .await
            .expect("Failed to create test database");

        // Initialize repositories and services
        let repositories = Repositories::new(pool);
        let services = Services::new(repositories);

        // Create a user story with acceptance criteria
        let user_story_request = CreateUserStoryRequest {
            id: "US-TEST".to_string(),
            title: "Integration Test Story".to_string(),
            description: "Testing the full integration flow".to_string(),
            persona: "Test User".to_string(),
        };

        let criteria_requests = vec![CreateAcceptanceCriteriaRequest {
            id: "AC-TEST-1".to_string(),
            user_story_id: "US-TEST".to_string(),
            description:
                "Given the system is running, when I create a story, then it should succeed"
                    .to_string(),
        }];

        // Test the full flow
        let result = services
            .user_stories
            .create_with_criteria(user_story_request, criteria_requests)
            .await;

        assert!(result.is_ok());
        let story_with_criteria = result.unwrap();
        assert_eq!(story_with_criteria.user_story.id, "US-TEST");
        assert_eq!(story_with_criteria.acceptance_criteria.len(), 1);

        // Test statistics
        let stats = services.user_stories.get_statistics().await.unwrap();
        assert_eq!(stats.total_stories, 1);
        assert_eq!(stats.total_criteria, 1);
    }

    #[tokio::test]
    async fn test_search_functionality() {
        let pool = create_test_db()
            .await
            .expect("Failed to create test database");
        let repositories = Repositories::new(pool);
        let services = Services::new(repositories);

        // Create multiple user stories
        let stories = vec![
            CreateUserStoryRequest {
                id: "US-001".to_string(),
                title: "Login Feature".to_string(),
                description: "User authentication system".to_string(),
                persona: "End User".to_string(),
            },
            CreateUserStoryRequest {
                id: "US-002".to_string(),
                title: "Search Products".to_string(),
                description: "Product search functionality".to_string(),
                persona: "Customer".to_string(),
            },
        ];

        for story_request in stories {
            services.user_stories.create(story_request).await.unwrap();
        }

        // Test search
        let search_results = services.user_stories.search("login").await.unwrap();
        assert_eq!(search_results.len(), 1);
        assert_eq!(search_results[0].id, "US-001");

        // Test persona filtering
        let customer_stories = services
            .user_stories
            .get_by_persona("Customer")
            .await
            .unwrap();
        assert_eq!(customer_stories.len(), 1);
        assert_eq!(customer_stories[0].id, "US-002");
    }
}
