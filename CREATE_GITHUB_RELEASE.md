# How to Create GitHub Release v0.1.0

Follow these steps to create the official GitHub release for Universal MCP Gateway.

## Step 1: Go to Releases Page

Visit: **https://github.com/bhaskarvilles/universal-mcp-gateway/releases/new**

Or navigate manually:
1. Go to https://github.com/bhaskarvilles/universal-mcp-gateway
2. Click on "Releases" (right sidebar)
3. Click "Draft a new release" button

## Step 2: Fill in Release Details

### Choose a tag
- **Tag:** `v0.1.0`
- **Target:** `main` branch
- Click "Create new tag: v0.1.0 on publish"

### Release title
```
v0.1.0 - Initial Release
```

### Release description

Copy the entire content from `RELEASE_NOTES_v0.1.0.md` file, or use this:

```markdown
# Universal MCP Gateway v0.1.0 - Initial Release

**"Docker for AI Tools"** - Connect anything to AI agents in one command.

## 🎉 What's New

This is the initial public release of Universal MCP Gateway!

### ✨ Features

#### Core Gateway Engine
- ✅ **Production-ready MCP protocol implementation**
- ✅ **High-performance Rust backend**
- ✅ **Plugin-based adapter system**
- ✅ **Security sandboxing with WASM**
- ✅ **Authentication & authorization**
- ✅ **Rate limiting**

#### Built-in Adapters
- ✅ **OpenAPI/Swagger Adapter** - Auto-generate tools from API specs
- ✅ **PostgreSQL Adapter** - Database introspection and queries
- ✅ **CLI Wrapper** - Safely expose command-line tools

#### TypeScript SDK
- ✅ **Full-featured client library**
- ✅ **Type-safe API with TypeScript**
- ✅ **Published on npm:** `npm install universal-mcp-gateway-sdk`

## 🚀 Quick Start

### Using Docker
```bash
docker run -p 8080:8080 -v $(pwd)/config.yaml:/app/config.yaml bhaskarvilles/universal-mcp-gateway:0.1.0
```

### Using npm
```bash
npm install universal-mcp-gateway-sdk
```

### From Source
```bash
git clone https://github.com/bhaskarvilles/universal-mcp-gateway.git
cd universal-mcp-gateway
cargo build --release
./target/release/mcp-gateway --config examples/config.yaml
```

## 📚 Documentation

- [Getting Started](GETTING_STARTED.md)
- [Architecture](docs/architecture.md)
- [API Reference](docs/api-reference.md)
- [Security Guide](docs/security.md)
- [Deployment Guide](docs/deployment.md)
- [SDK Documentation](sdk/README.md)

## 📦 Installation

**NPM Package:**
```bash
npm install universal-mcp-gateway-sdk
```
https://www.npmjs.com/package/universal-mcp-gateway-sdk

**Docker Image:**
```bash
docker pull bhaskarvilles/universal-mcp-gateway:0.1.0
```

**From Source:**
```bash
cargo build --release
```

## 🔒 Security Features

- WASM Sandboxing
- Authentication (API keys, OAuth2, JWT)
- Rate Limiting
- Input Validation
- Audit Logging
- TLS/SSL Support

## 🗺️ Roadmap

### v0.2.0 (Next)
- GraphQL adapter
- MySQL adapter
- MongoDB adapter
- Webhook support

See [ROADMAP.md](ROADMAP.md) for full roadmap.

## 🤝 Contributing

Contributions welcome! See [CONTRIBUTING.md](CONTRIBUTING.md)

## 📄 License

MIT License - See [LICENSE](LICENSE)

## 🔗 Links

- **Repository:** https://github.com/bhaskarvilles/universal-mcp-gateway
- **NPM Package:** https://www.npmjs.com/package/universal-mcp-gateway-sdk
- **Issues:** https://github.com/bhaskarvilles/universal-mcp-gateway/issues
- **Discussions:** https://github.com/bhaskarvilles/universal-mcp-gateway/discussions

---

**⭐ Star this repository if you find it useful!**

*Released: May 7, 2026*
```

## Step 3: Optional - Attach Binaries

If you have pre-built binaries, you can attach them:

### Build binaries for different platforms:

