# Deployment Guide

## Overview

This guide covers deploying Universal MCP Gateway to various environments.

## Deployment Options

### 1. Docker (Recommended)

#### Single Container

```bash
docker run -d \
  --name mcp-gateway \
  -p 8080:8080 \
  -v $(pwd)/config.yaml:/app/config.yaml:ro \
  -e DATABASE_URL="postgresql://..." \
  -e API_KEY="..." \
  --restart unless-stopped \
  universalmcp/gateway:latest
```

#### Docker Compose

```yaml
version: '3.8'

services:
  gateway:
    image: universalmcp/gateway:latest
    ports:
      - "8080:8080"
    volumes:
      - ./config.yaml:/app/config.yaml:ro
    environment:
      - DATABASE_URL=${DATABASE_URL}
      - API_KEY=${API_KEY}
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 10s
      retries: 3
```

### 2. Kubernetes

#### Deployment

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: mcp-gateway
spec:
  replicas: 3
  selector:
    matchLabels:
      app: mcp-gateway
  template:
    metadata:
      labels:
        app: mcp-gateway
    spec:
      containers:
      - name: gateway
        image: universalmcp/gateway:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: gateway-secrets
              key: database-url
        volumeMounts:
        - name: config
          mountPath: /app/config.yaml
          subPath: config.yaml
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: config
        configMap:
          name: gateway-config
```

#### Service

```yaml
apiVersion: v1
kind: Service
metadata:
  name: mcp-gateway
spec:
  selector:
    app: mcp-gateway
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: LoadBalancer
```

#### ConfigMap

```yaml
apiVersion: v1
kind: ConfigMap
metadata:
  name: gateway-config
data:
  config.yaml: |
    gateway:
      host: "0.0.0.0"
      port: 8080
    security:
      enable_sandbox: true
    sources:
      - name: "my-api"
        type: "openapi"
        spec: "${API_SPEC_URL}"
