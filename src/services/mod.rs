pub mod acceptance_criteria_service;
pub mod user_story_service;

pub use acceptance_criteria_service::AcceptanceCriteriaService;
pub use user_story_service::UserStoryService;

use crate::repositories::Repositories;
use std::sync::Arc;

#[derive(Clone)]
pub struct Services {
    pub user_stories: Arc<UserStoryService>,
    #[allow(dead_code)]
    pub acceptance_criteria: Arc<AcceptanceCriteriaService>,
}

impl Services {
    pub fn new(repositories: Repositories) -> Self {
        Self {
            user_stories: Arc::new(UserStoryService::new(repositories.clone())),
            acceptance_criteria: Arc::new(AcceptanceCriteriaService::new(repositories)),
        }
    }
}
