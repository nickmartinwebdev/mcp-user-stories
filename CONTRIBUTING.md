# Contributing to MCP User Stories

Thank you for your interest in contributing to the MCP User Stories library! This document provides guidelines and information for contributors.

## Getting Started

### Prerequisites

- Rust 1.75 or later
- SQLite for local development
- Git

### Development Setup

1. Fork the repository
2. Clone your fork:
   ```bash
   git clone https://github.com/your-username/mcp-user-stories.git
   cd mcp-user-stories
   ```
3. Install dependencies:
   ```bash
   cargo build
   ```
4. Run tests to ensure everything works:
   ```bash
   cargo test
   ```

## Development Guidelines

### Code Style

- Follow standard Rust formatting with `cargo fmt`
- Use meaningful variable and function names
- Add documentation comments for public APIs
- Keep functions focused and small when possible

### Testing

- Write tests for new functionality
- Maintain test coverage above 80%
- Use descriptive test names that explain the scenario
- Include both positive and negative test cases
- Test error conditions and edge cases

### Database Changes

- Always create a new migration for database schema changes
- Test migrations both up and down (where applicable)
- Update SQLx data files when adding new queries
- Ensure compatibility with both SQLite and Cloudflare D1

## Contribution Process

### Before You Start

1. Check existing issues to see if your idea is already being worked on
2. For significant changes, open an issue first to discuss the approach
3. Make sure there's alignment on the solution before implementing

### Making Changes

1. Create a feature branch from `main`:
   ```bash
   git checkout -b feature/your-feature-name
   ```
2. Make your changes following the guidelines above
3. Add or update tests as needed
4. Update documentation if you're changing public APIs
5. Run the full test suite:
   ```bash
   cargo test --all-features
   ```
6. Check formatting and linting:
   ```bash
   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings -A dead-code
   ```

### Submitting Changes

1. Push your branch to your fork
2. Create a Pull Request against the `main` branch
3. Fill out the PR template with:
   - Clear description of what the PR does
   - Why the change is needed
   - Any breaking changes
   - Testing approach used
4. Link any related issues

### PR Review Process

- All PRs require at least one review from a maintainer
- CI must pass (tests, linting, formatting)
- Breaking changes require special consideration and documentation
- We aim to review PRs within 2-3 business days

## Types of Contributions

### Bug Fixes
- Always include a test that reproduces the bug
- Keep fixes minimal and focused
- Update documentation if the bug was in documented behavior

### New Features
- Discuss the feature in an issue first
- Include comprehensive tests
- Add documentation and examples
- Consider backward compatibility

### Documentation
- Fix typos, improve clarity, add examples
- Keep documentation up to date with code changes
- Consider adding more detailed examples for complex features

### Performance Improvements
- Include benchmarks showing the improvement
- Ensure no functional regressions
- Document any trade-offs made

## Multi-Agent LLM Integration

This library is specifically designed for multi-agent LLM systems. When contributing:

- Consider how features will be used by AI agents
- Design APIs to be clear and self-documenting
- Include validation that helps prevent common AI mistakes
- Think about batch operations and efficiency
- Consider how features support iterative refinement workflows

## Code of Conduct

- Be respectful and inclusive in all interactions
- Focus on constructive feedback
- Help newcomers get started
- Assume positive intent

## Getting Help

- Check the documentation and README first
- Look through existing issues
- Join discussions in GitHub Discussions
- For questions about contributing, open an issue with the "question" label

## License

By contributing to this project, you agree that your contributions will be licensed under the MIT License.

Thank you for contributing to MCP User Stories!