# Universal MCP Gateway - Project Structure

## Overview

This document provides a comprehensive overview of the project structure and organization.

## Directory Structure

```
universal-mcp-gateway/
├── .github/
│   └── workflows/
│       ├── ci.yml                    # Continuous Integration workflow
│       └── release.yml               # Release automation workflow
│
├── docs/
│   ├── architecture.md               # System architecture documentation
│   ├── adapters.md                   # Adapter development guide
│   ├── api-reference.md              # Complete API documentation
│   ├── deployment.md                 # Deployment guide for various platforms
│   ├── quickstart.md                 # Quick start guide
│   └── security.md                   # Security best practices
│
├── examples/
│   ├── config.yaml                   # Example configuration file
│   ├── rust-custom-adapter.rs        # Example custom adapter in Rust
│   └── typescript-usage.ts           # Example SDK usage in TypeScript
│
├── gateway-core/                     # Rust backend (main gateway)
│   ├── src/
│   │   ├── adapters/
│   │   │   ├── mod.rs               # Adapter trait and types
│   │   │   ├── openapi.rs           # OpenAPI/Swagger adapter
│   │   │   ├── postgresql.rs        # PostgreSQL database adapter
│   │   │   ├── cli.rs               # CLI wrapper adapter
│   │   │   ├── registry.rs          # Adapter registry
│   │   │   └── factory.rs           # Adapter factory
│   │   ├── api.rs                   # HTTP API server
│   │   ├── auth.rs                  # Authentication module
│   │   ├── config.rs                # Configuration management
│   │   ├── gateway.rs               # Main gateway orchestration
│   │   ├── main.rs                  # Entry point
│   │   ├── protocol.rs              # MCP protocol implementation
│   │   └── sandbox.rs               # WASM sandbox for secure execution
│   ├── tests/                       # Integration tests
│   └── Cargo.toml                   # Rust dependencies
│
├── sdk/                             # TypeScript SDK
│   ├── src/
│   │   ├── client.ts               # Main gateway client
│   │   ├── types.ts                # TypeScript type definitions
│   │   ├── errors.ts               # Error classes
│   │   └── index.ts                # SDK entry point
│   ├── tests/                      # SDK tests
│   ├── package.json                # Node.js dependencies
│   ├── tsconfig.json               # TypeScript configuration
│   ├── jest.config.js              # Jest test configuration
│   ├── .eslintrc.json              # ESLint configuration
│   └── .prettierrc                 # Prettier configuration
│
├── Cargo.toml                       # Workspace configuration
├── Dockerfile                       # Docker image definition
├── docker-compose.yml               # Docker Compose configuration
├── .dockerignore                    # Docker ignore patterns
├── .gitignore                       # Git ignore patterns
├── Makefile                         # Build automation
├── rustfmt.toml                     # Rust formatting configuration
├── README.md                        # Main project documentation
├── CONTRIBUTING.md                  # Contribution guidelines
├── LICENSE                          # MIT License
├── CHANGELOG.md                     # Version history
└── PROJECT_STRUCTURE.md             # This file

```

## Core Components

### 1. Gateway Core (Rust)

**Location:** `gateway-core/`

The heart of the system, written in Rust for performance and safety.

**Key Files:**
- `main.rs` - Application entry point, CLI argument parsing
- `gateway.rs` - Main gateway orchestration and initialization
- `config.rs` - YAML configuration parsing and validation
- `api.rs` - HTTP/REST API server using Axum
- `protocol.rs` - MCP JSON-RPC protocol implementation
- `auth.rs` - Authentication and authorization
- `sandbox.rs` - WASM-based execution sandbox

**Adapters:**
- `adapters/mod.rs` - Adapter trait definition
- `adapters/openapi.rs` - OpenAPI/Swagger adapter
- `adapters/postgresql.rs` - PostgreSQL database adapter
- `adapters/cli.rs` - CLI tool wrapper
- `adapters/registry.rs` - Adapter registry
- `adapters/factory.rs` - Adapter factory pattern

### 2. TypeScript SDK

**Location:** `sdk/`

Client library for interacting with the gateway from Node.js/TypeScript applications.

**Key Files:**
- `client.ts` - Main MCPGateway class
- `types.ts` - TypeScript interfaces and types
- `errors.ts` - Custom error classes
- `index.ts` - Public API exports

### 3. Documentation

**Location:** `docs/`

Comprehensive documentation covering all aspects of the project.

**Files:**
- `architecture.md` - System design and architecture
- `adapters.md` - How to create custom adapters
- `api-reference.md` - Complete API documentation
- `deployment.md` - Deployment guides for various platforms
- `quickstart.md` - Getting started guide
- `security.md` - Security best practices

