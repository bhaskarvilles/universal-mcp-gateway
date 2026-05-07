# Adapter Development Guide

## Overview

Adapters are the bridge between the MCP Gateway and external systems. Each adapter translates between MCP tools and a specific type of data source or API.

## Adapter Interface

All adapters must implement the `Adapter` trait:

```rust
use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait Adapter: Send + Sync {
    /// Returns the adapter name
    fn name(&self) -> &str;
    
    /// Initialize the adapter (connect, authenticate, etc.)
    async fn initialize(&mut self) -> Result<(), AdapterError>;
    
    /// Discover available tools
    async fn discover_tools(&self) -> Result<Vec<Tool>, AdapterError>;
    
    /// Execute a tool
    async fn execute(
        &self,
        tool: &str,
        params: Value,
        ctx: ExecutionContext,
    ) -> Result<Value, AdapterError>;
    
    /// Health check (optional)
    async fn health_check(&self) -> Result<(), AdapterError> {
        Ok(())
    }
}
```

## Creating a Custom Adapter

### Step 1: Define Your Adapter Struct

```rust
pub struct MyAdapter {
    name: String,
    config: MyAdapterConfig,
    client: Option<MyClient>,
    tools: Vec<Tool>,
}

#[derive(Debug, Clone)]
struct MyAdapterConfig {
    endpoint: String,
    api_key: String,
    // ... other config
}
```

### Step 2: Implement Constructor

```rust
impl MyAdapter {
    pub fn new(config: SourceConfig) -> Result<Self, AdapterError> {
        let endpoint = config.config.get("endpoint")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AdapterError::InitializationError(
                "Missing 'endpoint' in config".to_string()
            ))?
            .to_string();
        
        let api_key = config.config.get("api_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AdapterError::InitializationError(
                "Missing 'api_key' in config".to_string()
            ))?
            .to_string();
        
        Ok(Self {
            name: config.name,
            config: MyAdapterConfig { endpoint, api_key },
            client: None,
            tools: Vec::new(),
        })
    }
}
```

### Step 3: Implement Adapter Trait

```rust
#[async_trait]
impl Adapter for MyAdapter {
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn initialize(&mut self) -> Result<(), AdapterError> {
        tracing::info!("Initializing {} adapter", self.name);
        
        // Create client, authenticate, etc.
        let client = MyClient::new(&self.config.endpoint, &self.config.api_key)
            .map_err(|e| AdapterError::InitializationError(e.to_string()))?;
        
        self.client = Some(client);
        
        // Discover tools
        self.tools = self.discover_tools_internal().await?;
        
        Ok(())
    }
    
    async fn discover_tools(&self) -> Result<Vec<Tool>, AdapterError> {
        Ok(self.tools.clone())
    }
    
    async fn execute(
        &self,
        tool: &str,
        params: Value,
        ctx: ExecutionContext,
    ) -> Result<Value, AdapterError> {
        let client = self.client.as_ref()
            .ok_or_else(|| AdapterError::ExecutionError(
                "Adapter not initialized".to_string()
            ))?;
        
        // Execute the tool
        match tool {
            "my-tool" => self.execute_my_tool(client, params, ctx).await,
            _ => Err(AdapterError::ToolNotFound(tool.to_string())),
        }
    }
}
```

### Step 4: Register Your Adapter

Add to `adapters/factory.rs`:

```rust
use super::my_adapter::MyAdapter;

impl AdapterFactory {
    pub async fn create_adapter(
        &self,
        adapter_type: &str,
        config: SourceConfig,
    ) -> Result<Arc<dyn Adapter>, AdapterError> {
        match adapter_type.to_lowercase().as_str() {
            // ... existing adapters
            "my-adapter" => {
                let mut adapter = MyAdapter::new(config)?;
                adapter.initialize().await?;
                Ok(Arc::new(adapter))
            }
            _ => Err(AdapterError::InitializationError(
                format!("Unknown adapter type: {}", adapter_type)
            )),
        }
    }
}
```

## Tool Definition

Tools are defined using the `Tool` struct:

```rust
Tool {
    name: "get-user".to_string(),
    description: "Retrieve user information by ID".to_string(),
    parameters: ToolParameters {
        param_type: "object".to_string(),
        properties: {
            let mut props = HashMap::new();
            props.insert("user_id".to_string(), ParameterProperty {
                prop_type: "string".to_string(),
                description: Some("User ID".to_string()),
                default: None,
            });
            props
        },
        required: vec!["user_id".to_string()],
    },
    returns: Some("object".to_string()),
}
```

## Best Practices

### 1. Error Handling

Use appropriate error types:

```rust
// Initialization errors
return Err(AdapterError::InitializationError("Failed to connect".to_string()));

// Tool not found
return Err(AdapterError::ToolNotFound(tool.to_string()));

// Execution errors
return Err(AdapterError::ExecutionError("API call failed".to_string()));

// Invalid parameters
return Err(AdapterError::InvalidParameters("Missing required field".to_string()));
```

### 2. Logging

Use structured logging:

```rust
tracing::info!("Initializing adapter: {}", self.name);
tracing::debug!("Executing tool: {} with params: {:?}", tool, params);
tracing::warn!("Rate limit approaching for adapter: {}", self.name);
tracing::error!("Failed to execute tool: {}", error);
```

### 3. Configuration

Support environment variable substitution:

```yaml
sources:
  - name: "my-adapter"
    type: "my-adapter"
    endpoint: "${MY_API_ENDPOINT}"
    api_key: "${MY_API_KEY}"
```

### 4. Timeouts

Respect the execution context timeout:

```rust
async fn execute(&self, tool: &str, params: Value, ctx: ExecutionContext) -> Result<Value, AdapterError> {
    let timeout = Duration::from_secs(ctx.timeout);
    
    tokio::time::timeout(timeout, self.do_work())
        .await
        .map_err(|_| AdapterError::ExecutionError("Timeout".to_string()))?
}
```

### 5. Connection Pooling

Reuse connections:

```rust
pub struct MyAdapter {
    client: Arc<MyClient>, // Shared client with connection pool
    // ...
}
```

### 6. Validation

Validate parameters before execution:

```rust
fn validate_params(&self, params: &Value) -> Result<(), AdapterError> {
    let user_id = params.get("user_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| AdapterError::InvalidParameters(
            "Missing or invalid 'user_id'".to_string()
        ))?;
    
    if user_id.is_empty() {
        return Err(AdapterError::InvalidParameters(
            "'user_id' cannot be empty".to_string()
        ));
    }
    
    Ok(())
}
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_adapter_initialization() {
        let config = SourceConfig {
            name: "test".to_string(),
            source_type: "my-adapter".to_string(),
            config: serde_json::json!({
                "endpoint": "https://api.example.com",
                "api_key": "test-key"
            }).as_object().unwrap().clone(),
        };
        
        let mut adapter = MyAdapter::new(config).unwrap();
        assert!(adapter.initialize().await.is_ok());
    }
}
```

### Integration Tests

```rust
#[tokio::test]
async fn test_tool_execution() {
    let adapter = setup_test_adapter().await;
    
    let params = serde_json::json!({
        "user_id": "123"
    });
    
    let ctx = ExecutionContext {
        user_id: None,
        session_id: "test".to_string(),
        timeout: 30,
        metadata: HashMap::new(),
    };
    
    let result = adapter.execute("get-user", params, ctx).await;
    assert!(result.is_ok());
}
```

## Examples

See the `examples/` directory for complete adapter implementations:

- `examples/adapters/weather.rs` - Simple REST API adapter
- `examples/adapters/redis.rs` - Database adapter
- `examples/adapters/custom_cli.rs` - CLI wrapper adapter

## Publishing Your Adapter

1. Create a separate crate in `adapters/my-adapter/`
2. Add to workspace in root `Cargo.toml`
3. Document configuration options
4. Add examples
5. Submit a pull request

## Resources

- [Adapter API Reference](https://docs.universal-mcp-gateway.dev/api/adapter)
- [Example Adapters](../examples/adapters/)
- [Community Adapters](https://github.com/universal-mcp-gateway/adapters)
