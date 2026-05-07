# Contributing to Universal MCP Gateway

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing.

## Code of Conduct

We are committed to providing a welcoming and inclusive environment. Please be respectful and professional in all interactions.

## How to Contribute

### Reporting Bugs

1. Check if the bug has already been reported in [Issues](https://github.com/yourusername/universal-mcp-gateway/issues)
2. If not, create a new issue with:
   - Clear title and description
   - Steps to reproduce
   - Expected vs actual behavior
   - Environment details (OS, Rust version, etc.)
   - Relevant logs or error messages

### Suggesting Features

1. Check [Discussions](https://github.com/yourusername/universal-mcp-gateway/discussions) for similar ideas
2. Create a new discussion or issue describing:
   - The problem you're trying to solve
   - Your proposed solution
   - Alternative approaches considered
   - Potential impact on existing functionality

### Pull Requests

1. **Fork the repository** and create a branch from `main`
2. **Make your changes** following our coding standards
3. **Add tests** for new functionality
4. **Update documentation** as needed
5. **Run tests** and ensure they pass
6. **Submit a pull request** with a clear description

## Development Setup

### Prerequisites

- Rust 1.75+ (`rustup install stable`)
- Node.js 18+ and npm
- Docker (optional, for testing)
- PostgreSQL (optional, for database adapter testing)

### Setup Steps

```bash
# Clone your fork
git clone https://github.com/YOUR_USERNAME/universal-mcp-gateway.git
cd universal-mcp-gateway

# Build the Rust core
cd gateway-core
cargo build

# Install SDK dependencies
cd ../sdk
npm install

# Run tests
cd ../gateway-core
cargo test
cd ../sdk
npm test
```

## Project Structure

```
universal-mcp-gateway/
├── gateway-core/          # Rust backend
│   ├── src/
│   │   ├── main.rs       # Entry point
│   │   ├── gateway/      # Core gateway logic
│   │   ├── adapters/     # Built-in adapters
│   │   ├── sandbox/      # WASM sandbox
│   │   └── protocol/     # MCP protocol implementation
│   ├── tests/            # Integration tests
│   └── Cargo.toml
├── sdk/                  # TypeScript SDK
│   ├── src/
│   │   ├── client.ts     # Gateway client
│   │   ├── types.ts      # Type definitions
│   │   └── utils.ts
│   ├── tests/
│   └── package.json
├── web-ui/               # Web interface
├── examples/             # Example configurations
├── docs/                 # Documentation
└── docker/               # Docker configurations
```

## Coding Standards

### Rust

- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Write documentation comments for public APIs
- Add unit tests for new functions
- Use meaningful variable and function names

```rust
// Good
pub async fn execute_tool(
    &self,
    tool_name: &str,
    parameters: Value,
) -> Result<Value, ExecutionError> {
    // Implementation
}

// Bad
pub async fn exec(n: &str, p: Value) -> Result<Value, Error> {
    // Implementation
}
```

### TypeScript

- Follow [TypeScript Style Guide](https://google.github.io/styleguide/tsguide.html)
- Use ESLint and Prettier
- Write JSDoc comments for public APIs
- Use TypeScript strict mode
- Add unit tests for new functionality

```typescript
// Good
/**
 * Executes a tool through the MCP gateway
 * @param toolName - The fully qualified tool name
 * @param parameters - Tool execution parameters
 * @returns The tool execution result
 */
export async function executeTool(
  toolName: string,
  parameters: Record<string, unknown>
): Promise<ToolResult> {
  // Implementation
}
```

## Testing

### Rust Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_openapi_adapter

# Run with output
cargo test -- --nocapture

# Run integration tests
cargo test --test integration
```

### TypeScript Tests

```bash
# Run all tests
npm test

# Run with coverage
npm run test:coverage

# Run specific test
npm test -- client.test.ts
```

## Creating a New Adapter

1. Create a new file in `gateway-core/src/adapters/`
2. Implement the `Adapter` trait
3. Add tests in `gateway-core/tests/adapters/`
4. Update documentation in `docs/adapters.md`
5. Add example configuration in `examples/`

Example adapter structure:

```rust
use async_trait::async_trait;
use serde_json::Value;
use crate::adapter::{Adapter, Tool, ExecutionContext, AdapterError};

pub struct MyAdapter {
    config: MyAdapterConfig,
}

#[async_trait]
impl Adapter for MyAdapter {
    fn name(&self) -> &str {
        "my-adapter"
    }

    async fn initialize(&mut self) -> Result<(), AdapterError> {
        // Setup logic
        Ok(())
    }

    async fn discover_tools(&self) -> Result<Vec<Tool>, AdapterError> {
        // Return available tools
        Ok(vec![])
    }

    async fn execute(
        &self,
        tool: &str,
        params: Value,
        ctx: ExecutionContext,
    ) -> Result<Value, AdapterError> {
        // Execute the tool
        Ok(Value::Null)
    }
}
```

## Documentation

- Update relevant documentation in `docs/`
- Add examples to `examples/`
- Update README.md if adding major features
- Use clear, concise language
- Include code examples where appropriate

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
feat: add GraphQL adapter
fix: resolve connection timeout in PostgreSQL adapter
docs: update adapter development guide
test: add integration tests for CLI wrapper
refactor: simplify tool registry implementation
```

## Review Process

1. All PRs require at least one approval
2. CI checks must pass
3. Code coverage should not decrease
4. Documentation must be updated
5. Breaking changes require discussion

## Release Process

Maintainers handle releases:

1. Update version in `Cargo.toml` and `package.json`
2. Update CHANGELOG.md
3. Create release tag
4. Publish to crates.io and npm
5. Build and push Docker images

## Getting Help

- Join our [Discord](https://discord.gg/universal-mcp)
- Ask in [GitHub Discussions](https://github.com/yourusername/universal-mcp-gateway/discussions)
- Check existing [documentation](docs/)

## Recognition

Contributors are recognized in:
- README.md contributors section
- Release notes
- Project website

Thank you for contributing to Universal MCP Gateway! 🎉
