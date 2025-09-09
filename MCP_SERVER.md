# User Stories MCP Server

A Model Context Protocol (MCP) server that provides comprehensive user story and acceptance criteria management capabilities. This server exposes all the functionality of the user stories library through MCP tools, enabling AI agents and other MCP clients to interact with user story data.

**Current Status**: The server is now implemented using the `rmcp` crate for full MCP protocol support. The server provides core user story management functionality and can be used with any MCP-compatible client like Claude Desktop or Cursor.

## Features

The MCP server provides the following tools:

### Currently Available Tools
- `create_user_story` - Create a new user story
- `get_user_story` - Get a user story by ID  
- `get_all_user_stories` - Get all user stories
- `search_user_stories` - Search user stories by text
- `get_user_stories_statistics` - Get statistics about user stories

### Future Enhancements (Planned)
Additional tools for more comprehensive user story management:
- `create_user_story_with_criteria` - Create a user story with acceptance criteria in one operation
- `get_user_story_with_criteria` - Get a user story with its acceptance criteria  
- `get_all_user_stories_with_criteria` - Get all user stories with their acceptance criteria
- `update_user_story` - Update an existing user story
- `delete_user_story` - Delete a user story
- `get_user_stories_by_persona` - Get user stories filtered by persona
- `get_user_stories_grouped_by_persona` - Get user stories grouped by persona

### Acceptance Criteria Management (Future)
- `create_acceptance_criteria` - Create new acceptance criteria for a user story
- `get_acceptance_criteria` - Get acceptance criteria by ID  
- `get_acceptance_criteria_for_story` - Get all acceptance criteria for a user story
- `update_acceptance_criteria` - Update existing acceptance criteria
- `delete_acceptance_criteria` - Delete acceptance criteria

## Installation and Setup

### Prerequisites
- Rust 1.70 or later
- SQLite

### Building the Server
```bash
# Clone the repository
git clone https://github.com/nickmartinwebdev/mcp-user-stories
cd mcp-user-stories

# Build the MCP server binary
cargo build --release --bin mcp-server

# The binary will be available at: target/release/mcp-server
```

### Running the Server

#### Option 1: Direct Execution
```bash
# First, set up the database (only needed once)
sqlx database create --database-url sqlite://./user_stories.db
sqlx migrate run --database-url sqlite://./user_stories.db

# Run with default SQLite database (./user_stories.db)
./target/release/mcp-server

# Run with custom database URL
DATABASE_URL="sqlite:///path/to/your/database.db" ./target/release/mcp-server
```

#### Option 2: Using Cargo
```bash
# First, set up the database (only needed once)
sqlx database create --database-url sqlite://./user_stories.db
sqlx migrate run --database-url sqlite://./user_stories.db

# Run with default database
cargo run --bin mcp-server

# Run with custom database
DATABASE_URL="sqlite:///path/to/your/database.db" cargo run --bin mcp-server
```

## MCP Client Configuration

### Claude Desktop Configuration

Add the server to your Claude Desktop configuration file:

**macOS/Linux**: `~/.config/claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`

```json
{
  "mcpServers": {
    "user-stories": {
      "command": "/path/to/mcp-user-stories/target/release/mcp-server",
      "env": {
        "DATABASE_URL": "sqlite:///path/to/your/user_stories.db"
      }
    }
  }
}
```

### Using with Other MCP Clients

The server communicates via stdin/stdout using the MCP protocol. Any MCP-compatible client can connect to it by spawning the process and communicating through standard I/O.

## Tool Usage Examples

### Creating a User Story
```json
{
  "name": "create_user_story",
  "arguments": {
    "id": "US-001",
    "title": "User Login",
    "description": "As a user, I want to login to access my account",
    "persona": "End User"
  }
}
```

### Searching User Stories
```json
{
  "name": "search_user_stories", 
  "arguments": {
    "query": "login authentication"
  }
}
```

### Getting Statistics
```json
{
  "name": "get_user_stories_statistics",
  "arguments": {}
}
```

## Testing the Server

The MCP server uses stdio transport and expects to communicate with MCP clients. When run directly, it will show:

```bash
./target/debug/mcp-server
# Server starts and waits for MCP client connection
# Use Ctrl+C to stop

# You can verify it starts correctly by checking it exits cleanly with no input:
./target/debug/mcp-server < /dev/null
# Should show startup messages and then exit
```

For interactive testing, you can use the legacy JSON-RPC mode by setting an environment variable:

```bash
# Use legacy JSON-RPC mode for testing
MCP_LEGACY_MODE=1 ./target/debug/mcp-server
```

## Database Configuration

The server uses SQLite by default and automatically handles database initialization and migrations. You can specify a custom database location using the `DATABASE_URL` environment variable.

Supported database URL formats:
- `sqlite://./user_stories.db` (relative path)
- `sqlite:///absolute/path/to/database.db` (absolute path)
- `sqlite://:memory:` (in-memory database, data won't persist)

## Error Handling

The server provides comprehensive error handling:
- **InvalidRequest**: Invalid parameters or malformed data
- **InternalError**: Database errors or system failures
- **NotFound**: Requested resources don't exist

All errors include descriptive messages to help with debugging.

## Development

### Running Tests
```bash
# Run all tests
cargo test

# Run MCP-specific tests
cargo test mcp

# Run with output
cargo test -- --nocapture

# Test the MCP server manually
./test-mcp-server.sh
```

### Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## Troubleshooting

### Common Issues

1. **Database Permission Errors**
   - Ensure the directory containing the database file is writable
   - Check file permissions on the database file
   - Make sure to run database setup first:
     ```bash
     sqlx database create --database-url sqlite://./user_stories.db
     sqlx migrate run --database-url sqlite://./user_stories.db
     ```

2. **Port/Communication Issues**
   - The server uses stdin/stdout for MCP communication
   - Ensure no other processes are interfering with standard I/O

3. **Missing Dependencies**
   - Run `cargo build` to ensure all dependencies are installed
   - Check that SQLite is available on your system
   - Install sqlx-cli if needed: `cargo install sqlx-cli`

4. **MCP Protocol Issues**
   - The server uses the `rmcp` crate for full MCP protocol support
   - If the server immediately exits with "ConnectionClosed", this is normal when no MCP client is connected
   - Make sure your MCP client is configured correctly to connect to the server

### Debug Mode

For debugging, you can run the server with detailed logging:
```bash
RUST_LOG=debug cargo run --bin mcp-server
```

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Support

For issues, questions, or contributions, please visit the [GitHub repository](https://github.com/nickmartinwebdev/mcp-user-stories).