# 🌉 Universal MCP Gateway

**Docker for AI Tools** - Connect anything to AI agents in one command.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![TypeScript](https://img.shields.io/badge/typescript-%23007ACC.svg?style=flat&logo=typescript&logoColor=white)](https://www.typescriptlang.org/)

Universal MCP Gateway is an open-source gateway that automatically converts REST APIs, GraphQL APIs, databases, SDKs, and CLI tools into MCP (Model Context Protocol) compatible agent tools. Stop writing custom integrations—let the gateway do it for you.

## 🎯 Why Universal MCP Gateway?

**The Problem:** MCP is rapidly becoming the standard interoperability layer for AI agents, but developers must manually create MCP servers for every integration.

**The Solution:** Universal MCP Gateway automatically converts any data source or API into MCP-compatible tools that AI agents can use immediately.

## ✨ Features

- 🔄 **Auto-generate MCP tools** from Swagger/OpenAPI specifications
- 🗄️ **Database introspection** for PostgreSQL, MySQL, SQLite
- 🖥️ **CLI-to-MCP wrapper** - expose any command-line tool
- 🔐 **Authentication middleware** (OAuth2, API Keys, JWT)
- 🔒 **Secure execution sandbox** with WASM isolation
- 📊 **Auto-generated documentation** and web UI
- 🤖 **Multi-agent compatible** (Claude, GPT, Gemini, etc.)
- 🐳 **Docker support** for one-command deployment
- 🔌 **Plugin architecture** for custom adapters

## 🚀 Quick Start

### Using Docker (Recommended)

```bash
# Pull and run the gateway
docker run -p 8080:8080 -v ./config:/config universalmcp/gateway

# Or use docker-compose
docker-compose up
```

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/universal-mcp-gateway.git
cd universal-mcp-gateway

# Build the Rust backend
cd gateway-core
cargo build --release

# Install TypeScript SDK
cd ../sdk
npm install
npm run build

# Run the gateway
./target/release/mcp-gateway --config config.yaml
```

## 📖 Usage Examples

### Connect a REST API

```yaml
# config.yaml
sources:
  - name: "github-api"
    type: "openapi"
    spec: "https://api.github.com/openapi.json"
    auth:
      type: "bearer"
      token: "${GITHUB_TOKEN}"
```

### Connect a Database

```yaml
sources:
  - name: "postgres-db"
    type: "postgresql"
    connection: "postgresql://user:pass@localhost/mydb"
    tables: ["users", "orders", "products"]
    operations: ["read", "write"]
```

### Expose a CLI Tool

```yaml
sources:
  - name: "kubectl"
    type: "cli"
    command: "kubectl"
    allowed_args: ["get", "describe", "logs"]
    sandbox: true
```

### Use the TypeScript SDK

```typescript
import { MCPGateway } from '@universal-mcp/sdk';

const gateway = new MCPGateway({
  endpoint: 'http://localhost:8080',
  apiKey: process.env.GATEWAY_API_KEY
});

// Auto-discover available tools
const tools = await gateway.listTools();

// Execute a tool
const result = await gateway.executeTool('github-api.get-user', {
  username: 'octocat'
});
```

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    AI Agent (Claude, GPT, etc.)         │
└────────────────────┬────────────────────────────────────┘
                     │ MCP Protocol
┌────────────────────▼────────────────────────────────────┐
│              Universal MCP Gateway                       │
│  ┌──────────────────────────────────────────────────┐  │
│  │  Core Engine (Rust)                              │  │
│  │  - Protocol Handler                              │  │
│  │  - Tool Registry                                 │  │
│  │  - Execution Sandbox (WASM)                      │  │
│  └──────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────┐  │
│  │  Adapters                                        │  │
│  │  - OpenAPI/Swagger  - GraphQL  - PostgreSQL     │  │
│  │  - MySQL  - CLI Wrapper  - Custom Plugins       │  │
│  └──────────────────────────────────────────────────┘  │
└────────────────────┬────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        │            │            │
   ┌────▼───┐   ┌───▼────┐   ┌──▼─────┐
   │  APIs  │   │   DBs  │   │  CLIs  │
   └────────┘   └────────┘   └────────┘
```

## 🛠️ Configuration

The gateway uses a YAML configuration file to define sources and behavior:

```yaml
# Gateway settings
gateway:
  host: "0.0.0.0"
  port: 8080
  log_level: "info"
  
# Security settings
security:
  enable_sandbox: true
  max_execution_time: 30s
  rate_limit: 100/minute
  
# Sources to expose as MCP tools
sources:
  - name: "my-api"
    type: "openapi"
    spec: "./specs/api.yaml"
    auth:
      type: "api_key"
      header: "X-API-Key"
      value: "${API_KEY}"
    
  - name: "my-database"
    type: "postgresql"
    connection: "${DATABASE_URL}"
    tables: ["*"]
    read_only: true
```

## 🔐 Security

- **Sandboxed Execution:** All tool executions run in isolated WASM environments
- **Authentication:** Support for OAuth2, API Keys, JWT, and custom auth
- **Rate Limiting:** Built-in rate limiting per source and per agent
- **Audit Logging:** Complete audit trail of all tool executions
- **Secret Management:** Integration with environment variables and secret stores

## 🧩 Adapters

### Built-in Adapters

- **OpenAPI/Swagger:** Auto-generate tools from OpenAPI 3.0+ specs
- **GraphQL:** Introspect GraphQL schemas and expose queries/mutations
- **PostgreSQL:** Full database introspection and query generation
- **MySQL:** MySQL database adapter with introspection
- **SQLite:** Lightweight SQLite adapter
- **CLI:** Wrap command-line tools with safety controls
- **REST:** Generic REST API adapter

### Custom Adapters

Create custom adapters using the plugin API:

```rust
use mcp_gateway::adapter::{Adapter, Tool, ExecutionContext};

pub struct MyCustomAdapter;

impl Adapter for MyCustomAdapter {
    fn name(&self) -> &str { "my-adapter" }
    
    async fn discover_tools(&self) -> Vec<Tool> {
        // Return available tools
    }
    
    async fn execute(&self, tool: &str, params: Value, ctx: ExecutionContext) -> Result<Value> {
        // Execute the tool
    }
}
```

## 📊 Web UI

The gateway includes a built-in web UI for:

- Viewing all available tools
- Testing tool execution
- Monitoring usage and performance
- Managing configurations
- Viewing audit logs

Access at `http://localhost:8080/ui`

## 🤝 Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for details.

### Development Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js and npm
# (Use your preferred method)

# Clone and setup
git clone https://github.com/yourusername/universal-mcp-gateway.git
cd universal-mcp-gateway

# Run tests
cargo test
cd sdk && npm test

# Run in development mode
cargo run -- --config examples/config.yaml
```

## 📚 Documentation

- [Full Documentation](https://docs.universal-mcp-gateway.dev)
- [API Reference](https://docs.universal-mcp-gateway.dev/api)
- [Adapter Development Guide](docs/adapters.md)
- [Security Best Practices](docs/security.md)
- [Examples](examples/)

## 🗺️ Roadmap

- [x] Core gateway engine
- [x] OpenAPI adapter
- [x] PostgreSQL adapter
- [x] CLI wrapper
- [ ] GraphQL adapter
- [ ] MySQL adapter
- [ ] MongoDB adapter
- [ ] gRPC adapter
- [ ] Kubernetes operator
- [ ] Cloud deployment templates (AWS, GCP, Azure)
- [ ] Visual flow builder
- [ ] Multi-gateway federation

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built on the [Model Context Protocol](https://modelcontextprotocol.io/)
- Inspired by the need for universal AI agent interoperability
- Thanks to all contributors and the open-source community

## 💬 Community

- [Discord](https://discord.gg/universal-mcp)
- [GitHub Discussions](https://github.com/yourusername/universal-mcp-gateway/discussions)
- [Twitter](https://twitter.com/universalmcp)

---

**Star ⭐ this repository if you find it useful!**