### 4. Examples

**Location:** `examples/`

Working examples demonstrating various features.

**Files:**
- `config.yaml` - Example gateway configuration
- `rust-custom-adapter.rs` - Custom adapter implementation
- `typescript-usage.ts` - SDK usage examples

### 5. CI/CD

**Location:** `.github/workflows/`

Automated testing and release workflows.

**Files:**
- `ci.yml` - Continuous integration (tests, linting)
- `release.yml` - Automated releases (binaries, Docker, npm)

## Technology Stack

### Backend (Rust)
- **Runtime:** Tokio (async runtime)
- **Web Framework:** Axum
- **Serialization:** Serde (JSON, YAML)
- **Database:** SQLx (PostgreSQL, MySQL, SQLite)
- **HTTP Client:** Reqwest
- **WASM Runtime:** Wasmtime
- **Logging:** Tracing

### SDK (TypeScript)
- **HTTP Client:** Axios
- **Testing:** Jest
- **Linting:** ESLint
- **Formatting:** Prettier
- **Build:** TypeScript Compiler

### Infrastructure
- **Containerization:** Docker
- **Orchestration:** Kubernetes, Docker Compose
- **CI/CD:** GitHub Actions

## Build System

### Rust
```bash
cargo build          # Debug build
cargo build --release # Release build
cargo test           # Run tests
cargo clippy         # Linting
cargo fmt            # Formatting
```

### TypeScript
```bash
npm install          # Install dependencies
npm run build        # Build SDK
npm test             # Run tests
npm run lint         # Linting
npm run format       # Formatting
```

### Make Targets
```bash
make install         # Install all dependencies
make build           # Build everything
make test            # Run all tests
make run             # Run gateway locally
make docker-build    # Build Docker image
make docker-run      # Run with Docker Compose
```

## Configuration

### Gateway Configuration (`config.yaml`)

```yaml
gateway:
  host: "0.0.0.0"
  port: 8080
  log_level: "info"

security:
  enable_sandbox: true
  max_execution_time: 30
  rate_limit: "100/minute"

sources:
  - name: "adapter-name"
    type: "adapter-type"
    # adapter-specific config
```

### Environment Variables

- `DATABASE_URL` - Database connection string
- `API_KEY` - API authentication key
- `RUST_LOG` - Log level override
- Custom adapter variables (e.g., `GITHUB_TOKEN`)

## Testing

### Unit Tests
- Rust: `cargo test`
- TypeScript: `npm test`

### Integration Tests
- Located in `gateway-core/tests/`
- Run with `cargo test --test integration`

### End-to-End Tests
- Manual testing with example configurations
- Automated in CI pipeline

## Deployment

### Docker
```bash
docker build -t universalmcp/gateway .
docker run -p 8080:8080 universalmcp/gateway
```

### Kubernetes
```bash
kubectl apply -f k8s/deployment.yaml
kubectl apply -f k8s/service.yaml
```

### Cloud Platforms
- AWS: ECS, Fargate, EC2
- GCP: Cloud Run, GKE
- Azure: Container Instances, AKS

See `docs/deployment.md` for detailed instructions.

## Development Workflow

1. **Clone Repository**
   ```bash
   git clone https://github.com/yourusername/universal-mcp-gateway.git
   cd universal-mcp-gateway
   ```

2. **Install Dependencies**
   ```bash
   make install
   ```

3. **Make Changes**
   - Edit code in `gateway-core/src/` or `sdk/src/`
   - Add tests
   - Update documentation

4. **Test**
   ```bash
   make test
   ```

5. **Format and Lint**
   ```bash
   make format
   make lint
   ```

6. **Submit PR**
   - Create feature branch
   - Commit changes
   - Push and create pull request

## Release Process

1. Update version in `Cargo.toml` and `package.json`
2. Update `CHANGELOG.md`
3. Create git tag: `git tag v0.1.0`
4. Push tag: `git push origin v0.1.0`
5. GitHub Actions automatically:
   - Builds binaries for multiple platforms
   - Publishes Docker images
   - Publishes npm package
   - Creates GitHub release

## Contributing

See `CONTRIBUTING.md` for detailed contribution guidelines.

## License

MIT License - See `LICENSE` file for details.

## Support

- Documentation: https://docs.universal-mcp-gateway.dev
- GitHub Issues: https://github.com/yourusername/universal-mcp-gateway/issues
- Discord: https://discord.gg/universal-mcp
- Twitter: https://twitter.com/universalmcp
