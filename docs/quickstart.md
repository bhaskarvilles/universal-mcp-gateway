# Quick Start Guide

Get Universal MCP Gateway up and running in 5 minutes.

## Prerequisites

- Docker (recommended) OR
- Rust 1.75+ and Cargo
- Node.js 18+ (for SDK)

## Option 1: Docker (Recommended)

### 1. Create Configuration

Create `config.yaml`:

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
  - name: "example-api"
    type: "openapi"
    spec: "https://petstore3.swagger.io/api/v3/openapi.json"
```

### 2. Run with Docker

```bash
docker run -p 8080:8080 -v $(pwd)/config.yaml:/app/config.yaml universalmcp/gateway
```

### 3. Verify

```bash
curl http://localhost:8080/health
```

## Option 2: From Source

### 1. Clone Repository

```bash
git clone https://github.com/yourusername/universal-mcp-gateway.git
cd universal-mcp-gateway
```

### 2. Build

```bash
make build
```

### 3. Run

```bash
make run
```

## Using the Gateway

### List Available Tools

```bash
curl http://localhost:8080/tools
```

### Execute a Tool (MCP Protocol)

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": "1",
    "method": "tools/call",
    "params": {
      "name": "example-api.getPetById",
      "arguments": {
        "petId": "1"
      }
    }
  }'
```

### Using TypeScript SDK

```bash
npm install @universal-mcp/sdk
```

```typescript
import { MCPGateway } from '@universal-mcp/sdk';

const gateway = new MCPGateway({
  endpoint: 'http://localhost:8080'
});

await gateway.initialize();
const tools = await gateway.listTools();
console.log(tools);
```

## Next Steps

- [Configure adapters](./adapters.md)
- [Security best practices](./security.md)
- [Create custom adapters](./adapters.md#creating-a-custom-adapter)
- [Deploy to production](./deployment.md)

## Common Issues

### Port Already in Use

Change the port in `config.yaml`:

```yaml
gateway:
  port: 8081
```

### Connection Refused

Ensure the gateway is running:

```bash
docker ps  # Check if container is running
curl http://localhost:8080/health
```

### Tool Execution Fails

Check logs:

```bash
docker logs <container-id>
```

## Getting Help

- [Documentation](https://docs.universal-mcp-gateway.dev)
- [GitHub Issues](https://github.com/yourusername/universal-mcp-gateway/issues)
- [Discord Community](https://discord.gg/universal-mcp)
