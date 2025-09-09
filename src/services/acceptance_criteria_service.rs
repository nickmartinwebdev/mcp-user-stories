use crate::models::{
    AcceptanceCriteria, CreateAcceptanceCriteriaRequest, UpdateAcceptanceCriteriaRequest,
};
use crate::repositories::Repositories;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AcceptanceCriteriaServiceError {
    #[error("Acceptance criteria not found: {id}")]
    NotFound { id: String },
    #[error("Acceptance criteria already exists: {id}")]
    AlreadyExists { id: String },
    #[error("User story not found: {user_story_id}")]
    UserStoryNotFound { user_story_id: String },
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {message}")]
    Validation { message: String },
    #[error("Business rule violation: {message}")]
    BusinessRule { message: String },
}

pub type Result<T> = std::result::Result<T, AcceptanceCriteriaServiceError>;

#[derive(Clone)]
pub struct AcceptanceCriteriaService {
    repositories: Repositories,
}

impl AcceptanceCriteriaService {
    pub fn new(repositories: Repositories) -> Self {
        Self { repositories }
    }

    /// Create a new acceptance criteria with validation
    pub async fn create(
        &self,
        request: CreateAcceptanceCriteriaRequest,
    ) -> Result<AcceptanceCriteria> {
        // Validate the request
        self.validate_create_request(&request).await?;

        // Check if acceptance criteria already exists
        if self
            .repositories
            .acceptance_criteria
            .get_by_id(&request.id)
            .await?
            .is_some()
        {
            return Err(AcceptanceCriteriaServiceError::AlreadyExists {
                id: request.id.clone(),
            });
        }

        // Verify that the user story exists
        if self
            .repositories
            .user_stories
            .get_by_id(&request.user_story_id)
            .await?
            .is_none()
        {
            return Err(AcceptanceCriteriaServiceError::UserStoryNotFound {
                user_story_id: request.user_story_id.clone(),
            });
        }

        // Create the acceptance criteria
        let criteria = self
            .repositories
            .acceptance_criteria
            .create(request)
            .await?;

        Ok(criteria)
    }

    /// Create multiple acceptance criteria for a user story
    pub async fn create_batch(
        &self,
        requests: Vec<CreateAcceptanceCriteriaRequest>,
    ) -> Result<Vec<AcceptanceCriteria>> {
        if requests.is_empty() {
            return Err(AcceptanceCriteriaServiceError::Validation {
                message: "Cannot create empty batch of acceptance criteria".to_string(),
            });
        }

        // Validate all requests
        for request in &requests {
            self.validate_create_request(request).await?;
        }

        // Check if any criteria already exist
        for request in &requests {
            if self
                .repositories
                .acceptance_criteria
                .get_by_id(&request.id)
                .await?
                .is_some()
            {
                return Err(AcceptanceCriteriaServiceError::AlreadyExists {
                    id: request.id.clone(),
                });
            }
        }

        // Verify that all user stories exist
        let mut user_story_ids: std::collections::HashSet<String> =
            std::collections::HashSet::new();
        for request in &requests {
            user_story_ids.insert(request.user_story_id.clone());
        }

        for user_story_id in user_story_ids {
            if self
                .repositories
                .user_stories
                .get_by_id(&user_story_id)
                .await?
                .is_none()
            {
                return Err(AcceptanceCriteriaServiceError::UserStoryNotFound { user_story_id });
            }
        }

        // Create all criteria in batch
        let criteria = self
            .repositories
            .acceptance_criteria
            .create_batch(requests)
            .await?;

        Ok(criteria)
    }

    /// Get acceptance criteria by ID
    pub async fn get_by_id(&self, id: &str) -> Result<AcceptanceCriteria> {
        self.repositories
            .acceptance_criteria
            .get_by_id(id)
            .await?
            .ok_or_else(|| AcceptanceCriteriaServiceError::NotFound { id: id.to_string() })
    }

