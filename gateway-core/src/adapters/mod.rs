pub mod cli;
pub mod factory;
pub mod openapi;
pub mod postgresql;
pub mod registry;
pub mod sqlite;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use thiserror::Error;

pub use factory::AdapterFactory;
pub use registry::AdapterRegistry;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum AdapterError {
    #[error("Initialization failed: {0}")]
    InitializationError(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Execution failed: {0}")]
    ExecutionError(String),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Authentication failed: {0}")]
    AuthenticationError(String),

    #[error("Rate limit exceeded")]
    RateLimitExceeded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: ToolParameters,
    pub returns: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolParameters {
    #[serde(rename = "type")]
    pub param_type: String,
    pub properties: HashMap<String, ParameterProperty>,
    pub required: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParameterProperty {
    #[serde(rename = "type")]
    pub prop_type: String,
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<Value>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ExecutionContext {
    pub user_id: Option<String>,
    pub session_id: String,
    pub timeout: u64,
    pub metadata: HashMap<String, String>,
}

#[async_trait]
#[allow(dead_code)]
pub trait Adapter: Send + Sync {
    /// Returns the adapter name
    fn name(&self) -> &str;

    /// Initialize the adapter
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

    /// Health check
    async fn health_check(&self) -> Result<(), AdapterError> {
        Ok(())
    }
}
