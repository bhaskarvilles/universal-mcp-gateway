# Universal MCP Gateway SDK

TypeScript/JavaScript client library for [Universal MCP Gateway](https://github.com/bhaskarvilles/universal-mcp-gateway).

## Installation

```bash
npm install universal-mcp-gateway-sdk
```

## Quick Start

```typescript
import { MCPClient } from 'universal-mcp-gateway-sdk';

// Create client
const client = new MCPClient({
  baseURL: 'http://localhost:8080',
  apiKey: 'your-api-key' // optional
});

// List available tools
const tools = await client.listTools();
console.log('Available tools:', tools);

// Execute a tool
const result = await client.executeTool('petstore.getPetById', {
  petId: 123
});
console.log('Result:', result);
```

## API Reference

### Constructor

```typescript
new MCPClient(config: MCPClientConfig)
```

**Config Options:**
- `baseURL` (required): Gateway server URL
- `apiKey` (optional): API key for authentication
- `timeout` (optional): Request timeout in milliseconds (default: 30000)

### Methods

#### `listTools()`

List all available tools from the gateway.

```typescript
const tools = await client.listTools();
// Returns: Tool[]
```

#### `executeTool(toolName, parameters)`

Execute a specific tool with parameters.

```typescript
const result = await client.executeTool('toolName', {
  param1: 'value1',
  param2: 'value2'
});
// Returns: ToolResult
```

#### `getToolInfo(toolName)`

Get detailed information about a specific tool.

```typescript
const info = await client.getToolInfo('petstore.getPetById');
// Returns: Tool
```

#### `healthCheck()`

Check if the gateway is healthy and responding.

```typescript
const isHealthy = await client.healthCheck();
// Returns: boolean
```

## Error Handling

The SDK provides typed error classes for better error handling:

```typescript
import { 
  MCPClient, 
  MCPError, 
  NetworkError, 
  AuthenticationError, 
  ToolNotFoundError 
} from 'universal-mcp-gateway-sdk';

try {
  const result = await client.executeTool('myTool', { param: 'value' });
} catch (error) {
  if (error instanceof AuthenticationError) {
    console.error('Authentication failed:', error.message);
  } else if (error instanceof ToolNotFoundError) {
    console.error('Tool not found:', error.message);
  } else if (error instanceof NetworkError) {
    console.error('Network error:', error.message);
  } else if (error instanceof MCPError) {
    console.error('MCP error:', error.message);
  }
}
```

## Examples

### Basic Usage

```typescript
import { MCPClient } from 'universal-mcp-gateway-sdk';

const client = new MCPClient({
  baseURL: 'http://localhost:8080'
});

// List all tools
const tools = await client.listTools();
tools.forEach(tool => {
  console.log(`${tool.name}: ${tool.description}`);
});

// Execute a tool
const result = await client.executeTool('database.query', {
  sql: 'SELECT * FROM users LIMIT 10'
});
console.log(result.data);
```

### With Authentication

```typescript
const client = new MCPClient({
  baseURL: 'https://gateway.example.com',
  apiKey: process.env.MCP_API_KEY,
  timeout: 60000 // 60 seconds
});

const result = await client.executeTool('api.createUser', {
  name: 'John Doe',
  email: 'john@example.com'
});
```

### Error Handling

```typescript
import { MCPClient, ToolNotFoundError } from 'universal-mcp-gateway-sdk';

const client = new MCPClient({
  baseURL: 'http://localhost:8080'
});

try {
  const result = await client.executeTool('nonexistent.tool', {});
} catch (error) {
  if (error instanceof ToolNotFoundError) {
    console.error('Tool does not exist');
  } else {
    console.error('Unexpected error:', error);
  }
}
```

### Async/Await with Multiple Tools

```typescript
const client = new MCPClient({
  baseURL: 'http://localhost:8080'
});

async function processData() {
  // Execute multiple tools in sequence
  const users = await client.executeTool('database.getUsers', {});
  
  for (const user of users.data) {
    const profile = await client.executeTool('api.getUserProfile', {
      userId: user.id
    });
    console.log(profile);
  }
}

processData().catch(console.error);
```

### Promise-based Usage

```typescript
const client = new MCPClient({
  baseURL: 'http://localhost:8080'
});

client.listTools()
  .then(tools => {
    console.log('Available tools:', tools);
    return client.executeTool('myTool', { param: 'value' });
  })
  .then(result => {
    console.log('Result:', result);
  })
  .catch(error => {
    console.error('Error:', error);
  });
```

## TypeScript Support

The SDK is written in TypeScript and includes full type definitions:

```typescript
import { 
  MCPClient, 
  MCPClientConfig, 
  Tool, 
  ToolResult,
  ToolParameter 
} from 'universal-mcp-gateway-sdk';

const config: MCPClientConfig = {
  baseURL: 'http://localhost:8080',
  apiKey: 'my-key',
  timeout: 30000
};

const client: MCPClient = new MCPClient(config);

const tools: Tool[] = await client.listTools();
const result: ToolResult = await client.executeTool('tool', {});
```

## Configuration

### Environment Variables

You can use environment variables for configuration:

```typescript
const client = new MCPClient({
  baseURL: process.env.MCP_GATEWAY_URL || 'http://localhost:8080',
  apiKey: process.env.MCP_API_KEY,
  timeout: parseInt(process.env.MCP_TIMEOUT || '30000')
});
```

### Custom Axios Instance

For advanced use cases, you can access the underlying Axios instance:

```typescript
import { MCPClient } from 'universal-mcp-gateway-sdk';

const client = new MCPClient({
  baseURL: 'http://localhost:8080'
});

// Access the axios instance for custom configuration
// Note: This is an internal implementation detail
```

## Testing

The SDK includes comprehensive tests. To run them:

```bash
npm test
```

## Requirements

- Node.js 16+ or modern browser
- TypeScript 4.5+ (for TypeScript projects)

## Browser Support

The SDK works in modern browsers that support ES6+ and the Fetch API or XMLHttpRequest.

```html
<script type="module">
  import { MCPClient } from 'universal-mcp-gateway-sdk';
  
  const client = new MCPClient({
    baseURL: 'https://gateway.example.com'
  });
  
  client.listTools().then(tools => {
    console.log(tools);
  });
</script>
```

## Contributing

Contributions are welcome! Please see the [Contributing Guide](https://github.com/bhaskarvilles/universal-mcp-gateway/blob/main/CONTRIBUTING.md).

## License

MIT License - see [LICENSE](https://github.com/bhaskarvilles/universal-mcp-gateway/blob/main/LICENSE)

## Links

- **GitHub Repository:** https://github.com/bhaskarvilles/universal-mcp-gateway
- **Documentation:** https://github.com/bhaskarvilles/universal-mcp-gateway/tree/main/docs
- **Issues:** https://github.com/bhaskarvilles/universal-mcp-gateway/issues
- **NPM Package:** https://www.npmjs.com/package/universal-mcp-gateway-sdk

## Support

- **GitHub Issues:** For bug reports and feature requests
- **GitHub Discussions:** For questions and community support
- **Documentation:** Comprehensive guides in the main repository

## Changelog

See [CHANGELOG.md](https://github.com/bhaskarvilles/universal-mcp-gateway/blob/main/CHANGELOG.md) for version history.

---

**Built with ❤️ for the AI community**
