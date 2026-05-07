.PHONY: help build test clean run docker-build docker-run install dev

help: ## Show this help message
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Available targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "  %-20s %s\n", $$1, $$2}' $(MAKEFILE_LIST)

install: ## Install dependencies
	@echo "Installing Rust dependencies..."
	cd gateway-core && cargo build
	@echo "Installing Node.js dependencies..."
	cd sdk && npm install

build: ## Build the project
	@echo "Building Rust backend..."
	cargo build --release
	@echo "Building TypeScript SDK..."
	cd sdk && npm run build

test: ## Run tests
	@echo "Running Rust tests..."
	cargo test
	@echo "Running TypeScript tests..."
	cd sdk && npm test

clean: ## Clean build artifacts
	cargo clean
	cd sdk && rm -rf dist node_modules

run: ## Run the gateway locally
	cargo run --bin mcp-gateway -- --config examples/config.yaml

dev: ## Run in development mode with auto-reload
	cargo watch -x 'run --bin mcp-gateway -- --config examples/config.yaml'

docker-build: ## Build Docker image
	docker build -t universalmcp/gateway:latest .

docker-run: ## Run Docker container
	docker-compose up

docker-stop: ## Stop Docker container
	docker-compose down

lint: ## Run linters
	cargo clippy -- -D warnings
	cd sdk && npm run lint

format: ## Format code
	cargo fmt
	cd sdk && npm run format

check: lint test ## Run all checks

release: ## Build release binaries
	cargo build --release --target x86_64-unknown-linux-gnu
	cargo build --release --target x86_64-apple-darwin
	cargo build --release --target aarch64-apple-darwin