    /// Get all acceptance criteria for a user story
    pub async fn get_by_user_story_id(
        &self,
        user_story_id: &str,
    ) -> Result<Vec<AcceptanceCriteria>> {
        // Verify that the user story exists
        if self
            .repositories
            .user_stories
            .get_by_id(user_story_id)
            .await?
            .is_none()
        {
            return Err(AcceptanceCriteriaServiceError::UserStoryNotFound {
                user_story_id: user_story_id.to_string(),
            });
        }

        Ok(self
            .repositories
            .acceptance_criteria
            .get_by_user_story_id(user_story_id)
            .await?)
    }

    /// Get all acceptance criteria
    pub async fn get_all(&self) -> Result<Vec<AcceptanceCriteria>> {
        Ok(self.repositories.acceptance_criteria.get_all().await?)
    }

    /// Update acceptance criteria
    pub async fn update(
        &self,
        id: &str,
        request: UpdateAcceptanceCriteriaRequest,
    ) -> Result<AcceptanceCriteria> {
        // Validate the update request
        self.validate_update_request(&request)?;

        self.repositories
            .acceptance_criteria
            .update(id, request)
            .await?
            .ok_or_else(|| AcceptanceCriteriaServiceError::NotFound { id: id.to_string() })
    }

    /// Delete acceptance criteria
    pub async fn delete(&self, id: &str) -> Result<()> {
        let deleted = self.repositories.acceptance_criteria.delete(id).await?;

        if !deleted {
            return Err(AcceptanceCriteriaServiceError::NotFound { id: id.to_string() });
        }

        Ok(())
    }

    /// Delete all acceptance criteria for a user story
    pub async fn delete_by_user_story_id(&self, user_story_id: &str) -> Result<u64> {
        // Verify that the user story exists
        if self
            .repositories
            .user_stories
            .get_by_id(user_story_id)
            .await?
            .is_none()
        {
            return Err(AcceptanceCriteriaServiceError::UserStoryNotFound {
                user_story_id: user_story_id.to_string(),
            });
        }

        let deleted_count = self
            .repositories
            .acceptance_criteria
            .delete_by_user_story_id(user_story_id)
            .await?;

        Ok(deleted_count)
    }

    /// Search acceptance criteria by description
    pub async fn search(&self, query: &str) -> Result<Vec<AcceptanceCriteria>> {
        if query.trim().is_empty() {
            return Err(AcceptanceCriteriaServiceError::Validation {
                message: "Search query cannot be empty".to_string(),
            });
        }

        Ok(self.repositories.acceptance_criteria.search(query).await?)
    }

    /// Get count of acceptance criteria for a user story
    pub async fn count_by_user_story_id(&self, user_story_id: &str) -> Result<i64> {
        // Verify that the user story exists
        if self
            .repositories
            .user_stories
            .get_by_id(user_story_id)
            .await?
            .is_none()
        {
            return Err(AcceptanceCriteriaServiceError::UserStoryNotFound {
                user_story_id: user_story_id.to_string(),
            });
        }

        Ok(self
            .repositories
            .acceptance_criteria
            .count_by_user_story_id(user_story_id)
            .await?)
    }

    /// Get statistics about acceptance criteria
    pub async fn get_statistics(&self) -> Result<AcceptanceCriteriaStatistics> {
        let total_criteria = self.repositories.acceptance_criteria.count().await?;
        let total_stories = self.repositories.user_stories.count().await?;

        let avg_criteria_per_story = if total_stories > 0 {
            total_criteria as f64 / total_stories as f64
        } else {
            0.0
        };

        // Get criteria distribution by user story
        let all_stories = self.repositories.user_stories.get_all().await?;
        let mut criteria_distribution = std::collections::HashMap::new();

        for story in all_stories {
            let count = self
                .repositories
                .acceptance_criteria
                .count_by_user_story_id(&story.id)
                .await?;
            criteria_distribution.insert(story.id, count);
        }

        Ok(AcceptanceCriteriaStatistics {
            total_criteria,
            total_stories,
            avg_criteria_per_story,
            criteria_distribution,
        })
    }

