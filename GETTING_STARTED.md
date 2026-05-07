# Getting Started with Universal MCP Gateway

Welcome to Universal MCP Gateway! This guide will help you get up and running quickly.

## What is Universal MCP Gateway?

Universal MCP Gateway is an open-source tool that automatically converts REST APIs, GraphQL APIs, databases, SDKs, and CLI tools into MCP (Model Context Protocol) compatible tools that AI agents can use. Think of it as "Docker for AI Tools."

## Why Use It?

- **Stop writing custom integrations** - Automatically generate MCP tools from existing APIs and databases
- **Universal compatibility** - Works with Claude, GPT, Gemini, and any MCP-compatible agent
- **Production-ready** - Built with Rust for performance, includes security sandboxing, authentication, and rate limiting
- **Extensible** - Easy to create custom adapters for any data source

## Quick Start (5 minutes)

### Option 1: Docker (Easiest)

1. **Create a configuration file** (`config.yaml`):

```yaml
gateway:
  host: "0.0.0.0"
  port: 8080

security:
  enable_sandbox: true

sources:
  # Example: Expose a public API
  - name: "petstore"
    type: "openapi"
    spec: "https://petstore3.swagger.io/api/v3/openapi.json"
```

2. **Run the gateway**:

```bash
docker run -p 8080:8080 -v $(pwd)/config.yaml:/app/config.yaml universalmcp/gateway
```

3. **Test it**:

```bash
# Check health
curl http://localhost:8080/health

# List available tools
curl http://localhost:8080/tools
```

That's it! Your gateway is now running and exposing the Petstore API as MCP tools.

### Option 2: From Source

