# Raworc MCP Server

A production-ready Model Context Protocol (MCP) server for Raworc, enabling AI assistants to seamlessly interact with Raworc's agent runtime platform.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.70+-blue.svg)](https://www.rust-lang.org/)
[![npm](https://img.shields.io/badge/npm-@raworc/mcp--server-red.svg)](https://www.npmjs.com/package/@raworc/mcp-server)

## 🚀 Quick Start

### Option 1: Install via npx (Recommended)

```bash
# Test the installation
npx @raworc/mcp-server --help

# Use directly with Claude Desktop
npx @raworc/mcp-server
```

### Option 2: Install from Source

```bash
# Clone the repository
git clone https://github.com/harshapalnati/raworc-mcpserver.git
cd raworc-mcpserver

# Build the project
cargo build --release

# Test the installation
./target/release/raworc-mcp --help
```

## 📋 Prerequisites

- **Rust**: Version 1.70 or higher
- **Node.js**: Version 16.0 or higher (for npx installation)
- **Raworc Account**: Access to Raworc platform
- **Network Access**: Ability to reach `api.remoteagent.com`

## 🔧 Configuration

### Environment Variables

| Variable | Description | Default | Required |
|----------|-------------|---------|----------|
| `RAWORC_API_URL` | Raworc API base URL | `https://api.remoteagent.com/api/v0` | No |
| `RAWORC_AUTH_TOKEN` | JWT authentication token | - | Yes |
| `RAWORC_DEFAULT_SPACE` | Default space for operations | - | No |
| `RAWORC_TIMEOUT` | Request timeout in seconds | `30` | No |
| `LOG_LEVEL` | Logging level | `info` | No |

### Getting Your Authentication Token

```bash
curl -X POST https://api.remoteagent.com/api/v0/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "user": "your-username",
    "pass": "your-password"
  }'
```

## 🎯 Features

- **Session Management**: Create, pause, resume, and terminate sessions
- **Message Handling**: Send and retrieve messages from sessions
- **Space Management**: List and manage spaces
- **Agent Operations**: Deploy, monitor, and control agents
- **Secret Management**: Secure storage and retrieval of secrets
- **Real-time Communication**: Send messages and receive responses from agents
- **Health Monitoring**: Check API health and version information

## 🔌 Claude Desktop Integration

Add the MCP server to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "raworc": {
      "command": "npx",
      "args": ["@raworc/mcp-server"],
      "env": {
        "RAWORC_API_URL": "https://api.remoteagent.com/api/v0",
        "RAWORC_AUTH_TOKEN": "your-jwt-token",
        "RAWORC_DEFAULT_SPACE": "your-space",
        "RAWORC_TIMEOUT": "30",
        "LOG_LEVEL": "info"
      }
    }
  }
}
```

### Alternative: Direct Binary Path

```json
{
  "mcpServers": {
    "raworc": {
      "command": "/path/to/raworc-mcp",
      "env": {
        "RAWORC_API_URL": "https://api.remoteagent.com/api/v0",
        "RAWORC_AUTH_TOKEN": "your-jwt-token",
        "RAWORC_DEFAULT_SPACE": "your-space"
      }
    }
  }
}
```

## 🛠️ Available Tools

### Session Management

#### `list_sessions`
List all sessions in a space.

```json
{
  "name": "list_sessions",
  "arguments": {
    "space": "production"
  }
}
```

#### `create_session`
Create a new session.

```json
{
  "name": "create_session",
  "arguments": {
    "space": "development",
    "metadata": {
      "purpose": "testing",
      "user": "developer"
    }
  }
}
```

#### `get_session`
Get session details by ID.

```json
{
  "name": "get_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d"
  }
}
```

#### `send_message`
Send a message to a session.

```json
{
  "name": "send_message",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "content": "Generate a Python script to calculate fibonacci numbers"
  }
}
```

#### `get_messages`
Get messages from a session.

```json
{
  "name": "get_messages",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "limit": 10
  }
}
```

#### `pause_session`
Pause a session.

```json
{
  "name": "pause_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d"
  }
}
```

#### `resume_session`
Resume a paused session.

```json
{
  "name": "resume_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d"
  }
}
```

#### `terminate_session`
Terminate a session.

```json
{
  "name": "terminate_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d"
  }
}
```

### Space Management

#### `list_spaces`
List all spaces.

```json
{
  "name": "list_spaces",
  "arguments": {}
}
```

### Agent Management

#### `list_agents`
List agents in a space.

```json
{
  "name": "list_agents",
  "arguments": {
    "space": "production"
  }
}
```

#### `get_agent_logs`
Get logs for a specific agent.

```json
{
  "name": "get_agent_logs",
  "arguments": {
    "space": "production",
    "agent_name": "my-agent"
  }
}
```

### Secret Management

#### `list_secrets`
List secrets in a space.

```json
{
  "name": "list_secrets",
  "arguments": {
    "space": "production"
  }
}
```

#### `get_secret`
Get a specific secret.

```json
{
  "name": "get_secret",
  "arguments": {
    "space": "production",
    "key": "api-key"
  }
}
```

#### `set_secret`
Set a secret value.

```json
{
  "name": "set_secret",
  "arguments": {
    "space": "production",
    "key": "api-key",
    "value": "your-secret-value"
  }
}
```

#### `delete_secret`
Delete a secret.

```json
{
  "name": "delete_secret",
  "arguments": {
    "space": "production",
    "key": "api-key"
  }
}
```

### System Information

#### `health_check`
Check Raworc API health.

```json
{
  "name": "health_check",
  "arguments": {}
}
```

#### `get_version`
Get Raworc API version.

```json
{
  "name": "get_version",
  "arguments": {}
}
```

## 🧪 Testing

### Quick Test

```bash
# Test with npx
npx @raworc/mcp-server --help