    /// Validate create request
    async fn validate_create_request(
        &self,
        request: &CreateAcceptanceCriteriaRequest,
    ) -> Result<()> {
        if request.id.trim().is_empty() {
            return Err(AcceptanceCriteriaServiceError::Validation {
                message: "Acceptance criteria ID cannot be empty".to_string(),
            });
        }

        if request.user_story_id.trim().is_empty() {
            return Err(AcceptanceCriteriaServiceError::Validation {
                message: "User story ID cannot be empty".to_string(),
            });
        }

        if request.description.trim().is_empty() {
            return Err(AcceptanceCriteriaServiceError::Validation {
                message: "Acceptance criteria description cannot be empty".to_string(),
            });
        }

        // Validate ID format (should follow AC-XXX pattern)
        if !request.id.starts_with("AC-") {
            return Err(AcceptanceCriteriaServiceError::Validation {
                message: "Acceptance criteria ID should start with 'AC-'".to_string(),
            });
        }

        // Validate user story ID format
        if !request.user_story_id.starts_with("US-") {
            return Err(AcceptanceCriteriaServiceError::Validation {
                message: "User story ID should start with 'US-'".to_string(),
            });
        }

        // Validate description length
        if request.description.len() > 1000 {
            return Err(AcceptanceCriteriaServiceError::Validation {
                message: "Acceptance criteria description cannot exceed 1000 characters"
                    .to_string(),
            });
        }

        // Business rule: Check if the user story already has too many acceptance criteria
        let existing_count = self
            .repositories
            .acceptance_criteria
            .count_by_user_story_id(&request.user_story_id)
            .await?;

        const MAX_CRITERIA_PER_STORY: i64 = 20;
        if existing_count >= MAX_CRITERIA_PER_STORY {
            return Err(AcceptanceCriteriaServiceError::BusinessRule {
                message: format!(
                    "User story {} already has {} acceptance criteria. Maximum allowed is {}.",
                    request.user_story_id, existing_count, MAX_CRITERIA_PER_STORY
                ),
            });
        }

        Ok(())
    }

    /// Validate update request
    fn validate_update_request(&self, request: &UpdateAcceptanceCriteriaRequest) -> Result<()> {
        if let Some(ref description) = request.description {
            if description.trim().is_empty() {
                return Err(AcceptanceCriteriaServiceError::Validation {
                    message: "Acceptance criteria description cannot be empty".to_string(),
                });
            }
            if description.len() > 1000 {
                return Err(AcceptanceCriteriaServiceError::Validation {
                    message: "Acceptance criteria description cannot exceed 1000 characters"
                        .to_string(),
                });
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct AcceptanceCriteriaStatistics {
    pub total_criteria: i64,
    pub total_stories: i64,
    pub avg_criteria_per_story: f64,
    pub criteria_distribution: std::collections::HashMap<String, i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateUserStoryRequest;
    use crate::repositories::Repositories;

    async fn create_test_user_story(service: &AcceptanceCriteriaService) -> String {
        let user_story_request = CreateUserStoryRequest {
            id: "US-TEST-001".to_string(),
            title: "Test User Story".to_string(),
            description: "As a user, I want to test this functionality".to_string(),
            persona: "Test User".to_string(),
        };

        service
            .repositories
            .user_stories
            .create(user_story_request.clone())
            .await
            .unwrap();

        user_story_request.id
    }

    fn create_test_criteria_request(user_story_id: String) -> CreateAcceptanceCriteriaRequest {
        CreateAcceptanceCriteriaRequest {
            id: "AC-TEST-001".to_string(),
            user_story_id,
            description: "Given I am on the page, When I click the button, Then something happens"
                .to_string(),
        }
    }

    #[sqlx::test]
    async fn test_create_acceptance_criteria(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);
        let user_story_id = create_test_user_story(&service).await;
        let request = create_test_criteria_request(user_story_id);

        let result = service.create(request.clone()).await;
        assert!(result.is_ok());

        let criteria = result.unwrap();
        assert_eq!(criteria.id, request.id);
        assert_eq!(criteria.description, request.description);
    }

    #[sqlx::test]
    async fn test_create_criteria_for_nonexistent_user_story(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);
        let request = create_test_criteria_request("US-999".to_string());

        let result = service.create(request).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AcceptanceCriteriaServiceError::UserStoryNotFound { .. }
        ));
    }

    #[sqlx::test]
    async fn test_create_duplicate_criteria(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);
        let user_story_id = create_test_user_story(&service).await;
        let request = create_test_criteria_request(user_story_id);

