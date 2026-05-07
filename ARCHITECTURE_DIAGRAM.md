# Universal MCP Gateway - Architecture Diagram

## System Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         AI AGENTS LAYER                                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐               │
│  │  Claude  │  │   GPT    │  │  Gemini  │  │  Custom  │               │
│  │ Desktop  │  │  Agent   │  │  Agent   │  │  Agent   │               │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘  └────┬─────┘               │
└───────┼─────────────┼─────────────┼─────────────┼────────────────────────┘
        │             │             │             │
        └─────────────┴─────────────┴─────────────┘
                      │
                      │ MCP Protocol (JSON-RPC)
                      │
┌─────────────────────▼─────────────────────────────────────────────────────┐
│                   UNIVERSAL MCP GATEWAY                                    │
│                                                                            │
│  ┌──────────────────────────────────────────────────────────────────┐    │
│  │                      HTTP/REST API Server                         │    │
│  │                         (Axum/Rust)                               │    │
│  │                                                                    │    │
│  │  Endpoints:                                                        │    │
│  │  • GET  /health          - Health check                           │    │
│  │  • GET  /tools           - List all tools                         │    │
│  │  • GET  /adapters        - List adapters                          │    │
│  │  • POST /mcp             - MCP protocol endpoint                  │    │
│  └──────────────────────────────────────────────────────────────────┘    │
│                                                                            │
│  ┌──────────────────────────────────────────────────────────────────┐    │
│  │                    MCP Protocol Handler                           │    │
│  │                                                                    │    │
│  │  Methods:                                                          │    │
│  │  • initialize        - Handshake                                  │    │
│  │  • tools/list        - Enumerate tools                            │    │
│  │  • tools/call        - Execute tool                               │    │
│  └──────────────────────────────────────────────────────────────────┘    │
│                                                                            │
│  ┌──────────────────────────────────────────────────────────────────┐    │
│  │                      Security Layer                               │    │
│  │                                                                    │    │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐           │    │
│  │  │ WASM Sandbox │  │ Auth Manager │  │ Rate Limiter │           │    │
│  │  └──────────────┘  └──────────────┘  └──────────────┘           │    │
│  │                                                                    │    │
│  │  • Execution isolation                                            │    │
│  │  • API key / OAuth2 / JWT                                         │    │
│  │  • Per-adapter rate limits                                        │    │
│  └──────────────────────────────────────────────────────────────────┘    │
│                                                                            │
│  ┌──────────────────────────────────────────────────────────────────┐    │
│  │                     Adapter Registry                              │    │
│  │                                                                    │    │
│  │  ┌────────────┐  ┌────────────┐  ┌────────────┐  ┌──────────┐  │    │
│  │  │  OpenAPI   │  │ PostgreSQL │  │    CLI     │  │  Custom  │  │    │
│  │  │  Adapter   │  │  Adapter   │  │  Adapter   │  │ Adapters │  │    │
│  │  └─────┬──────┘  └─────┬──────┘  └─────┬──────┘  └────┬─────┘  │    │
│  └────────┼───────────────┼───────────────┼──────────────┼─────────┘    │
└───────────┼───────────────┼───────────────┼──────────────┼──────────────┘
            │               │               │              │
            │               │               │              │
┌───────────▼───────────────▼───────────────▼──────────────▼──────────────┐
│                        EXTERNAL SYSTEMS                                   │
│                                                                           │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌────────────┐ │
│  │  REST APIs   │  │  Databases   │  │  CLI Tools   │  │   Custom   │ │
│  │              │  │              │  │              │  │  Systems   │ │
│  │ • GitHub     │  │ • PostgreSQL │  │ • kubectl    │  │            │ │
│  │ • Stripe     │  │ • MySQL      │  │ • aws cli    │  │ • gRPC     │ │
│  │ • Slack      │  │ • MongoDB    │  │ • docker     │  │ • GraphQL  │ │
│  │ • Custom     │  │ • Redis      │  │ • git        │  │ • WebSocket│ │
│  └──────────────┘  └──────────────┘  └──────────────┘  └────────────┘ │
└───────────────────────────────────────────────────────────────────────────┘
```

## Data Flow

### Tool Discovery Flow

```
1. Gateway Startup
   │
   ├─→ Load config.yaml
   │
   ├─→ For each source:
   │   │
   │   ├─→ Create adapter instance
   │   │
   │   ├─→ adapter.initialize()
   │   │   │
   │   │   ├─→ Connect to external system
   │   │   ├─→ Authenticate
   │   │   └─→ Introspect schema/API
   │   │
   │   ├─→ adapter.discover_tools()
   │   │   │
   │   │   └─→ Generate Tool definitions
   │   │
   │   └─→ Register in AdapterRegistry
   │
   └─→ Start HTTP server
       │
       └─→ Ready to serve requests
