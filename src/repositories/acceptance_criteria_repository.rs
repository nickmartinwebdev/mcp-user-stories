use crate::database::DbPool;
use crate::models::{
    AcceptanceCriteria, CreateAcceptanceCriteriaRequest, UpdateAcceptanceCriteriaRequest,
};
use chrono::Utc;

#[derive(Clone)]
pub struct AcceptanceCriteriaRepository {
    pool: DbPool,
}

impl AcceptanceCriteriaRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new acceptance criteria
    pub async fn create(
        &self,
        request: CreateAcceptanceCriteriaRequest,
    ) -> Result<AcceptanceCriteria, sqlx::Error> {
        let now = Utc::now().naive_utc();

        let criteria = sqlx::query_as!(
            AcceptanceCriteria,
            r#"
            INSERT INTO acceptance_criteria (id, user_story_id, description, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, user_story_id, description, created_at, updated_at
            "#,
            request.id,
            request.user_story_id,
            request.description,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(criteria)
    }

    /// Get acceptance criteria by ID
    pub async fn get_by_id(&self, id: &str) -> Result<Option<AcceptanceCriteria>, sqlx::Error> {
        let criteria = sqlx::query_as!(
            AcceptanceCriteria,
            r#"
            SELECT id, user_story_id, description, created_at, updated_at
            FROM acceptance_criteria
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(criteria)
    }

    /// Get all acceptance criteria for a user story
    pub async fn get_by_user_story_id(
        &self,
        user_story_id: &str,
    ) -> Result<Vec<AcceptanceCriteria>, sqlx::Error> {
        let criteria = sqlx::query_as!(
            AcceptanceCriteria,
            r#"
            SELECT id, user_story_id, description, created_at, updated_at
            FROM acceptance_criteria
            WHERE user_story_id = $1
            ORDER BY created_at ASC
            "#,
            user_story_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(criteria)
    }

    /// Get all acceptance criteria
    pub async fn get_all(&self) -> Result<Vec<AcceptanceCriteria>, sqlx::Error> {
        let criteria = sqlx::query_as!(
            AcceptanceCriteria,
            r#"
            SELECT id, user_story_id, description, created_at, updated_at
            FROM acceptance_criteria
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(criteria)
    }

    /// Update acceptance criteria
    pub async fn update(
        &self,
        id: &str,
        request: UpdateAcceptanceCriteriaRequest,
    ) -> Result<Option<AcceptanceCriteria>, sqlx::Error> {
        // First, check if the acceptance criteria exists
        let existing = self.get_by_id(id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let now = Utc::now().naive_utc();

        let criteria = sqlx::query_as!(
            AcceptanceCriteria,
            r#"
            UPDATE acceptance_criteria
            SET
                description = COALESCE($2, description),
                updated_at = $3
            WHERE id = $1
            RETURNING id, user_story_id, description, created_at, updated_at
            "#,
            id,
            request.description,
            now
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(criteria)
    }

    /// Delete acceptance criteria
    pub async fn delete(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM acceptance_criteria
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Delete all acceptance criteria for a user story
    pub async fn delete_by_user_story_id(&self, user_story_id: &str) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM acceptance_criteria
            WHERE user_story_id = $1
            "#,
            user_story_id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected())
    }

    /// Search acceptance criteria by description
    pub async fn search(&self, query: &str) -> Result<Vec<AcceptanceCriteria>, sqlx::Error> {
        let search_pattern = format!("%{}%", query);

        let criteria = sqlx::query_as!(
            AcceptanceCriteria,
            r#"
            SELECT id, user_story_id, description, created_at, updated_at
            FROM acceptance_criteria
            WHERE description LIKE $1
            ORDER BY created_at DESC
            "#,
            search_pattern
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(criteria)
    }

    /// Get count of acceptance criteria for a user story
    pub async fn count_by_user_story_id(&self, user_story_id: &str) -> Result<i64, sqlx::Error> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM acceptance_criteria
            WHERE user_story_id = $1
            "#,
            user_story_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count)
    }

    /// Get total count of all acceptance criteria
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM acceptance_criteria
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count)
    }

    /// Create multiple acceptance criteria in a transaction
    pub async fn create_batch(
        &self,
        requests: Vec<CreateAcceptanceCriteriaRequest>,
    ) -> Result<Vec<AcceptanceCriteria>, sqlx::Error> {
        let mut tx = self.pool.begin().await?;
        let mut created_criteria = Vec::new();

        for request in requests {
            let now = Utc::now().naive_utc();

            let criteria = sqlx::query_as!(
                AcceptanceCriteria,
                r#"
                INSERT INTO acceptance_criteria (id, user_story_id, description, created_at, updated_at)
                VALUES ($1, $2, $3, $4, $5)
                RETURNING id, user_story_id, description, created_at, updated_at
                "#,
                request.id,
                request.user_story_id,
                request.description,
                now,
                now
            )
            .fetch_one(&mut *tx)
            .await?;

            created_criteria.push(criteria);
        }

        tx.commit().await?;
        Ok(created_criteria)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::CreateUserStoryRequest;
    use crate::repositories::UserStoryRepository;

    async fn create_test_user_story(user_story_repo: &UserStoryRepository) -> String {
        let request = CreateUserStoryRequest {
            id: "US-TEST-001".to_string(),
            title: "Test User Story".to_string(),
            description: "As a user, I want to test this functionality".to_string(),
            persona: "Test User".to_string(),
        };

        user_story_repo.create(request.clone()).await.unwrap();
        request.id
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
        let criteria_repo = AcceptanceCriteriaRepository::new(pool.clone());
        let user_story_repo = UserStoryRepository::new(pool);

        let user_story_id = create_test_user_story(&user_story_repo).await;
        let request = create_test_criteria_request(user_story_id);

        let result = criteria_repo.create(request.clone()).await;
        assert!(result.is_ok());

        let criteria = result.unwrap();
        assert_eq!(criteria.id, request.id);
        assert_eq!(criteria.user_story_id, request.user_story_id);
        assert_eq!(criteria.description, request.description);
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_get_criteria_by_id(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        // Get an acceptance criteria from fixtures
        let result = criteria_repo.get_by_id("AC-001").await;
        assert!(result.is_ok());

        let criteria = result.unwrap();
        assert!(criteria.is_some());

        let criteria = criteria.unwrap();
        assert_eq!(criteria.id, "AC-001");
        assert_eq!(criteria.user_story_id, "US-001");
        assert!(criteria.description.contains("valid credentials"));
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_get_criteria_by_user_story_id(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        // Get all acceptance criteria for US-001 from fixtures
        let result = criteria_repo.get_by_user_story_id("US-001").await;
        assert!(result.is_ok());

        let criteria_list = result.unwrap();
        assert_eq!(criteria_list.len(), 3); // AC-001, AC-002, AC-003
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_get_all_criteria(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        let result = criteria_repo.get_all().await;
        assert!(result.is_ok());

        let criteria_list = result.unwrap();
        assert_eq!(criteria_list.len(), 10); // Should match fixture count
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_update_acceptance_criteria(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        // Update an existing acceptance criteria from fixtures
        let update_request = UpdateAcceptanceCriteriaRequest {
            description: Some("Given I am on the updated login page, When I enter valid credentials, Then I should be logged in successfully".to_string()),
        };

        let result = criteria_repo.update("AC-001", update_request).await;
        assert!(result.is_ok());

        let criteria = result.unwrap();
        assert!(criteria.is_some());

        let criteria = criteria.unwrap();
        assert!(criteria.description.contains("updated login page"));
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_delete_acceptance_criteria(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        // Delete an existing acceptance criteria from fixtures
        let result = criteria_repo.delete("AC-001").await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Verify it's deleted
        let get_result = criteria_repo.get_by_id("AC-001").await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_none());
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_delete_by_user_story_id(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        // Delete all acceptance criteria for US-001
        let result = criteria_repo.delete_by_user_story_id("US-001").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3); // Should delete 3 criteria

        // Verify they're deleted
        let get_result = criteria_repo.get_by_user_story_id("US-001").await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_empty());
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_search_criteria(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        // Search for criteria containing "login"
        let result = criteria_repo.search("login").await;
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
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        // Count criteria for US-001
        let result = criteria_repo.count_by_user_story_id("US-001").await;
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 3); // Should match fixture count for US-001
    }

    #[sqlx::test(fixtures(
        "../../fixtures/user_stories.sql",
        "../../fixtures/acceptance_criteria.sql"
    ))]
    async fn test_count_all_criteria(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool);

        let result = criteria_repo.count().await;
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 10); // Should match fixture count
    }

    #[sqlx::test]
    async fn test_create_batch(pool: sqlx::SqlitePool) {
        let criteria_repo = AcceptanceCriteriaRepository::new(pool.clone());
        let user_story_repo = UserStoryRepository::new(pool);

        let user_story_id = create_test_user_story(&user_story_repo).await;

        let requests = vec![
            CreateAcceptanceCriteriaRequest {
                id: "AC-BATCH-001".to_string(),
                user_story_id: user_story_id.clone(),
                description: "First batch criteria".to_string(),
            },
            CreateAcceptanceCriteriaRequest {
                id: "AC-BATCH-002".to_string(),
                user_story_id: user_story_id.clone(),
                description: "Second batch criteria".to_string(),
            },
        ];

        let result = criteria_repo.create_batch(requests).await;
        assert!(result.is_ok());

        let created_criteria = result.unwrap();
        assert_eq!(created_criteria.len(), 2);

        // Verify they were actually created
        let all_criteria = criteria_repo
            .get_by_user_story_id(&user_story_id)
            .await
            .unwrap();
        assert_eq!(all_criteria.len(), 2);
    }
}
