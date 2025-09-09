use crate::models::{
    CreateAcceptanceCriteriaRequest, CreateUserStoryRequest, UpdateUserStoryRequest, UserStory,
    UserStoryWithCriteria,
};
use crate::repositories::Repositories;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserStoryServiceError {
    #[error("User story not found: {id}")]
    NotFound { id: String },
    #[error("User story already exists: {id}")]
    AlreadyExists { id: String },
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {message}")]
    Validation { message: String },
    #[error("Business rule violation: {message}")]
    BusinessRule { message: String },
}

pub type Result<T> = std::result::Result<T, UserStoryServiceError>;

#[derive(Clone)]
pub struct UserStoryService {
    repositories: Repositories,
}

impl UserStoryService {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }

    /// Create a new user story with validation
    pub async fn create(&self, request: CreateUserStoryRequest) -> Result<UserStory> {
        // Validate the request
        self.validate_create_request(&request)?;

        // Check if user story already exists
        if self
            .repositories
            .user_stories
            .get_by_id(&request.id)
            .await?
            .is_some()
        {
            return Err(UserStoryServiceError::AlreadyExists {
                id: request.id.clone(),
            });
        }

        // Create the user story
        let user_story = self.repositories.user_stories.create(request).await?;

        Ok(user_story)
    }

    /// Create a user story with its acceptance criteria in a transaction-like manner
    pub async fn create_with_criteria(
        &self,
        user_story_request: CreateUserStoryRequest,
        criteria_requests: Vec<CreateAcceptanceCriteriaRequest>,
    ) -> Result<UserStoryWithCriteria> {
        // Validate user story request
        self.validate_create_request(&user_story_request)?;

        // Validate that all criteria belong to this user story
        for criteria in &criteria_requests {
            if criteria.user_story_id != user_story_request.id {
                return Err(UserStoryServiceError::Validation {
                    message: format!(
                        "Acceptance criteria {} does not belong to user story {}",
                        criteria.id, user_story_request.id
                    ),
                });
            }
        }

        // Check if user story already exists
        if self
            .repositories
            .user_stories
            .get_by_id(&user_story_request.id)
            .await?
            .is_some()
        {
            return Err(UserStoryServiceError::AlreadyExists {
                id: user_story_request.id.clone(),
            });
        }

        // Create user story first
        let user_story = self
            .repositories
            .user_stories
            .create(user_story_request)
            .await?;

        // Create acceptance criteria
        let acceptance_criteria = if !criteria_requests.is_empty() {
            self.repositories
                .acceptance_criteria
                .create_batch(criteria_requests)
                .await?
        } else {
            Vec::new()
        };

        Ok(UserStoryWithCriteria {
            user_story,
            acceptance_criteria,
        })
    }

    /// Get user story by ID
    pub async fn get_by_id(&self, id: &str) -> Result<UserStory> {
        self.repositories
            .user_stories
            .get_by_id(id)
            .await?
            .ok_or_else(|| UserStoryServiceError::NotFound { id: id.to_string() })
    }

    /// Get user story with its acceptance criteria
    pub async fn get_with_criteria(&self, id: &str) -> Result<UserStoryWithCriteria> {
        let user_story = self.get_by_id(id).await?;
        let acceptance_criteria = self
            .repositories
            .acceptance_criteria
            .get_by_user_story_id(id)
            .await?;

        Ok(UserStoryWithCriteria {
            user_story,
            acceptance_criteria,
        })
    }

    /// Get all user stories
    pub async fn get_all(&self) -> Result<Vec<UserStory>> {
        Ok(self.repositories.user_stories.get_all().await?)
    }

    /// Get all user stories with their acceptance criteria
    pub async fn get_all_with_criteria(&self) -> Result<Vec<UserStoryWithCriteria>> {
        let user_stories = self.repositories.user_stories.get_all().await?;
        let mut result = Vec::new();

        for user_story in user_stories {
            let acceptance_criteria = self
                .repositories
                .acceptance_criteria
                .get_by_user_story_id(&user_story.id)
                .await?;

            result.push(UserStoryWithCriteria {
                user_story,
                acceptance_criteria,
            });
        }

        Ok(result)
    }

    /// Get user stories with pagination
    pub async fn get_paginated(&self, limit: i64, offset: i64) -> Result<Vec<UserStory>> {
        if limit <= 0 || limit > 100 {
            return Err(UserStoryServiceError::Validation {
                message: "Limit must be between 1 and 100".to_string(),
            });
        }

        if offset < 0 {
            return Err(UserStoryServiceError::Validation {
                message: "Offset must be non-negative".to_string(),
            });
        }

        Ok(self
            .repositories
            .user_stories
            .get_paginated(limit, offset)
            .await?)
    }

    /// Update user story
    pub async fn update(&self, id: &str, request: UpdateUserStoryRequest) -> Result<UserStory> {
        // Validate the update request
        self.validate_update_request(&request)?;

        self.repositories
            .user_stories
            .update(id, request)
            .await?
            .ok_or_else(|| UserStoryServiceError::NotFound { id: id.to_string() })
    }

    /// Delete user story (this will also delete associated acceptance criteria due to CASCADE)
    pub async fn delete(&self, id: &str) -> Result<()> {
        let deleted = self.repositories.user_stories.delete(id).await?;

        if !deleted {
            return Err(UserStoryServiceError::NotFound { id: id.to_string() });
        }

        Ok(())
    }

    /// Search user stories
    pub async fn search(&self, query: &str) -> Result<Vec<UserStory>> {
        if query.trim().is_empty() {
            return Err(UserStoryServiceError::Validation {
                message: "Search query cannot be empty".to_string(),
            });
        }

        Ok(self.repositories.user_stories.search(query).await?)
    }

    /// Get user stories by persona
    pub async fn get_by_persona(&self, persona: &str) -> Result<Vec<UserStory>> {
        if persona.trim().is_empty() {
            return Err(UserStoryServiceError::Validation {
                message: "Persona cannot be empty".to_string(),
            });
        }

        Ok(self
            .repositories
            .user_stories
            .get_by_persona(persona)
            .await?)
    }

    /// Get user stories grouped by persona
    pub async fn get_grouped_by_persona(&self) -> Result<HashMap<String, Vec<UserStory>>> {
        Ok(self
            .repositories
            .user_stories
            .get_grouped_by_persona()
            .await?)
    }

    /// Get statistics about user stories
    pub async fn get_statistics(&self) -> Result<UserStoryStatistics> {
        let total_stories = self.repositories.user_stories.count().await?;
        let total_criteria = self.repositories.acceptance_criteria.count().await?;
        let grouped_by_persona = self.get_grouped_by_persona().await?;

        let personas_count = grouped_by_persona.len() as i64;
        let avg_criteria_per_story = if total_stories > 0 {
            total_criteria as f64 / total_stories as f64
        } else {
            0.0
        };

        Ok(UserStoryStatistics {
            total_stories,
            total_criteria,
            personas_count,
            avg_criteria_per_story,
            stories_by_persona: grouped_by_persona
                .into_iter()
                .map(|(persona, stories)| (persona, stories.len() as i64))
                .collect(),
        })
    }

    /// Validate create request
    fn validate_create_request(&self, request: &CreateUserStoryRequest) -> Result<()> {
        if request.id.trim().is_empty() {
            return Err(UserStoryServiceError::Validation {
                message: "User story ID cannot be empty".to_string(),
            });
        }

        if request.title.trim().is_empty() {
            return Err(UserStoryServiceError::Validation {
                message: "User story title cannot be empty".to_string(),
            });
        }

        if request.description.trim().is_empty() {
            return Err(UserStoryServiceError::Validation {
                message: "User story description cannot be empty".to_string(),
            });
        }

        if request.persona.trim().is_empty() {
            return Err(UserStoryServiceError::Validation {
                message: "User story persona cannot be empty".to_string(),
            });
        }

        // Validate ID format (should follow US-XXX pattern)
        if !request.id.starts_with("US-") {
            return Err(UserStoryServiceError::Validation {
                message: "User story ID should start with 'US-'".to_string(),
            });
        }

        // Validate title length
        if request.title.len() > 200 {
            return Err(UserStoryServiceError::Validation {
                message: "User story title cannot exceed 200 characters".to_string(),
            });
        }

        // Validate description length
        if request.description.len() > 2000 {
            return Err(UserStoryServiceError::Validation {
                message: "User story description cannot exceed 2000 characters".to_string(),
            });
        }

        Ok(())
    }

    /// Validate update request
    fn validate_update_request(&self, request: &UpdateUserStoryRequest) -> Result<()> {
        if let Some(ref title) = request.title {
            if title.trim().is_empty() {
                return Err(UserStoryServiceError::Validation {
                    message: "User story title cannot be empty".to_string(),
                });
            }
            if title.len() > 200 {
                return Err(UserStoryServiceError::Validation {
                    message: "User story title cannot exceed 200 characters".to_string(),
                });
            }
        }

        if let Some(ref description) = request.description {
            if description.trim().is_empty() {
                return Err(UserStoryServiceError::Validation {
                    message: "User story description cannot be empty".to_string(),
                });
            }
            if description.len() > 2000 {
                return Err(UserStoryServiceError::Validation {
                    message: "User story description cannot exceed 2000 characters".to_string(),
                });
            }
        }

        if let Some(ref persona) = request.persona {
            if persona.trim().is_empty() {
                return Err(UserStoryServiceError::Validation {
                    message: "User story persona cannot be empty".to_string(),
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct UserStoryStatistics {
    pub total_stories: i64,
    pub total_criteria: i64,
    pub personas_count: i64,
    pub avg_criteria_per_story: f64,
    pub stories_by_persona: HashMap<String, i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::Repositories;

    fn create_test_request() -> CreateUserStoryRequest {
        CreateUserStoryRequest {
            id: "US-TEST-001".to_string(),
            title: "Test User Story".to_string(),
            description: "As a user, I want to test this functionality".to_string(),
            persona: "Test User".to_string(),
        }
    }

    #[sqlx::test]
    async fn test_create_user_story(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);
        let request = create_test_request();

        let result = service.create(request.clone()).await;
        assert!(result.is_ok());

        let user_story = result.unwrap();
        assert_eq!(user_story.id, request.id);
        assert_eq!(user_story.title, request.title);
        assert_eq!(user_story.description, request.description);
        assert_eq!(user_story.persona, request.persona);
    }

    #[sqlx::test]
    async fn test_create_duplicate_user_story(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);
        let request = create_test_request();

        // Create first user story
        service.create(request.clone()).await.unwrap();

        // Try to create duplicate
        let result = service.create(request).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UserStoryServiceError::AlreadyExists { .. }
        ));
    }

    #[sqlx::test]
    async fn test_validation_empty_id(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);
        let mut request = create_test_request();
        request.id = "".to_string();

        let result = service.create(request).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UserStoryServiceError::Validation { .. }
        ));
    }

    #[sqlx::test]
    async fn test_validation_invalid_id_format(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);
        let mut request = create_test_request();
        request.id = "INVALID-001".to_string();

        let result = service.create(request).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UserStoryServiceError::Validation { .. }
        ));
    }

    #[sqlx::test]
    async fn test_create_with_criteria(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);
        let user_story_request = create_test_request();

        let criteria_requests = vec![
            CreateAcceptanceCriteriaRequest {
                id: "AC-TEST-001".to_string(),
                user_story_id: user_story_request.id.clone(),
                description: "First criteria".to_string(),
            },
            CreateAcceptanceCriteriaRequest {
                id: "AC-TEST-002".to_string(),
                user_story_id: user_story_request.id.clone(),
                description: "Second criteria".to_string(),
            },
        ];

        let result = service
            .create_with_criteria(user_story_request.clone(), criteria_requests)
            .await;

        assert!(result.is_ok());
        let story_with_criteria = result.unwrap();
        assert_eq!(story_with_criteria.user_story.id, user_story_request.id);
        assert_eq!(story_with_criteria.acceptance_criteria.len(), 2);
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_by_id(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        // Get a user story from fixtures
        let result = service.get_by_id("US-001").await;
        assert!(result.is_ok());

        let user_story = result.unwrap();
        assert_eq!(user_story.id, "US-001");
        assert_eq!(user_story.title, "User Login Feature");
        assert_eq!(user_story.persona, "Registered User");
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_get_with_criteria(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        // Get a user story with criteria from fixtures
        let result = service.get_with_criteria("US-001").await;
        assert!(result.is_ok());

        let story_with_criteria = result.unwrap();
        assert_eq!(story_with_criteria.user_story.id, "US-001");
        assert_eq!(story_with_criteria.acceptance_criteria.len(), 3);
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_all(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        let result = service.get_all().await;
        assert!(result.is_ok());

        let user_stories = result.unwrap();
        assert_eq!(user_stories.len(), 5); // Should match fixture count
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_update_user_story(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        // Update an existing user story from fixtures
        let update_request = UpdateUserStoryRequest {
            title: Some("Updated User Login Feature".to_string()),
            description: None,
            persona: Some("Updated Persona".to_string()),
        };

        let result = service.update("US-001", update_request).await;
        assert!(result.is_ok());

        let user_story = result.unwrap();
        assert_eq!(user_story.title, "Updated User Login Feature");
        assert_eq!(user_story.persona, "Updated Persona");
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_delete_user_story(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        // Delete an existing user story from fixtures
        let result = service.delete("US-001").await;
        assert!(result.is_ok());

        // Verify it's deleted
        let get_result = service.get_by_id("US-001").await;
        assert!(get_result.is_err());
        assert!(matches!(
            get_result.unwrap_err(),
            UserStoryServiceError::NotFound { .. }
        ));
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_search_user_stories(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        // Search for "login" - should match the fixture user story
        let result = service.search("login").await;
        assert!(result.is_ok());

        let stories = result.unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].id, "US-001");
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_by_persona(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        // Search for stories by persona
        let result = service.get_by_persona("Registered User").await;
        assert!(result.is_ok());

        let stories = result.unwrap();
        assert_eq!(stories.len(), 3); // US-001, US-003, US-004 from fixtures
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_paginated(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        // Get first 2 user stories
        let result = service.get_paginated(2, 0).await;
        assert!(result.is_ok());

        let stories = result.unwrap();
        assert_eq!(stories.len(), 2);

        // Get next 2 user stories
        let result = service.get_paginated(2, 2).await;
        assert!(result.is_ok());

        let stories = result.unwrap();
        assert_eq!(stories.len(), 2);
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_grouped_by_persona(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        let result = service.get_grouped_by_persona().await;
        assert!(result.is_ok());

        let grouped = result.unwrap();
        assert_eq!(grouped.len(), 3); // Should have 3 unique personas from fixtures

        // Check that "Registered User" has 3 stories
        let registered_user_stories = grouped.get("Registered User").unwrap();
        assert_eq!(registered_user_stories.len(), 3);
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_get_statistics(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = UserStoryService::new(repositories);

        let stats = service.get_statistics().await.unwrap();
        assert_eq!(stats.total_stories, 5); // Should match fixture count
        assert_eq!(stats.total_criteria, 10); // Should match fixture count
        assert_eq!(stats.personas_count, 3); // Should have 3 unique personas
        assert_eq!(stats.avg_criteria_per_story, 2.0);
    }
}
