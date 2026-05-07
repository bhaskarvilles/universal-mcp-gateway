import axios, { AxiosInstance } from 'axios';
import { Tool, ToolResult, GatewayConfig, MCPRequest, MCPResponse } from './types';
import { GatewayError, ToolNotFoundError, ExecutionError } from './errors';

export class MCPGateway {
  private client: AxiosInstance;
  private endpoint: string;
  private apiKey?: string;

  constructor(config: GatewayConfig) {
    this.endpoint = config.endpoint;
    this.apiKey = config.apiKey;

    this.client = axios.create({
      baseURL: this.endpoint,
      headers: {
        'Content-Type': 'application/json',
        ...(this.apiKey && { 'Authorization': `Bearer ${this.apiKey}` }),
      },
      timeout: config.timeout || 30000,
    });
  }

  /**
   * Initialize connection with the gateway
   */
  async initialize(): Promise<void> {
    const request: MCPRequest = {
      jsonrpc: '2.0',
      id: this.generateId(),
      method: 'initialize',
      params: {
        protocolVersion: '1.0',
        clientInfo: {
          name: '@universal-mcp/sdk',
          version: '0.1.0',
        },
      },
    };

    const response = await this.sendRequest<MCPResponse>(request);
    
    if (response.error) {
      throw new GatewayError(response.error.message);
    }
  }

  /**
   * List all available tools from all adapters
   */
  async listTools(): Promise<Tool[]> {
    const request: MCPRequest = {
      jsonrpc: '2.0',
      id: this.generateId(),
      method: 'tools/list',
    };

    const response = await this.sendRequest<MCPResponse>(request);
    
    if (response.error) {
      throw new GatewayError(response.error.message);
    }

    return response.result?.tools || [];
  }

  /**
   * Execute a tool with the given parameters
   * @param toolName - Fully qualified tool name (adapter.tool)
   * @param parameters - Tool execution parameters
   */
  async executeTool(
    toolName: string,
    parameters: Record<string, unknown> = {}
  ): Promise<ToolResult> {
    const request: MCPRequest = {
      jsonrpc: '2.0',
      id: this.generateId(),
      method: 'tools/call',
      params: {
        name: toolName,
        arguments: parameters,
      },
    };

    const response = await this.sendRequest<MCPResponse>(request);
    
    if (response.error) {
      if (response.error.code === -32602) {
        throw new ToolNotFoundError(toolName);
      }
      throw new ExecutionError(response.error.message);
    }

    return {
      success: true,
      data: response.result,
    };
  }

  /**
   * Get information about a specific tool
   */
  async getTool(toolName: string): Promise<Tool | null> {
    const tools = await this.listTools();
    return tools.find(t => t.name === toolName) || null;
  }

  /**
   * List all available adapters
   */
  async listAdapters(): Promise<string[]> {
    try {
      const response = await this.client.get('/adapters');
      return response.data.adapters || [];
    } catch (error) {
      throw new GatewayError(`Failed to list adapters: ${error}`);
    }
  }

  /**
   * Check gateway health
   */
  async healthCheck(): Promise<{ status: string; adapters: number }> {
    try {
      const response = await this.client.get('/health');
      return response.data;
    } catch (error) {
      throw new GatewayError(`Health check failed: ${error}`);
    }
  }

  /**
   * Send a raw MCP request
   */
  private async sendRequest<T>(request: MCPRequest): Promise<T> {
    try {
      const response = await this.client.post('/mcp', request);
      return response.data;
    } catch (error) {
      if (axios.isAxiosError(error)) {
        throw new GatewayError(
          error.response?.data?.message || error.message
        );
      }
      throw error;
    }
  }

  /**
   * Generate a unique request ID
   */
  private generateId(): string {
    return `${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  }
}
