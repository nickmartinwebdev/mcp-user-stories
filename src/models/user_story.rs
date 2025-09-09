use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserStory {
    pub id: String,
    pub title: String,
    pub description: String,
    pub persona: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserStoryRequest {
    pub id: String,
    pub title: String,
    pub description: String,
    pub persona: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateUserStoryRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub persona: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserStoryWithCriteria {
    #[serde(flatten)]
    pub user_story: UserStory,
    pub acceptance_criteria: Vec<crate::models::AcceptanceCriteria>,
}
