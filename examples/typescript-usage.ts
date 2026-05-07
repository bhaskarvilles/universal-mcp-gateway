import { MCPGateway } from 'universal-mcp-gateway-sdk';

async function main() {
  // Initialize the gateway client
  const gateway = new MCPGateway({
    endpoint: 'http://localhost:8080',
    apiKey: process.env.GATEWAY_API_KEY,
    timeout: 30000,
  });

  try {
    // Initialize connection
    await gateway.initialize();
    console.log('✓ Connected to gateway');

    // Check health
    const health = await gateway.healthCheck();
    console.log(`✓ Gateway healthy with ${health.adapters} adapters`);

    // List all available tools
    const tools = await gateway.listTools();
    console.log(`\n📋 Available tools (${tools.length}):`);
    tools.forEach(tool => {
      console.log(`  - ${tool.name}: ${tool.description}`);
    });

    // Example 1: Query a database
    console.log('\n🗄️  Example 1: Database Query');
    const users = await gateway.executeTool('app-database.query_users', {
      filters: { active: true },
      limit: 10,
    });
    console.log('Users:', users.data);

    // Example 2: Call an API
    console.log('\n🌐 Example 2: API Call');
    const githubUser = await gateway.executeTool('github-api.get-user', {
      username: 'octocat',
    });
    console.log('GitHub user:', githubUser.data);

    // Example 3: Execute a CLI command
    console.log('\n🖥️  Example 3: CLI Execution');
    const pods = await gateway.executeTool('kubectl.execute', {
      args: ['get', 'pods', '-n', 'default'],
    });
    console.log('Kubernetes pods:', pods.data);

    // Example 4: Get tool information
    console.log('\n📖 Example 4: Tool Information');
    const toolInfo = await gateway.getTool('app-database.query_users');
    if (toolInfo) {
      console.log('Tool schema:', JSON.stringify(toolInfo.inputSchema, null, 2));
    }

    // Example 5: List adapters
    console.log('\n🔌 Example 5: List Adapters');
    const adapters = await gateway.listAdapters();
    console.log('Active adapters:', adapters);

  } catch (error) {
    console.error('❌ Error:', error);
    process.exit(1);
  }
}

main();
