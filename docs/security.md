# Security Best Practices

## Overview

Security is a critical concern when exposing APIs, databases, and CLI tools to AI agents. This guide covers security features and best practices for Universal MCP Gateway.

## Security Features

### 1. Sandboxed Execution

All tool executions run in isolated WASM environments:

```yaml
security:
  enable_sandbox: true
  max_execution_time: 30
```

**Benefits:**
- Prevents resource exhaustion
- Isolates failures
- Limits system access
- Enforces timeouts

### 2. Authentication

Multiple authentication methods supported:

#### API Keys

```yaml
sources:
  - name: "my-api"
    type: "openapi"
    auth:
      type: "api_key"
      header: "X-API-Key"
      value: "${API_KEY}"
```

#### Bearer Tokens

```yaml
sources:
  - name: "github"
    type: "openapi"
    auth:
      type: "bearer"
      token: "${GITHUB_TOKEN}"
```

#### OAuth2

```yaml
sources:
  - name: "google-api"
    type: "openapi"
    auth:
      type: "oauth2"
      client_id: "${OAUTH_CLIENT_ID}"
      client_secret: "${OAUTH_CLIENT_SECRET}"
      token_url: "https://oauth2.googleapis.com/token"
```

### 3. Rate Limiting

Prevent abuse with rate limiting:

```yaml
security:
  rate_limit: "100/minute"  # Per adapter
```

### 4. Input Validation

All tool parameters are validated against schemas before execution.

### 5. Audit Logging

Complete audit trail of all operations:

```json
{
  "timestamp": "2026-05-07T10:30:00Z",
  "user_id": "user123",
  "adapter": "postgres-db",
  "tool": "query_users",
  "parameters": {"limit": 10},
  "result": "success",
  "duration_ms": 45
}
```

## Threat Model

### Protected Against

1. **SQL Injection**
   - Parameterized queries
   - Input sanitization
   - Read-only mode option

2. **Command Injection**
   - Allowed argument whitelisting
   - Shell escaping
   - Sandboxed execution

3. **Resource Exhaustion**
   - Execution timeouts
   - Memory limits
   - Rate limiting

4. **Unauthorized Access**
   - Authentication required
   - Per-adapter permissions
   - Tool-level access control

5. **Data Exfiltration**
   - Network restrictions in sandbox
   - Audit logging
   - Output size limits

### Assumptions

- Gateway runs in a trusted environment
- Configuration files are secure
- Network between gateway and data sources is trusted
- Secrets are properly managed

## Best Practices

### 1. Secret Management

**Never hardcode secrets:**

```yaml
# ❌ Bad
sources:
  - name: "api"
    auth:
      token: "sk-1234567890abcdef"

# ✅ Good
sources:
  - name: "api"
    auth:
      token: "${API_TOKEN}"
```

**Use environment variables or secret managers:**

```bash
export API_TOKEN="sk-1234567890abcdef"
export DATABASE_URL="postgresql://user:pass@localhost/db"
```

### 2. Principle of Least Privilege

**Grant minimum necessary permissions:**

```yaml
sources:
  - name: "database"
    type: "postgresql"
    connection: "${DATABASE_URL}"
    tables: ["users", "orders"]  # Only specific tables
    read_only: true              # No writes
```

### 3. Network Segmentation

**Run gateway in isolated network:**

```
┌─────────────────┐
│   Public Net    │
│   (AI Agents)   │
└────────┬────────┘
         │ Firewall
┌────────▼────────┐
│    Gateway      │
│   (DMZ/VPC)     │
└────────┬────────┘
         │ Firewall
┌────────▼────────┐
│  Internal Net   │
│  (Databases)    │
└─────────────────┘
```

### 4. CLI Tool Safety

**Whitelist allowed commands:**

```yaml
sources:
  - name: "kubectl"
    type: "cli"
    command: "kubectl"
    allowed_args:
      - "get"
      - "describe"
      - "logs"
    # Dangerous commands like "delete" not allowed
    sandbox: true
```

### 5. Database Security

**Use read-only connections when possible:**

```yaml
sources:
  - name: "analytics-db"
    type: "postgresql"
    connection: "${READONLY_DATABASE_URL}"
    read_only: true
```

**Limit table access:**

```yaml
sources:
  - name: "app-db"
    type: "postgresql"
    tables: ["public.users", "public.orders"]
    # Excludes sensitive tables like "auth.credentials"
```

### 6. CORS Configuration

**Restrict allowed origins:**

```yaml
security:
  allowed_origins:
    - "https://app.example.com"
    - "https://admin.example.com"
```

### 7. TLS/HTTPS

**Always use HTTPS in production:**

```bash
# Use reverse proxy (nginx, Caddy) for TLS termination
# Or configure gateway with TLS certificates
```

### 8. Regular Updates

**Keep dependencies updated:**

```bash
# Update Rust dependencies
cargo update

# Update Node dependencies
cd sdk && npm update
```

## Security Checklist

Before deploying to production:

- [ ] All secrets in environment variables
- [ ] TLS/HTTPS enabled
- [ ] Rate limiting configured
- [ ] Sandbox enabled
- [ ] Audit logging enabled
- [ ] CORS properly configured
- [ ] Database connections use least privilege
- [ ] CLI tools have argument whitelists
- [ ] Network segmentation in place
- [ ] Regular security updates scheduled
- [ ] Monitoring and alerting configured
- [ ] Incident response plan documented

## Incident Response

### 1. Detection

Monitor for:
- Unusual tool execution patterns
- Rate limit violations
- Authentication failures
- Execution timeouts
- Error spikes

### 2. Response

If compromise suspected:

1. **Isolate:** Stop the gateway
2. **Investigate:** Review audit logs
3. **Contain:** Revoke compromised credentials
4. **Recover:** Restore from known good state
5. **Learn:** Update security measures

### 3. Audit Log Analysis

```bash
# Find failed authentication attempts
grep "auth_failed" logs/gateway.log

# Find unusual tool executions
grep "tool_executed" logs/gateway.log | grep -v "normal_pattern"

# Find rate limit violations
grep "rate_limit_exceeded" logs/gateway.log
```

## Compliance

### GDPR

- Audit logs may contain personal data
- Implement data retention policies
- Provide data export capabilities
- Support data deletion requests

### SOC 2

- Enable comprehensive audit logging
- Implement access controls
- Regular security reviews
- Incident response procedures

### HIPAA

- Encrypt data in transit and at rest
- Implement access controls
- Audit all data access
- Business associate agreements

## Reporting Security Issues

If you discover a security vulnerability:

1. **Do not** open a public issue
2. Email security@universal-mcp-gateway.dev
3. Include:
   - Description of vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)

We will respond within 48 hours.

## Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [Node.js Security Best Practices](https://nodejs.org/en/docs/guides/security/)
