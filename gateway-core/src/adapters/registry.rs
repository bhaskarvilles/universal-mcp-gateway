use std::collections::HashMap;
use std::sync::Arc;

use super::{Adapter, Tool, AdapterError};

pub struct AdapterRegistry {
    adapters: HashMap<String, Arc<dyn Adapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        Self {
            adapters: HashMap::new(),
        }
    }
    
    pub fn register(&mut self, name: String, adapter: Arc<dyn Adapter>) {
        self.adapters.insert(name, adapter);
    }
    
    pub fn get(&self, name: &str) -> Option<Arc<dyn Adapter>> {
        self.adapters.get(name).cloned()
    }
    
    pub async fn list_all_tools(&self) -> Result<Vec<(String, Vec<Tool>)>, AdapterError> {
        let mut all_tools = Vec::new();
        
        for (name, adapter) in &self.adapters {
            match adapter.discover_tools().await {
                Ok(tools) => {
                    all_tools.push((name.clone(), tools));
                }
                Err(e) => {
                    tracing::warn!("Failed to discover tools for adapter '{}': {}", name, e);
                }
            }
        }
        
        Ok(all_tools)
    }
    
    pub fn list_adapters(&self) -> Vec<String> {
        self.adapters.keys().cloned().collect()
    }
}

impl Default for AdapterRegistry {
    fn default() -> Self {
        Self::new()
    }
}
