// Example: Custom Weather API Adapter

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use mcp_gateway::adapters::{
    Adapter, Tool, ToolParameters, ParameterProperty, ExecutionContext, AdapterError
};
use mcp_gateway::config::SourceConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct WeatherResponse {
    temperature: f64,
    conditions: String,
    humidity: f64,
    wind_speed: f64,
}

pub struct WeatherAdapter {
    name: String,
    api_key: String,
    base_url: String,
    tools: Vec<Tool>,
}

impl WeatherAdapter {
    pub fn new(config: SourceConfig) -> Result<Self, AdapterError> {
        let api_key = config.config.get("api_key")
            .and_then(|v| v.as_str())
            .ok_or_else(|| AdapterError::InitializationError(
                "Missing 'api_key' in config".to_string()
            ))?
            .to_string();
        
        let base_url = config.config.get("base_url")
            .and_then(|v| v.as_str())
            .unwrap_or("https://api.weather.com")
            .to_string();
        
        Ok(Self {
            name: config.name,
            api_key,
            base_url,
            tools: Vec::new(),
        })
    }
    
    fn create_tools(&self) -> Vec<Tool> {
        vec![
            Tool {
                name: "get-current-weather".to_string(),
                description: "Get current weather for a location".to_string(),
                parameters: ToolParameters {
                    param_type: "object".to_string(),
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("location".to_string(), ParameterProperty {
                            prop_type: "string".to_string(),
                            description: Some("City name or coordinates".to_string()),
                            default: None,
                        });
                        props.insert("units".to_string(), ParameterProperty {
                            prop_type: "string".to_string(),
                            description: Some("Temperature units (celsius/fahrenheit)".to_string()),
                            default: Some(Value::String("celsius".to_string())),
                        });
                        props
                    },
                    required: vec!["location".to_string()],
                },
                returns: Some("object".to_string()),
            },
            Tool {
                name: "get-forecast".to_string(),
                description: "Get weather forecast for a location".to_string(),
                parameters: ToolParameters {
                    param_type: "object".to_string(),
                    properties: {
                        let mut props = HashMap::new();
                        props.insert("location".to_string(), ParameterProperty {
                            prop_type: "string".to_string(),
                            description: Some("City name or coordinates".to_string()),
                            default: None,
                        });
                        props.insert("days".to_string(), ParameterProperty {
                            prop_type: "integer".to_string(),
                            description: Some("Number of days (1-7)".to_string()),
                            default: Some(Value::Number(3.into())),
                        });
                        props
                    },
                    required: vec!["location".to_string()],
                },
                returns: Some("array".to_string()),
            },
        ]
    }
    
    async fn fetch_current_weather(
        &self,
        location: &str,
        units: &str,
    ) -> Result<WeatherResponse, AdapterError> {
        // In a real implementation, this would make an HTTP request
        // to the weather API
        
        let client = reqwest::Client::new();
        let url = format!("{}/current", self.base_url);
        
        let response = client
            .get(&url)
            .query(&[
                ("location", location),
                ("units", units),
                ("apikey", &self.api_key),
            ])
            .send()
            .await
            .map_err(|e| AdapterError::ExecutionError(e.to_string()))?;
        
        if !response.status().is_success() {
            return Err(AdapterError::ExecutionError(
                format!("API returned status: {}", response.status())
            ));
        }
        
        let weather: WeatherResponse = response
            .json()
            .await
            .map_err(|e| AdapterError::ExecutionError(e.to_string()))?;
        
        Ok(weather)
    }
}

#[async_trait]
impl Adapter for WeatherAdapter {
    fn name(&self) -> &str {
        &self.name
    }
    
    async fn initialize(&mut self) -> Result<(), AdapterError> {
        tracing::info!("Initializing Weather adapter: {}", self.name);
        self.tools = self.create_tools();
        Ok(())
    }
    
    async fn discover_tools(&self) -> Result<Vec<Tool>, AdapterError> {
        Ok(self.tools.clone())
    }
    
    async fn execute(
        &self,
        tool: &str,
        params: Value,
        _ctx: ExecutionContext,
    ) -> Result<Value, AdapterError> {
        match tool {
            "get-current-weather" => {
                let location = params.get("location")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AdapterError::InvalidParameters(
                        "Missing 'location' parameter".to_string()
                    ))?;
                
                let units = params.get("units")
                    .and_then(|v| v.as_str())
                    .unwrap_or("celsius");
                
                let weather = self.fetch_current_weather(location, units).await?;
                
                Ok(serde_json::to_value(weather)
                    .map_err(|e| AdapterError::ExecutionError(e.to_string()))?)
            }
            "get-forecast" => {
                // Similar implementation for forecast
                Ok(Value::Array(Vec::new()))
            }
            _ => Err(AdapterError::ToolNotFound(tool.to_string())),
        }
    }
    
    async fn health_check(&self) -> Result<(), AdapterError> {
        // Verify API is accessible
        let client = reqwest::Client::new();
        let url = format!("{}/health", self.base_url);
        
        client
            .get(&url)
            .send()
            .await
            .map_err(|e| AdapterError::ExecutionError(e.to_string()))?;
        
        Ok(())
    }
}

// Usage in config.yaml:
// sources:
//   - name: "weather"
//     type: "weather"
//     api_key: "${WEATHER_API_KEY}"
//     base_url: "https://api.weather.com"
