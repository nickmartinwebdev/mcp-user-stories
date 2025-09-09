# User Stories Management System

[![CI](https://github.com/nickmartinwebdev/mcp-user-stories/workflows/CI/badge.svg)](https://github.com/nickmartinwebdev/mcp-user-stories/actions)
[![Codecov](https://codecov.io/gh/nickmartinwebdev/mcp-user-stories/branch/main/graph/badge.svg)](https://codecov.io/gh/nickmartinwebdev/mcp-user-stories)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](https://www.rust-lang.org)

A comprehensive Rust library for managing user stories and acceptance criteria with CRUD operations, built with SQLx compile-time checked queries and designed for multi-agent LLM systems and Cloudflare D1 database compatibility.

## Features

- 🚀 **Full CRUD Operations** - Create, Read, Update, Delete for user stories and acceptance criteria
- 🔍 **Search & Filtering** - Advanced search capabilities and persona-based filtering
- 📊 **Statistics & Reporting** - Comprehensive analytics and system statistics
- 🛡️ **Type Safety** - Compile-time checked SQL queries with SQLx
- ⚡ **Async/Await** - Full async support with Tokio runtime
- 🗃️ **Database Migrations** - Automatic schema management
- ✅ **Business Logic Validation** - Input validation and business rules enforcement
- 📦 **Cloudflare D1 Compatible** - Works with SQLite and Cloudflare D1 databases

## Quick Start

### Prerequisites

- Rust 2021 edition or later
- SQLite (for local development)

### Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mcp-user-stories = "0.1.0"
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "migrate"] }
tokio = { version = "1.47", features = ["full"] }
```

### Basic Usage

```rust
use mcp_user_stories::{
    database::initialize_database,
    models::{CreateUserStoryRequest, CreateAcceptanceCriteriaRequest},
    repositories::Repositories,
    services::Services,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database with migrations
    let pool = initialize_database("sqlite://./user_stories.db").await?;
    
    // Setup repositories and services
    let repositories = Repositories::new(pool);
    let services = Services::new(repositories);
    
    // Create a user story
    let user_story_request = CreateUserStoryRequest {
        id: "US-001".to_string(),
        title: "User Login".to_string(),
        description: "As a user, I want to login to access my account".to_string(),
        persona: "End User".to_string(),
    };
    
    let user_story = services.user_stories.create(user_story_request).await?;
    println!("Created: {}", user_story.title);
    
    Ok(())
}
```

## Data Model

The system manages two main entities:

### User Stories

```rust
pub struct UserStory {
    pub id: String,           // Format: "US-XXX"
    pub title: String,        // Max 200 characters
    pub description: String,  // Max 2000 characters
    pub persona: String,      // User persona/role
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Acceptance Criteria

```rust
pub struct AcceptanceCriteria {
    pub id: String,           // Format: "AC-XXX"
    pub user_story_id: String,// Reference to user story
    pub description: String,  // Max 1000 characters
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## API Reference

### User Story Operations

#### Create
```rust
// Create single user story
let story = services.user_stories.create(request).await?;

// Create user story with acceptance criteria
let story_with_criteria = services.user_stories
    .create_with_criteria(story_request, criteria_requests).await?;
```

#### Read
```rust
// Get by ID
let story = services.user_stories.get_by_id("US-001").await?;

// Get with acceptance criteria
let story_with_criteria = services.user_stories
    .get_with_criteria("US-001").await?;

// Get all
let all_stories = services.user_stories.get_all().await?;

// Get with pagination
let page = services.user_stories.get_paginated(10, 0).await?;

// Get by persona
let customer_stories = services.user_stories
    .get_by_persona("Customer").await?;
```

#### Update
```rust
let update_request = UpdateUserStoryRequest {
    title: Some("New Title".to_string()),
    description: None, // Keep existing
    persona: Some("Updated Persona".to_string()),
};

let updated = services.user_stories.update("US-001", update_request).await?;
```

#### Delete
```rust
services.user_stories.delete("US-001").await?;
```

#### Search
```rust
let results = services.user_stories.search("authentication").await?;
```

### Acceptance Criteria Operations

#### Create
```rust
// Single criteria
let criteria = services.acceptance_criteria.create(request).await?;

// Batch create
let criteria_list = services.acceptance_criteria
    .create_batch(requests).await?;
```

#### Read
```rust
// Get by ID
let criteria = services.acceptance_criteria.get_by_id("AC-001").await?;

// Get all for a user story
let story_criteria = services.acceptance_criteria
    .get_by_user_story_id("US-001").await?;

// Get all
let all_criteria = services.acceptance_criteria.get_all().await?;
```

#### Update
```rust
let update_request = UpdateAcceptanceCriteriaRequest {
    description: Some("Updated description".to_string()),
};

let updated = services.acceptance_criteria
    .update("AC-001", update_request).await?;
```

#### Delete
```rust
// Delete single criteria
services.acceptance_criteria.delete("AC-001").await?;

// Delete all for a user story
let deleted_count = services.acceptance_criteria
    .delete_by_user_story_id("US-001").await?;
```

### Statistics

```rust
// User story statistics
let stats = services.user_stories.get_statistics().await?;
println!("Total stories: {}", stats.total_stories);
println!("Average criteria per story: {:.2}", stats.avg_criteria_per_story);

// Acceptance criteria statistics
let criteria_stats = services.acceptance_criteria.get_statistics().await?;
```

## Database Schema

The system uses two tables with a foreign key relationship:

```sql
CREATE TABLE user_stories (
    id TEXT PRIMARY KEY NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    persona TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE acceptance_criteria (
    id TEXT PRIMARY KEY NOT NULL,
    user_story_id TEXT NOT NULL,
    description TEXT NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_story_id) REFERENCES user_stories(id) ON DELETE CASCADE
);
```

## Validation Rules

### User Stories
- ID must start with "US-"
- Title: 1-200 characters
- Description: 1-2000 characters
- Persona: cannot be empty

### Acceptance Criteria
- ID must start with "AC-"
- Description: 1-1000 characters
- Maximum 20 criteria per user story
- Must belong to existing user story

## Examples

### Running Examples

```bash
# Run the comprehensive example
cargo run --example comprehensive_example

# Run the main application
cargo run
```

### Example: E-commerce User Story

```rust
let user_story = CreateUserStoryRequest {
    id: "US-105".to_string(),
    title: "Quick Product Filtering".to_string(),
    description: "As a frequent shopper, I want to filter search results by price, brand, and customer rating so that I can quickly find the best product for me without scrolling through pages of irrelevant items.".to_string(),
    persona: "Frequent Shopper".to_string(),
};

let acceptance_criteria = vec![
    CreateAcceptanceCriteriaRequest {
        id: "AC-1".to_string(),
        user_story_id: "US-105".to_string(),
        description: "Given I am on the search results page for a product, I see filter options for Price, Brand, and Average Rating.".to_string(),
    },
    CreateAcceptanceCriteriaRequest {
        id: "AC-2".to_string(),
        user_story_id: "US-105".to_string(),
        description: "When I set a minimum and maximum price, only products within that price range are shown.".to_string(),
    },
    // ... more criteria
];

let result = services.user_stories
    .create_with_criteria(user_story, acceptance_criteria)
    .await?;
```

## Error Handling

The library uses custom error types with detailed messages:

```rust
use mcp_user_stories::services::UserStoryServiceError;

match services.user_stories.create(request).await {
    Ok(story) => println!("Created: {}", story.title),
    Err(UserStoryServiceError::AlreadyExists { id }) => {
        eprintln!("Story {} already exists", id);
    },
    Err(UserStoryServiceError::Validation { message }) => {
        eprintln!("Validation error: {}", message);
    },
    Err(e) => eprintln!("Other error: {}", e),
}
```

## Testing

This project implements comprehensive database unit tests using SQLx's advanced testing features, providing excellent test isolation and automatic fixture loading.

### Test Architecture

- **SQLx Test Integration**: Uses `#[sqlx::test]` attribute for automatic test database setup and teardown
- **Fixture System**: Automatic loading of test data from SQL fixture files
- **Test Isolation**: Each test gets its own fresh database instance
- **Migration Automation**: Database migrations are automatically applied before each test
- **Parallel Execution**: Tests can run in parallel safely due to database isolation

### Test Setup

The testing setup includes:

1. **Automatic Migrations**: Each test automatically runs database migrations
2. **Fixture Loading**: Test data is loaded from `fixtures/` directory
3. **Clean State**: Every test starts with a clean database state
4. **No Manual Setup**: No need for manual database cleanup or setup

### Fixture Files

Test fixtures are stored in `fixtures/` directory:

```
fixtures/
├── user_stories.sql        # Sample user stories data
└── acceptance_criteria.sql # Sample acceptance criteria data
```

**User Stories Fixtures** (`fixtures/user_stories.sql`):
```sql
INSERT INTO user_stories (id, title, description, persona, created_at, updated_at) VALUES
('US-001', 'User Login Feature', 'As a registered user, I want to log into the system...', 'Registered User', '2024-01-01 10:00:00', '2024-01-01 10:00:00'),
('US-002', 'User Registration', 'As a new user, I want to create an account...', 'New User', '2024-01-01 11:00:00', '2024-01-01 11:00:00'),
-- ... more test data
```

**Acceptance Criteria Fixtures** (`fixtures/acceptance_criteria.sql`):
```sql
INSERT INTO acceptance_criteria (id, user_story_id, description, created_at, updated_at) VALUES
('AC-001', 'US-001', 'Given I am on the login page, When I enter valid credentials...', '2024-01-01 10:05:00', '2024-01-01 10:05:00'),
('AC-002', 'US-001', 'Given I am on the login page, When I enter invalid credentials...', '2024-01-01 10:06:00', '2024-01-01 10:06:00'),
-- ... more test data
```

### Test Types

#### Repository Tests
Repository tests use the `#[sqlx::test]` attribute with automatic fixture loading:

```rust
#[sqlx::test(fixtures("../../fixtures/user_stories.sql"))]
async fn test_get_user_story_by_id(pool: sqlx::SqlitePool) {
    let repo = UserStoryRepository::new(pool);
    
    // Get a user story from fixtures
    let result = repo.get_by_id("US-001").await;
    assert!(result.is_ok());
    
    let user_story = result.unwrap().unwrap();
    assert_eq!(user_story.id, "US-001");
    assert_eq!(user_story.title, "User Login Feature");
}
```

#### Service Tests
Service tests focus on business logic validation:

```rust
#[tokio::test]
async fn test_create_user_story() {
    let pool = create_test_db().await.unwrap();
    let repos = Repositories::new(pool);
    let service = UserStoryService::new(repos);
    
    let request = CreateUserStoryRequest {
        id: "US-001".to_string(),
        title: "Test Story".to_string(),
        description: "Test description".to_string(),
        persona: "Test User".to_string(),
    };
    
    let result = service.create(request).await;
    assert!(result.is_ok());
}
```

#### Integration Tests
Full end-to-end integration tests:

```rust
#[tokio::test]
async fn test_full_integration() {
    let pool = create_test_db().await.unwrap();
    let repos = Repositories::new(pool);
    let services = Services::new(repos);
    
    // Create user story with acceptance criteria
    let story_result = services.user_stories.create_with_criteria(
        story_request,
        criteria_requests
    ).await;
    
    assert!(story_result.is_ok());
    // ... additional assertions
}
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test module
cargo test repositories::user_story_repository

# Run specific test
cargo test test_create_user_story

# Run tests in parallel (default)
cargo test --jobs 4

# Run repository tests only
cargo test repositories

# Run service tests only
cargo test services

# Run integration tests only
cargo test tests::
```

### Test Configuration

The project includes SQLx configuration for testing in `sqlx-data.json`:

```json
{
  "database_url": "sqlite::memory:",
  "offline": false
}
```

### Dependencies

Testing requires these Cargo.toml features:

```toml
[dependencies]
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "sqlite", "chrono", "uuid", "migrate", "macros"] }

[dev-dependencies]
tokio-test = "0.4"
```

### Test Coverage

The test suite covers:

- **Repository Layer**: All CRUD operations, search, filtering, batch operations
- **Service Layer**: Business logic, validation rules, error handling
- **Database Layer**: Migrations, schema validation
- **Integration**: End-to-end workflows, cross-service interactions
- **Error Cases**: Invalid input, constraint violations, business rule violations

### Benefits of This Testing Approach

1. **Isolation**: Each test runs in complete isolation with its own database
2. **Speed**: In-memory SQLite databases provide fast test execution
3. **Reliability**: No test interdependencies or shared state issues
4. **Realism**: Tests run against real database with actual SQL queries
5. **Maintainability**: Fixtures provide consistent, versioned test data
6. **Parallel Execution**: Tests can run safely in parallel
7. **Automatic Setup**: No manual database setup or teardown required

### Writing New Tests

When adding new tests:

1. Use `#[sqlx::test]` for repository tests requiring database access
2. Use `#[tokio::test]` for service tests with manual database setup
3. Load appropriate fixtures using the `fixtures()` parameter
4. Follow the AAA pattern (Arrange, Act, Assert)
5. Test both success and error scenarios
6. Include edge cases and boundary conditions

Example of a new repository test:

```rust
#[sqlx::test(fixtures("../../fixtures/user_stories.sql", "../../fixtures/acceptance_criteria.sql"))]
async fn test_new_functionality(pool: sqlx::SqlitePool) {
    // Arrange
    let repo = UserStoryRepository::new(pool);
    
    // Act
    let result = repo.new_method("US-001").await;
    
    // Assert
    assert!(result.is_ok());
    let data = result.unwrap();
    assert_eq!(data.len(), 3);
}
```

This comprehensive testing setup ensures high code quality and reliability while maintaining developer productivity through fast, isolated, and automated tests.

## Database Configuration

### SQLite (Local Development)
```rust
let pool = initialize_database("sqlite://./user_stories.db").await?;
```

### Cloudflare D1 (Production)
```rust
// D1 connection string format
let pool = initialize_database("sqlite://path/to/d1/database").await?;
```

## Performance Considerations

- All SQL queries are compile-time checked with SQLx
- Indexes are automatically created for foreign keys and timestamp columns
- Batch operations are available for bulk inserts
- Pagination support for large datasets

## Architecture

The library follows a clean architecture pattern:

```
┌─────────────────┐
│    Services     │  ← Business Logic & Validation
├─────────────────┤
│  Repositories   │  ← Data Access Layer (CRUD)
├─────────────────┤
│    Database     │  ← Connection & Migration Management
└─────────────────┘
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Run tests (`cargo test`)
5. Commit your changes (`git commit -am 'Add amazing feature'`)
6. Push to the branch (`git push origin feature/amazing-feature`)
7. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Changelog

### Version 0.1.0
- Initial release
- Full CRUD operations for user stories and acceptance criteria
- SQLx compile-time checked queries
- Database migrations
- Search and filtering capabilities
- Statistics and reporting
- Comprehensive validation
- Example applications