#!/bin/bash

# User Stories MCP Server Runner Script
# This script builds and runs the MCP server with proper error handling

set -e  # Exit on any error

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Default values
DATABASE_URL="${DATABASE_URL:-sqlite://./user_stories.db}"
BUILD_MODE="${BUILD_MODE:-release}"

echo -e "${GREEN}User Stories MCP Server${NC}"
echo "==============================="

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    print_error "Cargo (Rust) is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -f "src/bin/mcp-server.rs" ]; then
    print_error "Please run this script from the mcp-user-stories project root directory"
    exit 1
fi

# Build the project
print_status "Building MCP server..."
if [ "$BUILD_MODE" = "debug" ]; then
    cargo build --bin mcp-server
    BINARY_PATH="./target/debug/mcp-server"
else
    cargo build --release --bin mcp-server
    BINARY_PATH="./target/release/mcp-server"
fi

if [ $? -eq 0 ]; then
    print_status "Build completed successfully"
else
    print_error "Build failed"
    exit 1
fi

# Check if binary exists
if [ ! -f "$BINARY_PATH" ]; then
    print_error "Binary not found at $BINARY_PATH"
    exit 1
fi

# Display configuration
echo
print_status "Configuration:"
echo "  Database URL: $DATABASE_URL"
echo "  Build Mode: $BUILD_MODE"
echo "  Binary Path: $BINARY_PATH"
echo

# Run database migrations/setup if needed
print_status "Initializing database..."
export DATABASE_URL="$DATABASE_URL"

# Start the server
print_status "Starting MCP server..."
print_warning "Server is running and communicating via stdin/stdout"
print_warning "Press Ctrl+C to stop the server"
echo

# Export environment variables and run the server
exec "$BINARY_PATH"
