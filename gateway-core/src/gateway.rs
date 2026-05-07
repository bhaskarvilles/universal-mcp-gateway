use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error};

use crate::config::Config;
use crate::adapters::{AdapterRegistry, AdapterFactory};
use crate::protocol::MCPProtocolHandler;
use crate::api::ApiServer;

pub struct Gateway {
    config: Config,
    adapter_registry: Arc<RwLock<AdapterRegistry>>,
    protocol_handler: Arc<MCPProtocolHandler>,
}

impl Gateway {
    pub async fn new(config: Config) -> Result<Self> {
        info!("Initializing gateway...");
        
        // Create adapter registry
        let adapter_registry = Arc::new(RwLock::new(AdapterRegistry::new()));
        
        // Initialize adapters from config
        let factory = AdapterFactory::new();
        for source_config in &config.sources {
            info!("Loading adapter: {} ({})", source_config.name, source_config.source_type);
            
            match factory.create_adapter(&source_config.source_type, source_config.clone()).await {
                Ok(adapter) => {
                    adapter_registry.write().await.register(source_config.name.clone(), adapter);
                    info!("✓ Adapter '{}' loaded successfully", source_config.name);
                }
                Err(e) => {
                    error!("✗ Failed to load adapter '{}': {}", source_config.name, e);
                }
            }
        }
        
        // Create protocol handler
        let protocol_handler = Arc::new(MCPProtocolHandler::new(adapter_registry.clone()));
        
        Ok(Self {
            config,
            adapter_registry,
            protocol_handler,
        })
    }
    
    pub async fn run(self) -> Result<()> {
        info!("Starting API server...");
        
        let api_server = ApiServer::new(
            self.config.clone(),
            self.adapter_registry.clone(),
            self.protocol_handler.clone(),
        );
        
        api_server.serve().await?;
        
        Ok(())
    }
}