1. **Install prerequisites**:
   - Rust 1.75+ ([Install Rust](https://rustup.rs/))
   - Node.js 18+ (for SDK)

2. **Clone and build**:

```bash
git clone https://github.com/yourusername/universal-mcp-gateway.git
cd universal-mcp-gateway
make build
```

3. **Run**:

```bash
make run
```

## Your First Integration

Let's connect a real API and database to AI agents.

### Example 1: GitHub API

```yaml
sources:
  - name: "github"
    type: "openapi"
    spec: "https://raw.githubusercontent.com/github/rest-api-description/main/descriptions/api.github.com/api.github.com.json"
    auth:
      type: "bearer"
      token: "${GITHUB_TOKEN}"
```

Set your token:
```bash
export GITHUB_TOKEN="ghp_your_token_here"
```

Now AI agents can:
- Get user information
- List repositories
- Create issues
- And more!

### Example 2: PostgreSQL Database

```yaml
sources:
  - name: "app-db"
    type: "postgresql"
    connection: "${DATABASE_URL}"
    tables: ["users", "orders", "products"]
    read_only: true  # Safe for AI agents
```

Set your connection:
```bash
export DATABASE_URL="postgresql://user:pass@localhost/mydb"
```

Now AI agents can:
- Query user data
- Analyze orders
- Search products
- Generate reports

### Example 3: CLI Tools

```yaml
sources:
  - name: "kubectl"
    type: "cli"
    command: "kubectl"
    allowed_args: ["get", "describe", "logs"]
    sandbox: true
```

Now AI agents can:
- Get pod status
- Describe resources
- View logs
- (But can't delete or modify - safe!)

## Using the TypeScript SDK

Install the SDK:

```bash
npm install @universal-mcp/sdk
```

Use it in your code:

```typescript
import { MCPGateway } from '@universal-mcp/sdk';

const gateway = new MCPGateway({
  endpoint: 'http://localhost:8080'
});

// Initialize
await gateway.initialize();

// List available tools
const tools = await gateway.listTools();
console.log('Available tools:', tools.map(t => t.name));

// Execute a tool
const result = await gateway.executeTool('github.get-user', {
  username: 'octocat'
});
console.log('Result:', result.data);
```

## Connecting to AI Agents

### Claude Desktop

Add to your Claude Desktop config:

```json
{
  "mcpServers": {
    "universal-gateway": {
      "command": "curl",
      "args": ["-X", "POST", "http://localhost:8080/mcp"],
      "env": {}
    }
  }
}
```

### Custom Integration

Use the MCP protocol directly:

```bash
curl -X POST http://localhost:8080/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": "1",
    "method": "tools/list"
  }'
```

## Common Use Cases

### 1. Internal Tools for AI Assistants

Connect your company's internal APIs and databases so AI assistants can help employees:

```yaml
sources:
  - name: "crm"
    type: "openapi"
    spec: "${CRM_API_SPEC}"
  
  - name: "analytics-db"
    type: "postgresql"
    connection: "${ANALYTICS_DB_URL}"
    read_only: true
```

### 2. DevOps Automation

Let AI agents help with infrastructure management:

```yaml
sources:
  - name: "kubectl"
    type: "cli"
    command: "kubectl"
    allowed_args: ["get", "describe", "logs"]
  
  - name: "aws"
    type: "cli"
    command: "aws"
    allowed_args: ["s3", "ec2", "describe"]
```

### 3. Data Analysis

Connect databases for AI-powered analytics:

```yaml
sources:
  - name: "warehouse"
    type: "postgresql"
    connection: "${WAREHOUSE_URL}"
    tables: ["sales", "customers", "products"]
    read_only: true
```

### 4. API Aggregation

Combine multiple APIs into a single interface:

```yaml
sources:
  - name: "github"
    type: "openapi"
    spec: "https://api.github.com/openapi.json"
  
  - name: "jira"
    type: "openapi"
    spec: "${JIRA_API_SPEC}"
  
  - name: "slack"
    type: "openapi"
    spec: "${SLACK_API_SPEC}"
```

## Configuration Tips

### Environment Variables

Use environment variables for secrets:

```yaml
sources:
  - name: "api"
    auth:
      token: "${API_TOKEN}"  # Reads from environment
```

### Security Settings

Adjust security based on your needs:

```yaml
security:
  enable_sandbox: true           # Isolate executions
  max_execution_time: 30         # Timeout in seconds
  rate_limit: "100/minute"       # Prevent abuse
  allowed_origins: ["*"]         # CORS settings
```

### Multiple Adapters

Connect as many sources as you need:

```yaml
sources:
  - name: "api-1"
    type: "openapi"
    spec: "..."
  
  - name: "api-2"
    type: "openapi"
    spec: "..."
  
  - name: "db-1"
    type: "postgresql"
    connection: "..."
  
  - name: "cli-1"
    type: "cli"
    command: "..."
```

## Next Steps

### Learn More

- [Architecture Overview](docs/architecture.md) - Understand how it works
- [API Reference](docs/api-reference.md) - Complete API documentation
- [Security Guide](docs/security.md) - Best practices for production

### Extend the Gateway

- [Create Custom Adapters](docs/adapters.md) - Build adapters for new data sources
- [Examples](examples/) - See working examples

### Deploy to Production

- [Deployment Guide](docs/deployment.md) - Deploy to AWS, GCP, Azure, Kubernetes
- [Docker Compose](docker-compose.yml) - Multi-container setup

## Troubleshooting

### Gateway won't start

```bash
# Check logs
docker logs <container-id>

# Verify config
cat config.yaml
```

### Can't connect to database

```bash
# Test connection
psql "${DATABASE_URL}"

# Check firewall
telnet localhost 5432
```

### Tools not appearing

```bash
# Check adapter status
curl http://localhost:8080/adapters

# View detailed health
curl http://localhost:8080/health
```

## Getting Help

- **Documentation:** [docs/](docs/)
- **Examples:** [examples/](examples/)
- **Issues:** [GitHub Issues](https://github.com/yourusername/universal-mcp-gateway/issues)
- **Discord:** [Join our community](https://discord.gg/universal-mcp)
- **Twitter:** [@universalmcp](https://twitter.com/universalmcp)

## Contributing

We welcome contributions! See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## What's Next?

The gateway is under active development. Upcoming features:

- [ ] GraphQL adapter
- [ ] MySQL adapter
- [ ] MongoDB adapter
- [ ] gRPC adapter
- [ ] Visual flow builder
- [ ] Multi-gateway federation
- [ ] Real-time tool updates

Star ⭐ the repository to follow development!

---

**Ready to connect your tools to AI agents? Let's go! 🚀**
