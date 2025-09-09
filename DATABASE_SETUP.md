# Database Setup and SQLx Usage

This document explains how the SQLite database and SQLx compile-time queries are set up and used in the MCP User Stories project.

## Overview

This project uses SQLite as the database backend with SQLx for compile-time verified queries. All SQL queries are validated at compile time, ensuring type safety and preventing runtime SQL errors.

## Database Structure

The database consists of two main tables:

### `user_stories`
- `id` (TEXT, PRIMARY KEY): Unique identifier for the user story
- `title` (TEXT, NOT NULL): Title of the user story
- `description` (TEXT, NOT NULL): Detailed description
- `persona` (TEXT, NOT NULL): User persona associated with the story
- `created_at` (DATETIME, NOT NULL): Timestamp of creation
- `updated_at` (DATETIME, NOT NULL): Timestamp of last update

### `acceptance_criteria`
- `id` (TEXT, PRIMARY KEY): Unique identifier for the criteria
- `user_story_id` (TEXT, NOT NULL): Foreign key to user_stories.id
- `description` (TEXT, NOT NULL): Detailed acceptance criteria description
- `created_at` (DATETIME, NOT NULL): Timestamp of creation
- `updated_at` (DATETIME, NOT NULL): Timestamp of last update

## Setup Instructions

### Initial Setup

1. **Database Creation**: The database is automatically created when you first run the application:
   ```bash
   DATABASE_URL="sqlite://./user_stories.db" cargo run
   ```

2. **Migrations**: Database schema is managed through SQLx migrations located in the `migrations/` directory.

### SQLx Configuration

The project uses SQLx with compile-time query verification. Configuration is stored in `sqlx-data.json`:

```json
{
  "database_url": "sqlite://./user_stories.db",
  "offline": true
}
```

- `database_url`: Points to the SQLite database file
- `offline`: Set to `true` to use pre-compiled query metadata

### Compile-Time Query Preparation

All SQL queries are validated at compile time using `cargo sqlx prepare`. This process:

1. Connects to the database
2. Analyzes all `sqlx::query!` and `sqlx::query_as!` macros
3. Generates type-safe query metadata
4. Stores the metadata in `.sqlx/` directory

## Working with the Database

### Running the Application

```bash
# Set the database URL and run
DATABASE_URL="sqlite://./user_stories.db" cargo run
```

### Regenerating Query Metadata

If you modify SQL queries in the code, you need to regenerate the query metadata:

```bash
# Ensure database exists and has the latest schema
DATABASE_URL="sqlite://./user_stories.db" cargo run

# Regenerate query metadata
DATABASE_URL="sqlite://./user_stories.db" cargo sqlx prepare
```

### Development Workflow

1. **Make database schema changes**: Edit migration files in `migrations/`
2. **Update queries**: Modify SQL queries in repository files
3. **Run the application**: This applies migrations automatically
4. **Prepare queries**: Run `cargo sqlx prepare` to update metadata
5. **Commit changes**: Include both code changes and `.sqlx/` metadata

## Database File Location

The SQLite database file is created at `./user_stories.db` in the project root. This file:
- Contains all user stories and acceptance criteria data
- Should be included in `.gitignore` for development
- Can be backed up or copied for data persistence

## Benefits of This Setup

### Compile-Time Safety
- SQL syntax errors caught at compile time
- Type mismatches prevented before runtime
- Automatic type inference for query results

### Performance
- No runtime SQL parsing overhead
- Optimized query execution
- Connection pooling with SQLx

### Developer Experience
- IDE autocomplete for database fields
- Refactoring safety across SQL queries
- Clear error messages for database issues

## Troubleshooting

### Query Compilation Errors

If you see errors like "unable to open database file" during compilation:

1. Ensure the database file exists: `ls -la user_stories.db`
2. Verify `DATABASE_URL` environment variable is set correctly
3. Run the application once to create/migrate the database
4. Regenerate query metadata: `cargo sqlx prepare`

### Migration Issues

If migrations fail:

1. Check the current migration status: `sqlite3 user_stories.db "SELECT * FROM _sqlx_migrations;"`
2. Manually reset if needed: Drop tables and re-run the application
3. Ensure migration files are properly formatted SQL

### Offline Mode Issues

If compilation fails in offline mode:

1. Set `"offline": false` in `sqlx-data.json`
2. Ensure database is accessible
3. Run `cargo sqlx prepare` to regenerate metadata
4. Set `"offline": true` again

## File Structure

```
mcp-user-stories/
├── migrations/
│   └── 20240101000001_create_user_stories.sql
├── .sqlx/
│   ├── query-*.json (generated metadata files)
├── src/
│   ├── database/
│   ├── repositories/ (contains SQL queries)
│   └── ...
├── sqlx-data.json
├── user_stories.db (created at runtime)
└── DATABASE_SETUP.md (this file)
```

## Next Steps

- Consider adding database seeding for development
- Implement database backup/restore functionality
- Add database performance monitoring
- Consider implementing soft deletes for audit trails