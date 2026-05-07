import { MCPGateway } from './client';
import { GatewayError, ToolNotFoundError } from './errors';

// Mock axios
jest.mock('axios');
import axios from 'axios';
const mockedAxios = axios as jest.Mocked<typeof axios>;

describe('MCPGateway', () => {
  let gateway: MCPGateway;

  beforeEach(() => {
    gateway = new MCPGateway({
      endpoint: 'http://localhost:8080',
      apiKey: 'test-key',
    });

    // Setup axios mock
    mockedAxios.create.mockReturnValue(mockedAxios as any);
  });

  afterEach(() => {
    jest.clearAllMocks();
  });

  describe('constructor', () => {
    it('should create gateway instance with config', () => {
      expect(gateway).toBeInstanceOf(MCPGateway);
    });

    it('should create gateway without API key', () => {
      const gw = new MCPGateway({
        endpoint: 'http://localhost:8080',
      });
      expect(gw).toBeInstanceOf(MCPGateway);
    });
  });

  describe('initialize', () => {
    it('should send initialize request', async () => {
      mockedAxios.post.mockResolvedValueOnce({
        data: {
          jsonrpc: '2.0',
          id: '1',
          result: {
            protocolVersion: '1.0',
            serverInfo: { name: 'test', version: '0.1.0' },
          },
        },
      });

      await gateway.initialize();

      expect(mockedAxios.post).toHaveBeenCalledWith(
        '/mcp',
        expect.objectContaining({
          method: 'initialize',
        })
      );
    });

    it('should throw error on initialization failure', async () => {
      mockedAxios.post.mockResolvedValueOnce({
        data: {
          jsonrpc: '2.0',
          id: '1',
          error: { code: -1, message: 'Init failed' },
        },
      });

      await expect(gateway.initialize()).rejects.toThrow(GatewayError);
    });
  });

  describe('listTools', () => {
    it('should return list of tools', async () => {
      const mockTools = [
        {
          name: 'test-tool',
          description: 'Test tool',
          inputSchema: { type: 'object', properties: {} },
        },
      ];

      mockedAxios.post.mockResolvedValueOnce({
        data: {
          jsonrpc: '2.0',
          id: '1',
          result: { tools: mockTools },
        },
      });

      const tools = await gateway.listTools();

      expect(tools).toEqual(mockTools);
      expect(mockedAxios.post).toHaveBeenCalledWith(
        '/mcp',
        expect.objectContaining({
          method: 'tools/list',
        })
      );
    });

    it('should return empty array when no tools', async () => {
      mockedAxios.post.mockResolvedValueOnce({
        data: {
          jsonrpc: '2.0',
          id: '1',
          result: {},
        },
      });

      const tools = await gateway.listTools();
      expect(tools).toEqual([]);
    });
  });

  describe('executeTool', () => {
    it('should execute tool successfully', async () => {
      mockedAxios.post.mockResolvedValueOnce({
        data: {
          jsonrpc: '2.0',
          id: '1',
          result: { content: [{ type: 'text', text: 'success' }] },
        },
      });

      const result = await gateway.executeTool('test.tool', { param: 'value' });

      expect(result.success).toBe(true);
      expect(result.data).toBeDefined();
    });

    it('should throw ToolNotFoundError for invalid tool', async () => {
      mockedAxios.post.mockResolvedValueOnce({
        data: {
          jsonrpc: '2.0',
          id: '1',
          error: { code: -32602, message: 'Tool not found' },
        },
      });

      await expect(
        gateway.executeTool('invalid.tool', {})
      ).rejects.toThrow(ToolNotFoundError);
    });
  });

  describe('getTool', () => {
    it('should return tool if found', async () => {
      const mockTool = {
        name: 'test.tool',
        description: 'Test',
        inputSchema: { type: 'object', properties: {} },
      };

      mockedAxios.post.mockResolvedValueOnce({
        data: {
          jsonrpc: '2.0',
          id: '1',
          result: { tools: [mockTool] },
        },
      });

      const tool = await gateway.getTool('test.tool');
      expect(tool).toEqual(mockTool);
    });

    it('should return null if tool not found', async () => {
      mockedAxios.post.mockResolvedValueOnce({
        data: {
          jsonrpc: '2.0',
          id: '1',
          result: { tools: [] },
        },
      });

      const tool = await gateway.getTool('nonexistent.tool');
      expect(tool).toBeNull();
    });
  });

  describe('healthCheck', () => {
    it('should return health status', async () => {
      mockedAxios.get.mockResolvedValueOnce({
        data: { status: 'healthy', adapters: 3 },
      });

      const health = await gateway.healthCheck();

      expect(health.status).toBe('healthy');
      expect(health.adapters).toBe(3);
    });
  });

  describe('listAdapters', () => {
    it('should return list of adapters', async () => {
      mockedAxios.get.mockResolvedValueOnce({
        data: { adapters: ['adapter1', 'adapter2'] },
      });

      const adapters = await gateway.listAdapters();

      expect(adapters).toEqual(['adapter1', 'adapter2']);
    });
  });
});