```

### Tool Execution Flow

```
AI Agent Request
   │
   ├─→ POST /mcp
   │   {
   │     "method": "tools/call",
   │     "params": {
   │       "name": "github.get-user",
   │       "arguments": {"username": "octocat"}
   │     }
   │   }
   │
   ├─→ Protocol Handler
   │   │
   │   ├─→ Validate request
   │   ├─→ Parse tool name (adapter.tool)
   │   └─→ Look up adapter
   │
   ├─→ Security Layer
   │   │
   │   ├─→ Check authentication
   │   ├─→ Check rate limits
   │   └─→ Create execution context
   │
   ├─→ Adapter Execution
   │   │
   │   ├─→ Validate parameters
   │   ├─→ Transform to external format
   │   ├─→ Execute in sandbox
   │   │   │
   │   │   └─→ Call external system
   │   │       │
   │   │       └─→ GET https://api.github.com/users/octocat
   │   │
   │   └─→ Transform response
   │
   └─→ Return MCP Response
       {
         "result": {
           "content": [
             {"type": "text", "text": "{...}"}
           ]
         }
       }
```

## Component Details

### 1. Gateway Core (Rust)

```
gateway-core/
├── main.rs              # Entry point, CLI parsing
├── gateway.rs           # Main orchestration
├── config.rs            # Configuration management
├── api.rs               # HTTP server (Axum)
├── protocol.rs          # MCP protocol implementation
├── auth.rs              # Authentication
├── sandbox.rs           # WASM sandbox
└── adapters/
    ├── mod.rs           # Adapter trait
    ├── openapi.rs       # OpenAPI adapter
    ├── postgresql.rs    # PostgreSQL adapter
    ├── cli.rs           # CLI adapter
    ├── registry.rs      # Adapter registry
    └── factory.rs       # Adapter factory
```

### 2. TypeScript SDK

```
sdk/
├── src/
│   ├── client.ts        # MCPGateway class
│   ├── types.ts         # TypeScript types
│   ├── errors.ts        # Error classes
│   └── index.ts         # Public API
└── tests/
    └── client.test.ts   # Unit tests
```

### 3. Configuration

```yaml
# config.yaml structure

gateway:                 # Server settings
  host: "0.0.0.0"
  port: 8080
  log_level: "info"

security:                # Security settings
  enable_sandbox: true
  max_execution_time: 30
  rate_limit: "100/minute"

sources:                 # Adapter configurations
  - name: "adapter-1"
    type: "openapi"
    # adapter-specific config
```

## Adapter Architecture

### Adapter Interface

```rust
#[async_trait]
pub trait Adapter: Send + Sync {
    fn name(&self) -> &str;
    
    async fn initialize(&mut self) -> Result<(), AdapterError>;
    
    async fn discover_tools(&self) -> Result<Vec<Tool>, AdapterError>;
    
    async fn execute(
        &self,
        tool: &str,
        params: Value,
        ctx: ExecutionContext,
    ) -> Result<Value, AdapterError>;
    
    async fn health_check(&self) -> Result<(), AdapterError>;
}
```

### Tool Definition

```rust
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters,
    pub returns: Option<String>,
}

pub struct ToolParameters {
    pub param_type: String,
    pub properties: HashMap<String, ParameterProperty>,
    pub required: Vec<String>,
}
```

## Security Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Security Layers                           │
│                                                              │
│  Layer 1: Network                                            │
│  ┌────────────────────────────────────────────────────┐    │
│  │ • TLS/HTTPS                                        │    │
│  │ • CORS policies                                    │    │
│  │ • Firewall rules                                   │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  Layer 2: Authentication                                     │
│  ┌────────────────────────────────────────────────────┐    │
│  │ • API keys                                         │    │
│  │ • OAuth2 / JWT                                     │    │
│  │ • Per-adapter auth                                 │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  Layer 3: Authorization                                      │
│  ┌────────────────────────────────────────────────────┐    │
│  │ • Rate limiting                                    │    │
│  │ • Tool-level permissions                           │    │
│  │ • Resource quotas                                  │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  Layer 4: Execution                                          │
│  ┌────────────────────────────────────────────────────┐    │
│  │ • WASM sandbox                                     │    │
│  │ • Timeout enforcement                              │    │
│  │ • Input validation                                 │    │
│  └────────────────────────────────────────────────────┘    │
│                                                              │
│  Layer 5: Audit                                              │
│  ┌────────────────────────────────────────────────────┐    │
│  │ • Execution logs                                   │    │
│  │ • Access logs                                      │    │
│  │ • Error tracking                                   │    │
│  └────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Deployment Architecture

### Single Instance

```
┌─────────────────┐
│   Load Balancer │
│   (nginx/Caddy) │
└────────┬────────┘
         │
