use crate::database::DbPool;
use crate::models::{CreateUserStoryRequest, UpdateUserStoryRequest, UserStory};
use chrono::Utc;
use std::collections::HashMap;

#[derive(Clone)]
pub struct UserStoryRepository {
    pool: DbPool,
}

impl UserStoryRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }

    /// Create a new user story
    pub async fn create(&self, request: CreateUserStoryRequest) -> Result<UserStory, sqlx::Error> {
        let now = Utc::now().naive_utc();

        let user_story = sqlx::query_as!(
            UserStory,
            r#"
            INSERT INTO user_stories (id, title, description, persona, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, title, description, persona, created_at, updated_at
            "#,
            request.id,
            request.title,
            request.description,
            request.persona,
            now,
            now
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(user_story)
    }

    /// Get a user story by ID
    pub async fn get_by_id(&self, id: &str) -> Result<Option<UserStory>, sqlx::Error> {
        let user_story = sqlx::query_as!(
            UserStory,
            r#"
            SELECT id, title, description, persona, created_at, updated_at
            FROM user_stories
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user_story)
    }

    /// Get all user stories
    pub async fn get_all(&self) -> Result<Vec<UserStory>, sqlx::Error> {
        let user_stories = sqlx::query_as!(
            UserStory,
            r#"
            SELECT id, title, description, persona, created_at, updated_at
            FROM user_stories
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user_stories)
    }

    /// Get user stories with pagination
    pub async fn get_paginated(
        &self,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserStory>, sqlx::Error> {
        let user_stories = sqlx::query_as!(
            UserStory,
            r#"
            SELECT id, title, description, persona, created_at, updated_at
            FROM user_stories
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user_stories)
    }

    /// Update a user story
    pub async fn update(
        &self,
        id: &str,
        request: UpdateUserStoryRequest,
    ) -> Result<Option<UserStory>, sqlx::Error> {
        // First, check if the user story exists
        let existing = self.get_by_id(id).await?;
        if existing.is_none() {
            return Ok(None);
        }

        let now = Utc::now().naive_utc();

        let user_story = sqlx::query_as!(
            UserStory,
            r#"
            UPDATE user_stories
            SET
                title = COALESCE($2, title),
                description = COALESCE($3, description),
                persona = COALESCE($4, persona),
                updated_at = $5
            WHERE id = $1
            RETURNING id, title, description, persona, created_at, updated_at
            "#,
            id,
            request.title,
            request.description,
            request.persona,
            now
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(user_story)
    }

    /// Delete a user story
    pub async fn delete(&self, id: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            r#"
            DELETE FROM user_stories
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    /// Search user stories by title or description
    pub async fn search(&self, query: &str) -> Result<Vec<UserStory>, sqlx::Error> {
        let search_pattern = format!("%{}%", query);

        let user_stories = sqlx::query_as!(
            UserStory,
            r#"
            SELECT id, title, description, persona, created_at, updated_at
            FROM user_stories
            WHERE title LIKE $1 OR description LIKE $1 OR persona LIKE $1
            ORDER BY created_at DESC
            "#,
            search_pattern
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user_stories)
    }

    /// Get user stories by persona
    pub async fn get_by_persona(&self, persona: &str) -> Result<Vec<UserStory>, sqlx::Error> {
        let user_stories = sqlx::query_as!(
            UserStory,
            r#"
            SELECT id, title, description, persona, created_at, updated_at
            FROM user_stories
            WHERE persona = $1
            ORDER BY created_at DESC
            "#,
            persona
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(user_stories)
    }

    /// Get count of all user stories
    pub async fn count(&self) -> Result<i64, sqlx::Error> {
        let count = sqlx::query!(
            r#"
            SELECT COUNT(*) as count
            FROM user_stories
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.count)
    }

    /// Get user stories grouped by persona
    pub async fn get_grouped_by_persona(
        &self,
    ) -> Result<HashMap<String, Vec<UserStory>>, sqlx::Error> {
        let user_stories = self.get_all().await?;
        let mut grouped = HashMap::new();

        for story in user_stories {
            grouped
                .entry(story.persona.clone())
                .or_insert_with(Vec::new)
                .push(story);
        }

        Ok(grouped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let repo = UserStoryRepository::new(pool);
        let request = create_test_request();

        let result = repo.create(request.clone()).await;
        assert!(result.is_ok());

        let user_story = result.unwrap();
        assert_eq!(user_story.id, request.id);
        assert_eq!(user_story.title, request.title);
        assert_eq!(user_story.description, request.description);
        assert_eq!(user_story.persona, request.persona);
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_user_story_by_id(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        // Get a user story from fixtures
        let result = repo.get_by_id("US-001").await;
        assert!(result.is_ok());

        let user_story = result.unwrap();
        assert!(user_story.is_some());

        let user_story = user_story.unwrap();
        assert_eq!(user_story.id, "US-001");
        assert_eq!(user_story.title, "User Login Feature");
        assert_eq!(user_story.persona, "Registered User");
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_all_user_stories(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        let result = repo.get_all().await;
        assert!(result.is_ok());

        let user_stories = result.unwrap();
        assert_eq!(user_stories.len(), 5); // Should match fixture count
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_update_user_story(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        // Update an existing user story from fixtures
        let update_request = UpdateUserStoryRequest {
            title: Some("Updated User Login Feature".to_string()),
            description: None,
            persona: Some("Updated Persona".to_string()),
        };

        let result = repo.update("US-001", update_request).await;
        assert!(result.is_ok());

        let user_story = result.unwrap();
        assert!(user_story.is_some());

        let user_story = user_story.unwrap();
        assert_eq!(user_story.title, "Updated User Login Feature");
        assert_eq!(user_story.persona, "Updated Persona");
        // Description should remain unchanged from fixture
        assert!(user_story.description.contains("registered user"));
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_delete_user_story(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        // Delete an existing user story from fixtures
        let result = repo.delete("US-001").await;
        assert!(result.is_ok());
        assert!(result.unwrap());

        // Verify it's deleted
        let get_result = repo.get_by_id("US-001").await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_none());
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_search_user_stories(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        // Search for "login" - should match the fixture user story
        let result = repo.search("login").await;
        assert!(result.is_ok());

        let stories = result.unwrap();
        assert_eq!(stories.len(), 1);
        assert_eq!(stories[0].id, "US-001");
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_by_persona(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        // Search for stories by persona
        let result = repo.get_by_persona("Registered User").await;
        assert!(result.is_ok());

        let stories = result.unwrap();
        assert_eq!(stories.len(), 3); // US-001, US-003, US-004 from fixtures
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_count_user_stories(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        let result = repo.count().await;
        assert!(result.is_ok());

        let count = result.unwrap();
        assert_eq!(count, 5); // Should match fixture count
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_paginated(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        // Get first 2 user stories
        let result = repo.get_paginated(2, 0).await;
        assert!(result.is_ok());

        let stories = result.unwrap();
        assert_eq!(stories.len(), 2);

        // Get next 2 user stories
        let result = repo.get_paginated(2, 2).await;
        assert!(result.is_ok());

        let stories = result.unwrap();
        assert_eq!(stories.len(), 2);
    }

    #[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
    async fn test_get_grouped_by_persona(pool: sqlx::SqlitePool) {
        let repo = UserStoryRepository::new(pool);

        let result = repo.get_grouped_by_persona().await;
        assert!(result.is_ok());

        let grouped = result.unwrap();
        assert_eq!(grouped.len(), 3); // Should have 3 unique personas from fixtures

        // Check that "Registered User" has 3 stories
        let registered_user_stories = grouped.get("Registered User").unwrap();
        assert_eq!(registered_user_stories.len(), 3);
    }
}
