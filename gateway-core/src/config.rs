use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub gateway: GatewayConfig,
    pub security: SecurityConfig,
    pub sources: Vec<SourceConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    #[serde(default = "default_host")]
    pub host: String,
    
    #[serde(default = "default_port")]
    pub port: u16,
    
    #[serde(default = "default_log_level")]
    pub log_level: String,
    
    #[serde(default)]
    pub enable_ui: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    #[serde(default = "default_true")]
    pub enable_sandbox: bool,
    
    #[serde(default = "default_max_execution_time")]
    pub max_execution_time: u64,
    
    #[serde(default = "default_rate_limit")]
    pub rate_limit: String,
    
    #[serde(default)]
    pub allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceConfig {
    pub name: String,
    
    #[serde(rename = "type")]
    pub source_type: String,
    
    #[serde(flatten)]
    pub config: HashMap<String, serde_json::Value>,
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(path.as_ref())
            .context("Failed to read configuration file")?;
        
        let config: Config = serde_yaml::from_str(&content)
            .context("Failed to parse configuration file")?;
        
        config.validate()?;
        
        Ok(config)
    }
    
    fn validate(&self) -> Result<()> {
        // Validate source names are unique
        let mut names = std::collections::HashSet::new();
        for source in &self.sources {
            if !names.insert(&source.name) {
                anyhow::bail!("Duplicate source name: {}", source.name);
            }
        }
        
        Ok(())
    }
}

fn default_host() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_log_level() -> String {
    "info".to_string()
}

fn default_true() -> bool {
    true
}

fn default_max_execution_time() -> u64 {
    30
}

fn default_rate_limit() -> String {
    "100/minute".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_defaults() {
        let yaml = r#"
gateway: {}
security: {}
sources: []
"#;
        let config: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(config.gateway.host, "0.0.0.0");
        assert_eq!(config.gateway.port, 8080);
        assert!(config.security.enable_sandbox);
    }
}
