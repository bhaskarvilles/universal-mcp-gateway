# Security Policy

## Reporting a Vulnerability

We take the security of Universal MCP Gateway seriously. If you discover a security vulnerability, please follow these steps:

### 1. Do Not Open a Public Issue

Please **do not** open a public GitHub issue for security vulnerabilities.

### 2. Report Privately

Send an email to: **security@universal-mcp-gateway.dev** (or create a private security advisory on GitHub)

Include:
- Description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)
- Your contact information

### 3. Response Timeline

- **Initial Response:** Within 48 hours
- **Status Update:** Within 7 days
- **Fix Timeline:** Depends on severity
  - Critical: 1-7 days
  - High: 7-30 days
  - Medium: 30-90 days
  - Low: Next release cycle

## Supported Versions

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Security Best Practices

When using Universal MCP Gateway:

1. **Use Environment Variables** for secrets
2. **Enable Sandboxing** in production
3. **Use TLS/HTTPS** for all connections
4. **Implement Rate Limiting** appropriately
5. **Run with Least Privilege** (non-root user)
6. **Keep Dependencies Updated** regularly
7. **Monitor Audit Logs** for suspicious activity
8. **Use Read-Only Mode** for databases when possible

## Known Security Considerations

### Adapter Security

- **OpenAPI Adapter:** Validates API specs but trusts the remote API
- **PostgreSQL Adapter:** Use read-only connections when possible
- **CLI Adapter:** Whitelist allowed commands strictly

### Sandbox Limitations

The WASM sandbox provides isolation but:
- Network access may be available depending on configuration
- File system access is restricted but not eliminated
- Resource limits should be configured appropriately

## Security Features

- ✅ WASM-based execution sandboxing
- ✅ Authentication (API keys, OAuth2, JWT)
- ✅ Rate limiting per adapter
- ✅ Input validation and sanitization
- ✅ Audit logging
- ✅ Timeout enforcement
- ✅ Resource limits

## Disclosure Policy

- We will acknowledge receipt of your vulnerability report
- We will provide regular updates on our progress
- We will credit you in the security advisory (unless you prefer to remain anonymous)
- We will coordinate the disclosure timeline with you

## Bug Bounty

We currently do not have a bug bounty program, but we deeply appreciate security researchers who help keep our project secure.

## Contact

For security concerns: security@universal-mcp-gateway.dev
For general questions: GitHub Issues or Discussions

---

Thank you for helping keep Universal MCP Gateway secure! 🔒
