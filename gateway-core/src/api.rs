use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

use crate::config::Config;
use crate::adapters::AdapterRegistry;
use crate::protocol::{MCPProtocolHandler, MCPRequest, MCPResponse};

#[derive(Clone)]
pub struct AppState {
    config: Config,
    adapter_registry: Arc<RwLock<AdapterRegistry>>,
    protocol_handler: Arc<MCPProtocolHandler>,
}

pub struct ApiServer {
    state: AppState,
}

impl ApiServer {
    pub fn new(
        config: Config,
        adapter_registry: Arc<RwLock<AdapterRegistry>>,
        protocol_handler: Arc<MCPProtocolHandler>,
    ) -> Self {
        let state = AppState {
            config,
            adapter_registry,
            protocol_handler,
        };
        
        Self { state }
    }
    
    pub async fn serve(self) -> anyhow::Result<()> {
        let app = Router::new()
            .route("/", get(root_handler))
            .route("/health", get(health_handler))
            .route("/mcp", post(mcp_handler))
            .route("/adapters", get(list_adapters_handler))
            .route("/tools", get(list_tools_handler))
            .layer(CorsLayer::permissive())
            .layer(TraceLayer::new_for_http())
            .with_state(self.state.clone());
        
        let addr = format!("{}:{}", self.state.config.gateway.host, self.state.config.gateway.port);
        let listener = tokio::net::TcpListener::bind(&addr).await?;
        
        tracing::info!("🚀 Gateway listening on http://{}", addr);
        tracing::info!("📊 Health check: http://{}/health", addr);
        tracing::info!("🔧 MCP endpoint: http://{}/mcp", addr);
        
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}

async fn root_handler() -> impl IntoResponse {
    Json(serde_json::json!({
        "name": "Universal MCP Gateway",
        "version": env!("CARGO_PKG_VERSION"),
        "endpoints": {
            "health": "/health",
            "mcp": "/mcp",
            "adapters": "/adapters",
            "tools": "/tools"
        }
    }))
}

async fn health_handler(State(state): State<AppState>) -> impl IntoResponse {
    let registry = state.adapter_registry.read().await;
    let adapters = registry.list_adapters();
    
    Json(serde_json::json!({
        "status": "healthy",
        "adapters": adapters.len(),
        "adapter_names": adapters
    }))
}

async fn mcp_handler(
    State(state): State<AppState>,
    Json(request): Json<MCPRequest>,
) -> impl IntoResponse {
    let response = state.protocol_handler.handle_request(request).await;
    Json(response)
}

async fn list_adapters_handler(State(state): State<AppState>) -> impl IntoResponse {
    let registry = state.adapter_registry.read().await;
    let adapters = registry.list_adapters();
    
    Json(serde_json::json!({
        "adapters": adapters
    }))
}

async fn list_tools_handler(State(state): State<AppState>) -> impl IntoResponse {
    let registry = state.adapter_registry.read().await;
    
    match registry.list_all_tools().await {
        Ok(adapter_tools) => {
            let mut all_tools = Vec::new();
            
            for (adapter_name, tools) in adapter_tools {
                for tool in tools {
                    all_tools.push(serde_json::json!({
                        "adapter": adapter_name,
                        "name": tool.name,
                        "full_name": format!("{}.{}", adapter_name, tool.name),
                        "description": tool.description,
                        "parameters": tool.parameters,
                    }));
                }
            }
            
            (StatusCode::OK, Json(serde_json::json!({
                "tools": all_tools,
                "count": all_tools.len()
            })))
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({
                "error": format!("Failed to list tools: {}", e)
            })))
        }
    }
}