```

### 3. AWS

#### ECS Fargate

```json
{
  "family": "mcp-gateway",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "512",
  "memory": "1024",
  "containerDefinitions": [
    {
      "name": "gateway",
      "image": "universalmcp/gateway:latest",
      "portMappings": [
        {
          "containerPort": 8080,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "DATABASE_URL",
          "value": "postgresql://..."
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/mcp-gateway",
          "awslogs-region": "us-east-1",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ]
}
```

#### EC2

```bash
# Install Docker
sudo yum update -y
sudo yum install -y docker
sudo service docker start

# Run gateway
sudo docker run -d \
  --name mcp-gateway \
  -p 80:8080 \
  -v /etc/mcp-gateway/config.yaml:/app/config.yaml:ro \
  --restart always \
  universalmcp/gateway:latest
```

### 4. Google Cloud Platform

#### Cloud Run

```bash
# Build and push image
gcloud builds submit --tag gcr.io/PROJECT_ID/mcp-gateway

# Deploy
gcloud run deploy mcp-gateway \
  --image gcr.io/PROJECT_ID/mcp-gateway \
  --platform managed \
  --region us-central1 \
  --allow-unauthenticated \
  --set-env-vars DATABASE_URL="postgresql://..."
```

#### GKE

Use the Kubernetes deployment above with GKE-specific configurations.

### 5. Azure

#### Container Instances

```bash
az container create \
  --resource-group myResourceGroup \
  --name mcp-gateway \
  --image universalmcp/gateway:latest \
  --dns-name-label mcp-gateway \
  --ports 8080 \
  --environment-variables \
    DATABASE_URL="postgresql://..." \
    API_KEY="..."
```

#### AKS

Use the Kubernetes deployment above with AKS-specific configurations.

## Reverse Proxy Setup

### Nginx

```nginx
upstream mcp_gateway {
    server localhost:8080;
}

server {
    listen 80;
    server_name gateway.example.com;
    return 301 https://$server_name$request_uri;
}

server {
    listen 443 ssl http2;
    server_name gateway.example.com;

    ssl_certificate /etc/ssl/certs/gateway.crt;
    ssl_certificate_key /etc/ssl/private/gateway.key;

    location / {
        proxy_pass http://mcp_gateway;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Caddy

```
gateway.example.com {
    reverse_proxy localhost:8080
}
```

## Environment Variables

### Required

- `DATABASE_URL` - Database connection string (if using database adapters)

### Optional

- `RUST_LOG` - Log level (trace, debug, info, warn, error)
- `API_KEY` - API authentication key
- `GITHUB_TOKEN` - GitHub API token
- Custom adapter-specific variables

## Monitoring

### Prometheus Metrics

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'mcp-gateway'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
```

### Grafana Dashboard

Import the provided dashboard from `monitoring/grafana-dashboard.json`

### Health Checks

```bash
# Basic health check
curl http://localhost:8080/health

# Detailed status
curl http://localhost:8080/health?detailed=true
```

## Scaling

### Horizontal Scaling

Gateway is stateless and can be scaled horizontally:

```bash
# Docker Compose
docker-compose up --scale gateway=3

# Kubernetes
kubectl scale deployment mcp-gateway --replicas=5
```

### Load Balancing

Use a load balancer to distribute traffic:

- AWS: Application Load Balancer (ALB)
- GCP: Cloud Load Balancing
- Azure: Azure Load Balancer
- Self-hosted: Nginx, HAProxy

## Backup and Recovery

### Configuration Backup

```bash
# Backup config
cp config.yaml config.yaml.backup

# Version control
git add config.yaml
git commit -m "Update gateway config"
```

### Database Backup

```bash
# PostgreSQL
pg_dump -h localhost -U user dbname > backup.sql

# Restore
psql -h localhost -U user dbname < backup.sql
```

## Troubleshooting

### Gateway Won't Start

```bash
# Check logs
docker logs mcp-gateway

# Verify config
docker run --rm -v $(pwd)/config.yaml:/app/config.yaml \
  universalmcp/gateway:latest --config /app/config.yaml --validate
```

### High Memory Usage

```bash
# Set memory limits
docker run -m 512m universalmcp/gateway:latest

# Kubernetes
resources:
  limits:
    memory: "512Mi"
  requests:
    memory: "256Mi"
```

### Connection Issues

```bash
# Test connectivity
curl -v http://localhost:8080/health

# Check firewall
sudo iptables -L

# Verify DNS
nslookup gateway.example.com
```

## Security Hardening

### 1. Run as Non-Root

```dockerfile
USER gateway
```

### 2. Read-Only Filesystem

```bash
docker run --read-only universalmcp/gateway:latest
```

### 3. Drop Capabilities

```bash
docker run --cap-drop=ALL universalmcp/gateway:latest
```

### 4. Network Policies

```yaml
# Kubernetes NetworkPolicy
apiVersion: networking.k8s.io/v1
kind: NetworkPolicy
metadata:
  name: mcp-gateway-policy
spec:
  podSelector:
    matchLabels:
      app: mcp-gateway
  policyTypes:
  - Ingress
  - Egress
  ingress:
  - from:
    - podSelector:
        matchLabels:
          app: frontend
    ports:
    - protocol: TCP
      port: 8080
```

## Performance Tuning

### 1. Connection Pooling

```yaml
sources:
  - name: "database"
    type: "postgresql"
    connection: "${DATABASE_URL}"
    pool_size: 20
    max_connections: 50
```

### 2. Caching

```yaml
gateway:
  cache:
    enabled: true
    ttl: 300
    max_size: 1000
```

### 3. Resource Limits

```yaml
security:
  max_execution_time: 30
  max_memory: "256MB"
  max_concurrent_requests: 100
```

## Cost Optimization

### AWS

- Use Fargate Spot for non-critical workloads
- Enable auto-scaling based on CPU/memory
- Use Reserved Instances for predictable workloads

### GCP

- Use Cloud Run for variable traffic
- Enable autoscaling
- Use committed use discounts

### Azure

- Use Azure Container Instances for burst workloads
- Enable autoscaling
- Use reserved capacity

## Maintenance

### Updates

```bash
# Pull latest image
docker pull universalmcp/gateway:latest

# Restart with new image
docker-compose up -d

# Kubernetes rolling update
kubectl set image deployment/mcp-gateway \
  gateway=universalmcp/gateway:latest
```

### Logs Rotation

```yaml
# docker-compose.yml
services:
  gateway:
    logging:
      driver: "json-file"
      options:
        max-size: "10m"
        max-file: "3"
```

## Support

- [Documentation](https://docs.universal-mcp-gateway.dev)
- [GitHub Issues](https://github.com/yourusername/universal-mcp-gateway/issues)
- [Discord](https://discord.gg/universal-mcp)