        // Create first criteria
        service.create(request.clone()).await.unwrap();

        // Try to create duplicate
        let result = service.create(request).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AcceptanceCriteriaServiceError::AlreadyExists { .. }
        ));
    }

    #[sqlx::test]
    async fn test_validation_empty_id(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);
        let user_story_id = create_test_user_story(&service).await;
        let mut request = create_test_criteria_request(user_story_id);
        request.id = "".to_string();

        let result = service.create(request).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AcceptanceCriteriaServiceError::Validation { .. }
        ));
    }

    #[sqlx::test]
    async fn test_validation_invalid_id_format(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);
        let user_story_id = create_test_user_story(&service).await;
        let mut request = create_test_criteria_request(user_story_id);
        request.id = "INVALID-001".to_string();

        let result = service.create(request).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            AcceptanceCriteriaServiceError::Validation { .. }
        ));
    }

    #[sqlx::test]
    async fn test_create_batch(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);
        let user_story_id = create_test_user_story(&service).await;

        let requests = vec![
            CreateAcceptanceCriteriaRequest {
                id: "AC-BATCH-001".to_string(),
                user_story_id: user_story_id.clone(),
                description: "First criteria".to_string(),
            },
            CreateAcceptanceCriteriaRequest {
                id: "AC-BATCH-002".to_string(),
                user_story_id: user_story_id.clone(),
                description: "Second criteria".to_string(),
            },
        ];

        let result = service.create_batch(requests).await;
        assert!(result.is_ok());

        let criteria_list = result.unwrap();
        assert_eq!(criteria_list.len(), 2);
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_get_by_id(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);

        // Get an acceptance criteria from fixtures
        let result = service.get_by_id("AC-001").await;
        assert!(result.is_ok());

        let criteria = result.unwrap();
        assert_eq!(criteria.id, "AC-001");
        assert_eq!(criteria.user_story_id, "US-001");
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_get_by_user_story_id(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);

        // Get all acceptance criteria for US-001 from fixtures
        let result = service.get_by_user_story_id("US-001").await;
        assert!(result.is_ok());

        let criteria_list = result.unwrap();
        assert_eq!(criteria_list.len(), 3); // AC-001, AC-002, AC-003
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_update_acceptance_criteria(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);

        // Update an existing acceptance criteria from fixtures
        let update_request = UpdateAcceptanceCriteriaRequest {
            description: Some("Updated description".to_string()),
        };

        let result = service.update("AC-001", update_request).await;
        assert!(result.is_ok());

        let criteria = result.unwrap();
        assert_eq!(criteria.description, "Updated description");
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_delete_acceptance_criteria(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);

        // Delete an existing acceptance criteria from fixtures
        let result = service.delete("AC-001").await;
        assert!(result.is_ok());

        // Verify it's deleted
        let get_result = service.get_by_id("AC-001").await;
        assert!(get_result.is_err());
        assert!(matches!(
            get_result.unwrap_err(),
            AcceptanceCriteriaServiceError::NotFound { .. }
        ));
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_search_criteria(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);

        // Search for criteria containing "login"
        let result = service.search("login").await;
        assert!(result.is_ok());

        let criteria_list = result.unwrap();
        assert!(!criteria_list.is_empty());

        // Verify all results contain "login" in description
        for criteria in criteria_list {
            assert!(criteria.description.to_lowercase().contains("login"));
        }
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_count_by_user_story_id(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);

        // Count criteria for US-001
        let result = service.count_by_user_story_id("US-001").await;
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 3); // Should match fixture count for US-001
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_get_statistics(pool: sqlx::SqlitePool) {
        let repositories = Repositories::new(pool);
        let service = AcceptanceCriteriaService::new(repositories);

        let stats = service.get_statistics().await.unwrap();
        assert_eq!(stats.total_criteria, 10); // Should match fixture count
        assert_eq!(stats.total_stories, 5); // Should match fixture count
        assert_eq!(stats.avg_criteria_per_story, 2.0);
        assert_eq!(stats.criteria_distribution.get("US-001"), Some(&3));
    }
}
