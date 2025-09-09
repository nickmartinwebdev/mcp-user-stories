pub mod acceptance_criteria_repository;
pub mod user_story_repository;

pub use acceptance_criteria_repository::AcceptanceCriteriaRepository;
pub use user_story_repository::UserStoryRepository;

use crate::database::DbPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct Repositories {
    pub user_stories: Arc<UserStoryRepository>,
    pub acceptance_criteria: Arc<AcceptanceCriteriaRepository>,
}

impl Repositories {
    pub fn new(pool: DbPool) -> Self {
        Self {
            user_stories: Arc::new(UserStoryRepository::new(pool.clone())),
            acceptance_criteria: Arc::new(AcceptanceCriteriaRepository::new(pool)),
        }
    }
}
