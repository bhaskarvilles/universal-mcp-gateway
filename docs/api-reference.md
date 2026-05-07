# API Reference

## HTTP Endpoints

### GET /

Get gateway information.

**Response:**
```json
{
  "name": "Universal MCP Gateway",
  "version": "0.1.0",
  "endpoints": {
    "health": "/health",
    "mcp": "/mcp",
    "adapters": "/adapters",
    "tools": "/tools"
  }
}
```

### GET /health

Health check endpoint.

**Response:**
```json
{
  "status": "healthy",
  "adapters": 3,
  "adapter_names": ["github-api", "app-database", "kubectl"]
}
```

### GET /adapters

List all registered adapters.

**Response:**
```json
{
  "adapters": ["github-api", "app-database", "kubectl"]
}
```

### GET /tools

List all available tools from all adapters.

**Response:**
```json
{
  "tools": [
    {
      "adapter": "github-api",
      "name": "get-user",
      "full_name": "github-api.get-user",
      "description": "Get GitHub user information",
      "parameters": {
        "type": "object",
        "properties": {
          "username": {
            "type": "string",
            "description": "GitHub username"
          }
        },
        "required": ["username"]
      }
    }
  ],
  "count": 15
}
```

### POST /mcp

MCP protocol endpoint.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "tools/call",
  "params": {
    "name": "github-api.get-user",
    "arguments": {
      "username": "octocat"
    }
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"login\":\"octocat\",\"id\":1,...}"
      }
    ]
  }
}
```

## MCP Protocol Methods

### initialize

Initialize connection with the gateway.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "method": "initialize",
  "params": {
    "protocolVersion": "1.0",
    "clientInfo": {
      "name": "my-client",
      "version": "1.0.0"
    }
  }
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": "1",
  "result": {
    "protocolVersion": "1.0",
    "serverInfo": {
      "name": "universal-mcp-gateway",
      "version": "0.1.0"
    },
    "capabilities": {
      "tools": true
    }
  }
}
```

### tools/list

List all available tools.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": "2",
  "method": "tools/list"
}
```

**Response:**
```json
{
  "jsonrpc": "2.0",
  "id": "2",
  "result": {
    "tools": [
      {
        "name": "github-api.get-user",
        "description": "Get GitHub user information",
        "inputSchema": {
          "type": "object",
          "properties": {
            "username": {
              "type": "string",
              "description": "GitHub username"
            }
          },
          "required": ["username"]
        }
      }
    ]
  }
}
```

### tools/call

Execute a tool.

**Request:**
```json
{
  "jsonrpc": "2.0",
  "id": "3",
  "method": "tools/call",
  "params": {
    "name": "github-api.get-user",
    "arguments": {
      "username": "octocat"
    }
  }
}
```

**Response (Success):**
```json
{
  "jsonrpc": "2.0",
  "id": "3",
  "result": {
    "content": [
      {
        "type": "text",
        "text": "{\"login\":\"octocat\",...}"
      }
    ]
  }
}
```

**Response (Error):**
```json
{
  "jsonrpc": "2.0",
  "id": "3",
  "error": {
    "code": -32603,
    "message": "Tool execution failed: API rate limit exceeded"
  }
}
```

## Error Codes

| Code | Message | Description |
|------|---------|-------------|
| -32700 | Parse error | Invalid JSON |
| -32600 | Invalid request | Invalid request object |
| -32601 | Method not found | Method does not exist |
| -32602 | Invalid params | Invalid method parameters |
| -32603 | Internal error | Internal server error |

## TypeScript SDK

### MCPGateway

Main client class for interacting with the gateway.

#### Constructor

```typescript
new MCPGateway(config: GatewayConfig)
```

**Parameters:**
- `config.endpoint` (string): Gateway URL
- `config.apiKey` (string, optional): API key for authentication
- `config.timeout` (number, optional): Request timeout in milliseconds

**Example:**
```typescript
const gateway = new MCPGateway({
  endpoint: 'http://localhost:8080',
  apiKey: process.env.API_KEY,
  timeout: 30000
});
```

#### Methods

##### initialize()

Initialize connection with the gateway.

```typescript
await gateway.initialize(): Promise<void>
```

##### listTools()

List all available tools.

```typescript
await gateway.listTools(): Promise<Tool[]>
```

**Returns:**
```typescript
interface Tool {
  name: string;
  description: string;
  inputSchema: {
    type: string;
    properties: Record<string, PropertySchema>;
    required?: string[];
  };
}
```

##### executeTool()

Execute a tool.

```typescript
await gateway.executeTool(
  toolName: string,
  parameters?: Record<string, unknown>
): Promise<ToolResult>
```

**Parameters:**
- `toolName`: Fully qualified tool name (adapter.tool)
- `parameters`: Tool execution parameters

**Returns:**
```typescript
interface ToolResult {
  success: boolean;
  data?: unknown;
  error?: string;
}
```

##### getTool()

Get information about a specific tool.

```typescript
await gateway.getTool(toolName: string): Promise<Tool | null>
```

##### listAdapters()

List all available adapters.

```typescript
await gateway.listAdapters(): Promise<string[]>
```

##### healthCheck()

Check gateway health.

```typescript
await gateway.healthCheck(): Promise<{
  status: string;
  adapters: number;
}>
```

## Configuration Schema

### Gateway Configuration

```yaml
gateway:
  host: string              # Host to bind to (default: "0.0.0.0")
  port: number              # Port to bind to (default: 8080)
  log_level: string         # Log level (default: "info")
  enable_ui: boolean        # Enable web UI (default: false)
```

### Security Configuration

```yaml
security:
  enable_sandbox: boolean   # Enable WASM sandbox (default: true)
  max_execution_time: number # Max execution time in seconds (default: 30)
  rate_limit: string        # Rate limit (default: "100/minute")
  allowed_origins: string[] # CORS allowed origins (default: ["*"])
```

### Source Configuration

```yaml
sources:
  - name: string            # Unique adapter name (required)
    type: string            # Adapter type (required)
    # Adapter-specific configuration
```

#### OpenAPI Adapter

```yaml
- name: "my-api"
  type: "openapi"
  spec: string              # OpenAPI spec URL or path (required)
  base_url: string          # API base URL (optional)
  auth:                     # Authentication (optional)
    type: string            # "bearer", "api_key", "oauth2"
    token: string           # For bearer auth
    header: string          # For API key auth
    value: string           # For API key auth
```

#### PostgreSQL Adapter

```yaml
- name: "my-database"
  type: "postgresql"
  connection: string        # Connection string (required)
  tables: string[]          # Tables to expose (default: ["*"])
  read_only: boolean        # Read-only mode (default: true)
```

#### CLI Adapter

```yaml
- name: "my-cli"
  type: "cli"
  command: string           # Command to execute (required)
  allowed_args: string[]    # Allowed arguments (optional)
  sandbox: boolean          # Enable sandbox (default: true)
```

## Examples

See the [examples](../examples/) directory for complete examples:

- [TypeScript usage](../examples/typescript-usage.ts)
- [Custom adapter](../examples/rust-custom-adapter.rs)
- [Configuration examples](../examples/config.yaml)
