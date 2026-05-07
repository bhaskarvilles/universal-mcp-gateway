use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

mod config;
mod gateway;
mod adapters;
mod protocol;
mod sandbox;
mod api;
mod auth;

use crate::config::Config;
use crate::gateway::Gateway;

#[derive(Parser, Debug)]
#[command(name = "mcp-gateway")]
#[command(about = "Universal MCP Gateway - Connect anything to AI agents", long_about = None)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, default_value = "config.yaml")]
    config: PathBuf,

    /// Log level (trace, debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Host to bind to
    #[arg(long)]
    host: Option<String>,

    /// Port to bind to
    #[arg(long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    // Initialize logging
    let log_level = match args.log_level.to_lowercase().as_str() {
        "trace" => Level::TRACE,
        "debug" => Level::DEBUG,
        "info" => Level::INFO,
        "warn" => Level::WARN,
        "error" => Level::ERROR,
        _ => Level::INFO,
    };

    let subscriber = FmtSubscriber::builder()
        .with_max_level(log_level)
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    info!("🌉 Universal MCP Gateway starting...");

    // Load configuration
    let mut config = Config::from_file(&args.config)?;
    
    // Override with CLI args if provided
    if let Some(host) = args.host {
        config.gateway.host = host;
    }
    if let Some(port) = args.port {
        config.gateway.port = port;
    }

    info!("Configuration loaded from {:?}", args.config);
    info!("Gateway will listen on {}:{}", config.gateway.host, config.gateway.port);

    // Initialize and start gateway
    let gateway = Gateway::new(config).await?;
    gateway.run().await?;

    Ok(())
}
