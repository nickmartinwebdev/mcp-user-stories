use mcp_user_stories::mcp::run_server;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get database URL from environment variable or use default
    let database_url =
        env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://./user_stories.db".to_string());

    eprintln!(
        "Starting User Stories MCP Server with database: {}",
        database_url
    );

    // Run the MCP server
    run_server(&database_url).await?;

    Ok(())
}