**Linux (x86_64):**
```bash
cargo build --release --target x86_64-unknown-linux-gnu
```

**macOS (Intel):**
```bash
cargo build --release --target x86_64-apple-darwin
```

**macOS (Apple Silicon):**
```bash
cargo build --release --target aarch64-apple-darwin
```

**Windows:**
```bash
cargo build --release --target x86_64-pc-windows-msvc
```

Then attach the binaries from `target/release/mcp-gateway` (or `target/<platform>/release/mcp-gateway`)

### Current Binary Available:
- `target/release/mcp-gateway` (3.0MB) - Your current platform

You can attach this as:
- Filename: `mcp-gateway-macos-arm64` (or appropriate platform name)

## Step 4: Release Options

- ✅ **Set as the latest release** - Check this box
- ✅ **Create a discussion for this release** - Optional but recommended
- ⬜ **Set as a pre-release** - Leave unchecked (this is a stable release)

## Step 5: Publish Release

Click the **"Publish release"** button at the bottom.

## After Publishing

### 1. Verify the Release
Visit: https://github.com/bhaskarvilles/universal-mcp-gateway/releases/tag/v0.1.0

### 2. Update README Badge (Optional)
Add a release badge to your README.md:

```markdown
[![GitHub release](https://img.shields.io/github/v/release/bhaskarvilles/universal-mcp-gateway)](https://github.com/bhaskarvilles/universal-mcp-gateway/releases)
```

### 3. Announce the Release

**Twitter/X:**
```
🚀 Universal MCP Gateway v0.1.0 is here!

"Docker for AI Tools" - Connect any API, database, or CLI to AI agents in one command.

✅ Rust backend
✅ TypeScript SDK
✅ OpenAPI/PostgreSQL/CLI adapters
✅ Production-ready

⭐ https://github.com/bhaskarvilles/universal-mcp-gateway

#AI #Rust #OpenSource
```

**LinkedIn:**
```
I'm excited to announce the release of Universal MCP Gateway v0.1.0! 🚀

This open-source project makes it easy to connect any data source to AI agents:
• Auto-generate tools from OpenAPI specs
• Database introspection (PostgreSQL)
• Safe CLI tool execution
• TypeScript SDK included

Built with Rust for performance and safety.

Check it out: https://github.com/bhaskarvilles/universal-mcp-gateway

#AI #MachineLearning #OpenSource #Rust
```

**Reddit (r/rust):**
```
Title: [Release] Universal MCP Gateway v0.1.0 - Connect anything to AI agents

I'm happy to announce the first release of Universal MCP Gateway!

It's an open-source gateway that automatically converts REST APIs, databases, and CLI tools into MCP-compatible tools for AI agents.

Built with Rust (Tokio + Axum), includes TypeScript SDK, Docker support, and comprehensive documentation.

GitHub: https://github.com/bhaskarvilles/universal-mcp-gateway
NPM: https://www.npmjs.com/package/universal-mcp-gateway-sdk

Would love feedback from the community!
```

## Alternative: Using GitHub CLI

If you install GitHub CLI (`gh`), you can create the release from command line:

```bash
# Install gh (macOS)
brew install gh

# Login
gh auth login

# Create release
gh release create v0.1.0 \
  --title "v0.1.0 - Initial Release" \
  --notes-file RELEASE_NOTES_v0.1.0.md \
  --latest

# Optional: Attach binary
gh release upload v0.1.0 target/release/mcp-gateway#mcp-gateway-macos-arm64
```

## Checklist

Before publishing, verify:

- [ ] Tag is `v0.1.0`
- [ ] Target is `main` branch
- [ ] Title is "v0.1.0 - Initial Release"
- [ ] Description is complete and formatted
- [ ] "Set as the latest release" is checked
- [ ] "Set as a pre-release" is NOT checked
- [ ] All links in description work
- [ ] Ready to announce to community

## Need Help?

If you encounter any issues:
1. Check GitHub's release documentation: https://docs.github.com/en/repositories/releasing-projects-on-github
2. Ensure you have write access to the repository
3. Make sure the v0.1.0 tag exists (it should from earlier push)

---

**Ready to launch! 🚀**
