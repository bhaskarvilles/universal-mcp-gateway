export interface GatewayConfig {
  endpoint: string;
  apiKey?: string;
  timeout?: number;
}

export interface Tool {
  name: string;
  description: string;
  inputSchema: {
    type: string;
    properties: Record<string, PropertySchema>;
    required?: string[];
  };
}

export interface PropertySchema {
  type: string;
  description?: string;
  default?: unknown;
}

export interface ToolResult {
  success: boolean;
  data?: unknown;
  error?: string;
}

export interface MCPRequest {
  jsonrpc: string;
  id: string;
  method: string;
  params?: Record<string, unknown>;
}

export interface MCPResponse {
  jsonrpc: string;
  id: string;
  result?: {
    tools?: Tool[];
    content?: Array<{ type: string; text: string }>;
    [key: string]: unknown;
  };
  error?: {
    code: number;
    message: string;
    data?: unknown;
  };
}

export interface AdapterInfo {
  name: string;
  type: string;
  status: 'active' | 'inactive' | 'error';
}
