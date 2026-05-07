use anyhow::Result;
use std::sync::Arc;

use crate::config::SourceConfig;
use super::{Adapter, AdapterError};
use super::openapi::OpenAPIAdapter;
use super::postgresql::PostgreSQLAdapter;
use super::cli::CLIAdapter;

pub struct AdapterFactory;

impl AdapterFactory {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn create_adapter(
        &self,
        adapter_type: &str,
        config: SourceConfig,
    ) -> Result<Arc<dyn Adapter>, AdapterError> {
        match adapter_type.to_lowercase().as_str() {
            "openapi" | "swagger" => {
                let mut adapter = OpenAPIAdapter::new(config)?;
                adapter.initialize().await?;
                Ok(Arc::new(adapter))
            }
            "postgresql" | "postgres" => {
                let mut adapter = PostgreSQLAdapter::new(config)?;
                adapter.initialize().await?;
                Ok(Arc::new(adapter))
            }
            "cli" | "command" => {
                let mut adapter = CLIAdapter::new(config)?;
                adapter.initialize().await?;
                Ok(Arc::new(adapter))
            }
            _ => Err(AdapterError::InitializationError(
                format!("Unknown adapter type: {}", adapter_type)
            )),
        }
    }
}

impl Default for AdapterFactory {
    fn default() -> Self {
        Self::new()
    }
}
