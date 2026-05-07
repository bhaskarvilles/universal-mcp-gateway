export class GatewayError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'GatewayError';
  }
}

export class ToolNotFoundError extends Error {
  constructor(toolName: string) {
    super(`Tool not found: ${toolName}`);
    this.name = 'ToolNotFoundError';
  }
}

export class ExecutionError extends Error {
  constructor(message: string) {
    super(`Tool execution failed: ${message}`);
    this.name = 'ExecutionError';
  }
}

export class AuthenticationError extends Error {
  constructor(message: string) {
    super(`Authentication failed: ${message}`);
    this.name = 'AuthenticationError';
  }
}
