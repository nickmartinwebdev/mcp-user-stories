mod database;
mod models;
mod repositories;
mod services;

use database::initialize_database;
use repositories::Repositories;
use services::Services;

use anyhow::Result;
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    println!("Starting User Stories Management System");

    // Get database URL from environment or use default
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./user_stories.db".to_string());

    println!("Connecting to database: {}", database_url);

    // Initialize database with migrations
    let pool = initialize_database(&database_url)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to initialize database: {}", e))?;
    println!("Database initialized successfully");

    // Initialize repositories
    let repositories = Repositories::new(pool);

    // Initialize services
    let services = Services::new(repositories);

    // Example usage of the system
    println!("Running example operations...");

    // Create a sample user story
    let user_story_request = models::CreateUserStoryRequest {
        id: "US-105".to_string(),
        title: "Quick Product Filtering".to_string(),
        description: "As a frequent shopper, I want to filter search results by price, brand, and customer rating so that I can quickly find the best product for me without scrolling through pages of irrelevant items.".to_string(),
        persona: "Frequent Shopper".to_string(),
    };

    let acceptance_criteria = vec![
        models::CreateAcceptanceCriteriaRequest {
            id: "AC-1".to_string(),
            user_story_id: "US-105".to_string(),
            description: "Given I am on the search results page for a product, I see filter options for Price, Brand, and Average Rating.".to_string(),
        },
        models::CreateAcceptanceCriteriaRequest {
            id: "AC-2".to_string(),
            user_story_id: "US-105".to_string(),
            description: "When I set a minimum and maximum price, only products within that price range are shown.".to_string(),
        },
        models::CreateAcceptanceCriteriaRequest {
            id: "AC-3".to_string(),
            user_story_id: "US-105".to_string(),
            description: "When I select one or more specific brands, only products from those brands are shown.".to_string(),
        },
        models::CreateAcceptanceCriteriaRequest {
            id: "AC-4".to_string(),
            user_story_id: "US-105".to_string(),
            description: "When I select a minimum star rating (e.g., 4 stars and up), only products with an average rating equal to or greater than that value are shown.".to_string(),
        },
        models::CreateAcceptanceCriteriaRequest {
            id: "AC-5".to_string(),
            user_story_id: "US-105".to_string(),
            description: "I can combine multiple filters (e.g., Brand 'Nike' AND Price '$50-$100' AND Rating '4+ stars') and the results update accordingly.".to_string(),
        },
        models::CreateAcceptanceCriteriaRequest {
            id: "AC-6".to_string(),
            user_story_id: "US-105".to_string(),
            description: "If no products match the selected filters, a clear message is displayed: 'No products found. Try adjusting your filters.'".to_string(),
        },
    ];

    match services
        .user_stories
        .create_with_criteria(user_story_request.clone(), acceptance_criteria)
        .await
    {
        Ok(story_with_criteria) => {
            println!(
                "✅ Created user story: {}",
                story_with_criteria.user_story.title
            );
            println!("   ID: {}", story_with_criteria.user_story.id);
            println!(
                "   Description: {}",
                story_with_criteria.user_story.description
            );
            println!("   Persona: {}", story_with_criteria.user_story.persona);
            println!(
                "   Acceptance Criteria ({} items):",
                story_with_criteria.acceptance_criteria.len()
            );

            for (index, criteria) in story_with_criteria.acceptance_criteria.iter().enumerate() {
                println!(
                    "   {}. {} - {}",
                    index + 1,
                    criteria.id,
                    criteria.description
                );
            }
        }
        Err(e) => {
            println!("❌ Failed to create user story: {}", e);
        }
    }

    // Get statistics
    match services.user_stories.get_statistics().await {
        Ok(stats) => {
            println!("\n📊 System Statistics:");
            println!("   Total User Stories: {}", stats.total_stories);
            println!("   Total Acceptance Criteria: {}", stats.total_criteria);
            println!("   Number of Personas: {}", stats.personas_count);
            println!(
                "   Average Criteria per Story: {:.2}",
                stats.avg_criteria_per_story
            );

            if !stats.stories_by_persona.is_empty() {
                println!("   Stories by Persona:");
                for (persona, count) in &stats.stories_by_persona {
                    println!("     - {}: {}", persona, count);
                }
            }
        }
        Err(e) => {
            println!("❌ Failed to get statistics: {}", e);
        }
    }

    // Example of CRUD operations
    println!("\n🔄 Demonstrating CRUD operations...");

    // Read - Get all user stories
    match services.user_stories.get_all().await {
        Ok(stories) => {
            println!("📖 Found {} user stories in the system", stories.len());
        }
        Err(e) => {
            println!("❌ Failed to retrieve user stories: {}", e);
        }
    }

    // Update example
    let update_request = models::UpdateUserStoryRequest {
        title: Some("Enhanced Product Filtering System".to_string()),
        description: None,
        persona: None,
    };

    match services
        .user_stories
        .update(&user_story_request.id, update_request)
        .await
    {
        Ok(updated_story) => {
            println!("✏️  Updated user story title to: {}", updated_story.title);
        }
        Err(e) => {
            println!("❌ Failed to update user story: {}", e);
        }
    }

    // Search example
    match services.user_stories.search("filter").await {
        Ok(found_stories) => {
            println!(
                "🔍 Search for 'filter' found {} stories",
                found_stories.len()
            );
        }
        Err(e) => {
            println!("❌ Failed to search user stories: {}", e);
        }
    }

    println!("\n✅ User Stories Management System is running successfully!");
    println!("Database file: {}", database_url);

    Ok(())
}
