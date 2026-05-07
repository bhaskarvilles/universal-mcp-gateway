use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;
use std::process::Stdio;
use tokio::process::Command;

use crate::config::SourceConfig;
use super::{Adapter, Tool, ToolParameters, ParameterProperty, ExecutionContext, AdapterError};

pub struct CLIAdapter {
    name: String,
    command: String,
    allowed_args: Vec<String>,
    enable_sandbox: bool,
    tools: Vec<Tool>,
}

impl CLIAdapter {
    pub fn new(config: SourceConfig) -> Result<Self, AdapterError> {
        let command = config.config.get("command")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AdapterError::InitializationError(
                "Missing 'command' in CLI adapter config".to_string()
            ))?
            .to_string();
        
        let allowed_args = config.config.get("allowed_args")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default();
        
        let enable_sandbox = config.config.get("sandbox")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        
        Ok(Self {
            name: config.name,
            command,
            allowed_args,
            enable_sandbox,
            tools: Vec::new(),
        })
    }
    
    fn generate_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "execute".to_string(),
                description: format!("Execute {} command", self.command),
                parameters: ToolParameters {
                    param_type: "object".to_string(),
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("args".to_string(), ParameterProperty {
                            prop_type: "array".to_string(),
                            description: Some("Command arguments".to_string()),
                            default: None,
                        });
                        props.insert("stdin".to_string(), ParameterProperty {
                            prop_type: "string".to_string(),
                            description: Some("Standard input".to_string()),
                            default: None,
                        });
                        props
                    },
                    required: Vec::new(),
                },
                returns: Some("object".to_string()),
            }
        ]
    }
    
    fn validate_args(&self, args: &[String]) -> Result<(), AdapterError> {
        if self.allowed_args.is_empty() {
            return Ok(());
        }
        
        for arg in args {
            if !self.allowed_args.iter().any(|allowed| arg.starts_with(allowed)) {
                return Err(AdapterError::InvalidParameters(
                    format!("Argument '{}' is not allowed", arg)
                ));
            }
        }
        
        Ok(())
    }
}

#[async_trait]
impl Adapter for CLIAdapter {
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn initialize(&mut self) -> Result<(), AdapterError> {
        tracing::info!("Initializing CLI adapter: {} ({})", self.name, self.command);
        
        self.tools = self.generate_tools();
        
        Ok(())
    }
    
    async fn discover_tools(&self) -> Result<Vec<Tool>, AdapterError> {
        Ok(self.tools.clone())
    }
    
    async fn execute(
        &self,
        _tool: &str,
        params: Value,
        ctx: ExecutionContext,
    ) -> Result<Value, AdapterError> {
        let args = params.get("args")
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        
        self.validate_args(&args)?;
        
        let stdin_data = params.get("stdin")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());
        
        tracing::debug!("Executing command: {} {:?}", self.command, args);
        
        let mut cmd = Command::new(&self.command);
        cmd.args(&args);
        cmd.stdout(Stdio::piped());
        cmd.stderr(Stdio::piped());
        
        if stdin_data.is_some() {
            cmd.stdin(Stdio::piped());
        }
        
        // Set timeout from context
        let timeout = std::time::Duration::from_secs(ctx.timeout);
        
        let output = tokio::time::timeout(timeout, cmd.output())
            .await
            .map_err(|_| AdapterError::ExecutionError("Command timed out".to_string()))?
            .map_err(|e| AdapterError::ExecutionError(format!("Failed to execute command: {}", e)))?;
        
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        Ok(serde_json::json!({
            "exit_code": output.status.code(),
            "stdout": stdout,
            "stderr": stderr,
            "success": output.status.success(),
        }))
    }
}
