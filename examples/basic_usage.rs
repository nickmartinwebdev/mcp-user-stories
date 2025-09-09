//! Basic Usage Example
//!
//! This example demonstrates the core functionality of the mcp-user-stories library,
//! including creating user stories, adding acceptance criteria, and querying data.

use mcp_user_stories::{
    database::initialize_database,
    models::{CreateAcceptanceCriteriaRequest, CreateUserStoryRequest},
    repositories::Repositories,
    services::Services,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 MCP User Stories - Basic Usage Example");

    // Initialize database with migrations (using in-memory database for example)
    let database_url = "sqlite::memory:";
    let pool = initialize_database(database_url).await?;
    println!("✅ Database initialized successfully");

    // Setup repositories and services
    let repositories = Repositories::new(pool);
    let services = Services::new(repositories);
    println!("✅ Services initialized");

    // Create a user story
    println!("\n📝 Creating user story...");
    let user_story_request = CreateUserStoryRequest {
        id: "US-001".to_string(),
        title: "User Login".to_string(),
        description: "As a registered user, I want to log into the system so that I can access my personal dashboard and manage my account".to_string(),
        persona: "Registered User".to_string(),
    };

    let user_story = services.user_stories.create(user_story_request).await?;
    println!(
        "✅ Created user story: {} - {}",
        user_story.id, user_story.title
    );

    // Create acceptance criteria for the user story
    println!("\n📋 Adding acceptance criteria...");
    let criteria_requests = vec![
        CreateAcceptanceCriteriaRequest {
            id: "AC-001-1".to_string(),
            user_story_id: "US-001".to_string(),
            description: "Given I am on the login page, When I enter valid credentials, Then I should be redirected to my dashboard".to_string(),
        },
        CreateAcceptanceCriteriaRequest {
            id: "AC-001-2".to_string(),
            user_story_id: "US-001".to_string(),
            description: "Given I am on the login page, When I enter invalid credentials, Then I should see an error message".to_string(),
        },
        CreateAcceptanceCriteriaRequest {
            id: "AC-001-3".to_string(),
            user_story_id: "US-001".to_string(),
            description: "Given I have failed to login 3 times, When I attempt to login again, Then my account should be temporarily locked".to_string(),
        },
    ];

    for criteria_request in criteria_requests {
        let criteria = services
            .acceptance_criteria
            .create(criteria_request)
            .await?;
        println!("  ✅ Added criteria: {}", criteria.id);
    }

    // Create another user story with criteria in one operation
    println!("\n📝 Creating user story with criteria in one operation...");
    let user_story_request_2 = CreateUserStoryRequest {
        id: "US-002".to_string(),
        title: "Password Reset".to_string(),
        description: "As a user who forgot my password, I want to reset it so that I can regain access to my account".to_string(),
        persona: "Registered User".to_string(),
    };

    let criteria_requests_2 = vec![
        CreateAcceptanceCriteriaRequest {
            id: "AC-002-1".to_string(),
            user_story_id: "US-002".to_string(),
            description: "Given I am on the forgot password page, When I enter my email address, Then I should receive a password reset email".to_string(),
        },
        CreateAcceptanceCriteriaRequest {
            id: "AC-002-2".to_string(),
            user_story_id: "US-002".to_string(),
            description: "Given I click the reset link in the email, When I enter a new password, Then my password should be updated".to_string(),
        },
    ];

    let story_with_criteria = services
        .user_stories
        .create_with_criteria(user_story_request_2, criteria_requests_2)
        .await?;

    println!(
        "✅ Created user story with criteria: {} - {}",
        story_with_criteria.user_story.id, story_with_criteria.user_story.title
    );
    println!(
        "  📋 Acceptance criteria count: {}",
        story_with_criteria.acceptance_criteria.len()
    );

    // Query and display all user stories
    println!("\n📊 Querying all user stories...");
    let all_stories = services.user_stories.get_all().await?;
    for story in &all_stories {
        println!(
            "  📝 {} - {} (Persona: {})",
            story.id, story.title, story.persona
        );
    }

    // Search for stories
    println!("\n🔍 Searching for stories containing 'login'...");
    let search_results = services.user_stories.search("login").await?;
    for story in &search_results {
        println!("  🔍 Found: {} - {}", story.id, story.title);
    }

    // Get stories by persona
    println!("\n👤 Getting stories for 'Registered User' persona...");
    let persona_stories = services
        .user_stories
        .get_by_persona("Registered User")
        .await?;
    for story in &persona_stories {
        println!("  👤 {} - {}", story.id, story.title);
    }

    // Get detailed view of a story with its criteria
    println!("\n📄 Getting detailed view of US-001...");
    let detailed_story = services.user_stories.get_with_criteria("US-001").await?;
    println!(
        "  📝 Story: {} - {}",
        detailed_story.user_story.id, detailed_story.user_story.title
    );
    println!("  📋 Acceptance Criteria:");
    for criteria in &detailed_story.acceptance_criteria {
        println!("    ✓ {}: {}", criteria.id, criteria.description);
    }

    // Get system statistics
    println!("\n📊 System Statistics:");
    let stats = services.user_stories.get_statistics().await?;
    println!("  📝 Total Stories: {}", stats.total_stories);
    println!("  📋 Total Acceptance Criteria: {}", stats.total_criteria);
    println!("  👥 Number of Personas: {}", stats.personas_count);
    println!(
        "  📊 Average Criteria per Story: {:.2}",
        stats.avg_criteria_per_story
    );
    println!("  📈 Stories by Persona:");
    for (persona, count) in &stats.stories_by_persona {
        println!("    👤 {}: {} stories", persona, count);
    }

    println!("\n🎉 Example completed successfully!");
    Ok(())
}
