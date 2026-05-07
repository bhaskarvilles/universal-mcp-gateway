# Architecture

## Overview

Universal MCP Gateway is designed as a modular, extensible system that bridges the gap between AI agents and various data sources through the Model Context Protocol (MCP).

## Core Components

### 1. Gateway Core (Rust)

The heart of the system, written in Rust for performance and safety.

**Responsibilities:**
- HTTP/WebSocket server for MCP protocol
- Adapter lifecycle management
- Request routing and execution
- Security and sandboxing
- Rate limiting and authentication

**Key Modules:**
- `gateway.rs` - Main gateway orchestration
- `config.rs` - Configuration management
- `api.rs` - HTTP API endpoints
- `protocol.rs` - MCP protocol implementation

### 2. Adapter System

Adapters translate between MCP tools and external systems.

**Adapter Interface:**
```rust
#[async_trait]
pub trait Adapter: Send + Sync {
    fn name(&self) -> &str;
    async fn initialize(&mut self) -> Result<(), AdapterError>;
    async fn discover_tools(&self) -> Result<Vec<Tool>, AdapterError>;
    async fn execute(&self, tool: &str, params: Value, ctx: ExecutionContext) -> Result<Value, AdapterError>;
    async fn health_check(&self) -> Result<(), AdapterError>;
}
```

**Built-in Adapters:**
- **OpenAPI Adapter** - Converts OpenAPI/Swagger specs to MCP tools
- **PostgreSQL Adapter** - Database introspection and query execution
- **CLI Adapter** - Wraps command-line tools with safety controls

### 3. Protocol Handler

Implements the MCP JSON-RPC protocol.

**Supported Methods:**
- `initialize` - Handshake and capability negotiation
- `tools/list` - Enumerate available tools
- `tools/call` - Execute a tool

**Message Flow:**
```
AI Agent → MCP Request → Protocol Handler → Adapter → External System
                                                ↓
AI Agent ← MCP Response ← Protocol Handler ← Adapter ← External System
```

### 4. Security Layer

**Sandbox:**
- WASM-based execution isolation
- Resource limits (CPU, memory, time)
- Network restrictions

**Authentication:**
- API key validation
- JWT token support
- OAuth2 integration
- Per-adapter authentication

**Authorization:**
- Role-based access control
- Tool-level permissions
- Rate limiting per user/adapter

### 5. TypeScript SDK

Client library for interacting with the gateway.

**Features:**
- Type-safe tool discovery and execution
- Automatic retry and error handling
- Connection pooling
- Promise-based API

## Data Flow

### Tool Discovery

```
1. Gateway starts
2. Load configuration
3. For each source:
   a. Create adapter instance
   b. Call adapter.initialize()
   c. Call adapter.discover_tools()
   d. Register tools in registry
4. Tools available via MCP protocol
```

### Tool Execution

```
1. Agent sends tools/call request
2. Protocol handler validates request
3. Parse adapter.tool name format
4. Look up adapter in registry
5. Create execution context (timeout, user, etc.)
6. Call adapter.execute(tool, params, ctx)
7. Adapter translates to external system call
8. Return result via MCP response
```

## Configuration

Configuration is YAML-based with environment variable substitution:

```yaml
gateway:
  host: "0.0.0.0"
  port: 8080

security:
  enable_sandbox: true
  max_execution_time: 30

sources:
  - name: "my-api"
    type: "openapi"
    spec: "${API_SPEC_URL}"
    auth:
      type: "bearer"
      token: "${API_TOKEN}"
```

## Extensibility

### Custom Adapters

Create custom adapters by implementing the `Adapter` trait:

```rust
pub struct MyAdapter {
    config: MyConfig,
}

#[async_trait]
impl Adapter for MyAdapter {
    // Implement required methods
}
```

### Plugin System

Future: Dynamic plugin loading via WASM modules.

## Performance Considerations

- **Async I/O:** All I/O operations are async using Tokio
- **Connection Pooling:** Database connections are pooled
- **Caching:** Tool schemas cached after discovery
- **Parallel Execution:** Multiple tool calls can execute concurrently

## Deployment Architectures

### Single Instance

```
┌─────────┐
│ Gateway │
└────┬────┘
     │
  ┌──┴──┐
  │ DBs │
  └─────┘
```

### Load Balanced

```
┌─────────────┐
│ Load Balancer│
└──────┬──────┘
   ┌───┴───┐
   │       │
┌──▼──┐ ┌──▼──┐
│ GW1 │ │ GW2 │
└──┬──┘ └──┬──┘
   └───┬───┘
    ┌──▼──┐
    │ DBs │
    └─────┘
```

### Federated

```
┌─────────┐     ┌─────────┐
│ Gateway │────▶│ Gateway │
│  (Main) │     │ (Region)│
└─────────┘     └─────────┘
```

## Security Model

### Threat Model

**Protected Against:**
- Malicious tool parameters
- Resource exhaustion
- Unauthorized access
- Code injection
- Data exfiltration

**Assumptions:**
- Gateway runs in trusted environment
- Configuration is secure
- Network between gateway and data sources is trusted

### Security Boundaries

1. **Agent ↔ Gateway:** Authentication, rate limiting
2. **Gateway ↔ Adapter:** Sandboxing, validation
3. **Adapter ↔ External System:** Authentication, encryption

## Monitoring and Observability

- **Structured Logging:** JSON logs with tracing
- **Metrics:** Prometheus-compatible metrics
- **Health Checks:** `/health` endpoint
- **Audit Trail:** All tool executions logged

## Future Enhancements

- GraphQL adapter
- gRPC adapter
- MongoDB adapter
- Multi-gateway federation
- Visual flow builder
- Real-time tool updates
- Streaming responses
- Tool composition