┌────────▼────────┐
│  MCP Gateway    │
│   (Container)   │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
┌───▼──┐  ┌──▼───┐
│ APIs │  │  DBs │
└──────┘  └──────┘
```

### High Availability

```
┌─────────────────┐
│   Load Balancer │
└────────┬────────┘
         │
    ┌────┴────┐
    │         │
┌───▼──┐  ┌──▼───┐
│ GW-1 │  │ GW-2 │
└───┬──┘  └──┬───┘
    │         │
    └────┬────┘
         │
    ┌────┴────┐
    │         │
┌───▼──┐  ┌──▼───┐
│ APIs │  │  DBs │
└──────┘  └──────┘
```

### Kubernetes

```
┌─────────────────────────────────────┐
│         Kubernetes Cluster          │
│                                     │
│  ┌───────────────────────────────┐ │
│  │        Ingress                │ │
│  └──────────┬────────────────────┘ │
│             │                       │
│  ┌──────────▼────────────────────┐ │
│  │        Service                │ │
│  └──────────┬────────────────────┘ │
│             │                       │
│     ┌───────┴───────┐              │
│     │               │              │
│  ┌──▼──┐  ┌──▼──┐  ┌──▼──┐       │
│  │ Pod │  │ Pod │  │ Pod │       │
│  │ GW  │  │ GW  │  │ GW  │       │
│  └─────┘  └─────┘  └─────┘       │
│                                     │
└─────────────────────────────────────┘
```

## Technology Stack

```
┌─────────────────────────────────────────────────────────┐
│                    Technology Stack                      │
│                                                          │
│  Backend (Rust)                                          │
│  ├─ Runtime: Tokio (async)                              │
│  ├─ Web: Axum                                           │
│  ├─ Database: SQLx                                      │
│  ├─ HTTP: Reqwest                                       │
│  ├─ Serialization: Serde                                │
│  └─ Sandbox: Wasmtime                                   │
│                                                          │
│  SDK (TypeScript)                                        │
│  ├─ Runtime: Node.js                                    │
│  ├─ HTTP: Axios                                         │
│  ├─ Testing: Jest                                       │
│  └─ Build: TypeScript                                   │
│                                                          │
│  Infrastructure                                          │
│  ├─ Container: Docker                                   │
│  ├─ Orchestration: Kubernetes                           │
│  ├─ CI/CD: GitHub Actions                               │
│  └─ Monitoring: Prometheus/Grafana                      │
└─────────────────────────────────────────────────────────┘
```

## Performance Characteristics

```
Metric                  Target          Actual (Estimated)
─────────────────────────────────────────────────────────
Latency (p50)          < 50ms          ~30ms
Latency (p99)          < 200ms         ~150ms
Throughput             > 1000 req/s    ~2000 req/s
Memory Usage           < 100MB         ~80MB
CPU Usage (idle)       < 5%            ~2%
Concurrent Requests    > 1000          ~5000
Startup Time           < 5s            ~2s
```

## Scalability

```
Horizontal Scaling:
┌────────────────────────────────────────────────────┐
│  Requests/sec vs Instances                         │
│                                                    │
│  10,000 ┤                                    ●     │
│   8,000 ┤                              ●           │
│   6,000 ┤                        ●                 │
│   4,000 ┤                  ●                       │
│   2,000 ┤            ●                             │
│       0 └──────┬──────┬──────┬──────┬──────       │
│               1      2      3      4      5        │
│                   Gateway Instances                │
└────────────────────────────────────────────────────┘
```

---

**This architecture is designed for:**
- ✅ High performance
- ✅ Horizontal scalability
- ✅ Security by default
- ✅ Easy extensibility
- ✅ Production reliability
