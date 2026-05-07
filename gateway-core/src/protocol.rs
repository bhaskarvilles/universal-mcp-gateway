use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::adapters::{AdapterRegistry, ExecutionContext};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPRequest {
    pub jsonrpc: String,
    pub id: String,
    pub method: String,
    pub params: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPResponse {
    pub jsonrpc: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<MCPError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MCPError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

pub struct MCPProtocolHandler {
    adapter_registry: Arc<RwLock<AdapterRegistry>>,
}

impl MCPProtocolHandler {
    pub fn new(adapter_registry: Arc<RwLock<AdapterRegistry>>) -> Self {
        Self { adapter_registry }
    }
    
    pub async fn handle_request(&self, request: MCPRequest) -> MCPResponse {
        match request.method.as_str() {
            "tools/list" => self.handle_list_tools(request).await,
            "tools/call" => self.handle_call_tool(request).await,
            "initialize" => self.handle_initialize(request).await,
            _ => MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
            },
        }
    }
    
    async fn handle_initialize(&self, request: MCPRequest) -> MCPResponse {
        MCPResponse {
            jsonrpc: "2.0".to_string(),
            id: request.id,
            result: Some(serde_json::json!({
                "protocolVersion": "1.0",
                "serverInfo": {
                    "name": "universal-mcp-gateway",
                    "version": env!("CARGO_PKG_VERSION"),
                },
                "capabilities": {
                    "tools": true,
                }
            })),
            error: None,
        }
    }
    
    async fn handle_list_tools(&self, request: MCPRequest) -> MCPResponse {
        let registry = self.adapter_registry.read().await;
        
        match registry.list_all_tools().await {
            Ok(adapter_tools) => {
                let mut all_tools = Vec::new();
                
                for (adapter_name, tools) in adapter_tools {
                    for tool in tools {
                        all_tools.push(serde_json::json!({
                            "name": format!("{}.{}", adapter_name, tool.name),
                            "description": tool.description,
                            "inputSchema": {
                                "type": tool.parameters.param_type,
                                "properties": tool.parameters.properties,
                                "required": tool.parameters.required,
                            }
                        }));
                    }
                }
                
                MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(serde_json::json!({
                        "tools": all_tools
                    })),
                    error: None,
                }
            }
            Err(e) => MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError {
                    code: -32603,
                    message: format!("Failed to list tools: {}", e),
                    data: None,
                }),
            },
        }
    }
    
    async fn handle_call_tool(&self, request: MCPRequest) -> MCPResponse {
        let params = match request.params {
            Some(p) => p,
            None => {
                return MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(MCPError {
                        code: -32602,
                        message: "Missing parameters".to_string(),
                        data: None,
                    }),
                };
            }
        };
        
        let tool_name = match params.get("name").and_then(|v| v.as_str()) {
            Some(name) => name,
            None => {
                return MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(MCPError {
                        code: -32602,
                        message: "Missing tool name".to_string(),
                        data: None,
                    }),
                };
            }
        };
        
        let arguments = params.get("arguments").cloned().unwrap_or(Value::Object(serde_json::Map::new()));
        
        // Parse adapter.tool format
        let parts: Vec<&str> = tool_name.split('.').collect();
        if parts.len() != 2 {
            return MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError {
                    code: -32602,
                    message: "Invalid tool name format. Expected: adapter.tool".to_string(),
                    data: None,
                }),
            };
        }
        
        let adapter_name = parts[0];
        let tool = parts[1];
        
        let registry = self.adapter_registry.read().await;
        let adapter = match registry.get(adapter_name) {
            Some(a) => a,
            None => {
                return MCPResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: None,
                    error: Some(MCPError {
                        code: -32602,
                        message: format!("Adapter not found: {}", adapter_name),
                        data: None,
                    }),
                };
            }
        };
        
        let ctx = ExecutionContext {
            user_id: None,
            session_id: Uuid::new_v4().to_string(),
            timeout: 30,
            metadata: std::collections::HashMap::new(),
        };
        
        match adapter.execute(tool, arguments, ctx).await {
            Ok(result) => MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: Some(serde_json::json!({
                    "content": [
                        {
                            "type": "text",
                            "text": serde_json::to_string_pretty(&result).unwrap_or_default()
                        }
                    ]
                })),
                error: None,
            },
            Err(e) => MCPResponse {
                jsonrpc: "2.0".to_string(),
                id: request.id,
                result: None,
                error: Some(MCPError {
                    code: -32603,
                    message: format!("Tool execution failed: {}", e),
                    data: None,
                }),
            },
        }
    }
}