# Test health check
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | npx @raworc/mcp-server

# Test list spaces
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "list_spaces", "arguments": {}}}' | npx @raworc/mcp-server
```

### Manual API Testing

```bash
# Health check
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/health

# Get version
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/version

# List spaces
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/spaces
```

## 🏗️ Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/harshapalnati/raworc-mcpserver.git
cd raworc-mcpserver

# Build the project
cargo build --release

# Run tests
cargo test

# Install with npm (for npx distribution)
npm install
```

### Project Structure

```
raworc-mcpserver/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library exports
│   ├── client.rs        # Raworc API client
│   ├── error.rs         # Error handling
│   ├── models.rs        # Data models
│   └── mcp.rs           # MCP server implementation
├── bin/
│   └── raworc-mcp.js    # JavaScript wrapper for npx
├── scripts/
│   └── postinstall.js   # Build script for npm installation
├── tests/
│   └── integration_test.rs  # Integration tests
├── examples/
│   ├── claude-desktop-config.json  # Example Claude Desktop config
│   └── usage-example.md            # Usage examples
├── Cargo.toml           # Rust dependencies
├── package.json         # npm package configuration
├── QUICKSTART.md        # Quick start guide
├── TESTING.md           # Testing guide
└── README.md           # This file
```

## 🐛 Troubleshooting

### Common Issues

1. **Authentication Failed**
   - Verify your token is valid and not expired
   - Check that you have the correct permissions

2. **Connection Issues**
   - Verify you can reach `api.remoteagent.com`
   - Check your network connectivity

3. **Permission Errors**
   - Ensure your account has the necessary permissions
   - Check that you're using the correct space

### Debug Mode

```bash
export LOG_LEVEL="debug"
export RAWORC_API_URL="https://api.remoteagent.com/api/v0"
export RAWORC_AUTH_TOKEN="your-token"
npx @raworc/mcp-server
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📞 Support

- **Documentation**: [Raworc API Documentation](https://raworc.com/docs/api/rest-api)
- **Issues**: [GitHub Issues](https://github.com/harshapalnati/raworc-mcpserver/issues)
- **Discussions**: [GitHub Discussions](https://github.com/harshapalnati/raworc-mcpserver/discussions)

## 📝 Changelog

### v0.1.0

- Initial release
- Full MCP protocol implementation
- Complete Raworc API integration
- Session, space, agent, and secret management
- Health monitoring and version information
- npx installation support
- Production-ready error handling

## 🔗 Links

- **Repository**: [https://github.com/harshapalnati/raworc-mcpserver](https://github.com/harshapalnati/raworc-mcpserver)
- **npm Package**: [@raworc/mcp-server](https://www.npmjs.com/package/@raworc/mcp-server)
- **Raworc API**: [https://raworc.com/docs/api/rest-api](https://raworc.com/docs/api/rest-api)
