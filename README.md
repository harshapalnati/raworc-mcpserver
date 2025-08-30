# Raworc MCP Server

A Model Context Protocol (MCP) server for Raworc, providing seamless integration between AI assistants and Raworc's agent runtime platform.

## 🚀 Quick Start

**Want to get started immediately?** Check out our [Quick Start Guide](QUICKSTART.md) for step-by-step instructions!

**One-liner installation:**
```bash
npx @raworc/mcp-server --help
```

## Features

- **Session Management**: Create, pause, resume, and terminate sessions
- **Message Handling**: Send and retrieve messages from sessions
- **Space Management**: List and manage spaces
- **Agent Management**: List agents and retrieve logs
- **Secret Management**: Secure storage and retrieval of secrets
- **Health Monitoring**: Check API health and version information
- **Full MCP Protocol Support**: Complete implementation of the Model Context Protocol

## Installation

### Prerequisites

- **Rust**: Version 1.70.0 or higher (for building the binary)
- **Node.js**: Version 16.0.0 or higher (for npx installation)
- **Network Access**: Ability to reach `raworc.remoteagent.com:9000`

### Quick Install with npx (Recommended)

```bash
# Install and run directly with npx
npx @raworc/mcp-server --help

# Or install globally
npm install -g @raworc/mcp-server
raworc-mcp --help
```

The package will automatically build the Rust binary during installation.

### Manual Installation

#### Option 1: Clone and Install

```bash
# Clone the repository
git clone https://github.com/yourusername/raworc-mcp.git
cd raworc-mcp

# Install with npm (builds Rust binary automatically)
npm install

# Run the server
npx raworc-mcp --help
```

#### Option 2: Direct Rust Build

```bash
# Clone the repository
git clone https://github.com/yourusername/raworc-mcp.git
cd raworc-mcp

# Build the release version
cargo build --release

# Install to your system (choose one):
# Option A: System-wide installation (requires sudo)
sudo cp target/release/raworc-mcp /usr/local/bin/

# Option B: User installation
mkdir -p ~/.local/bin
cp target/release/raworc-mcp ~/.local/bin/
echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
source ~/.bashrc
```

## Configuration

The MCP server can be configured using environment variables or command-line arguments:

### Environment Variables

- `RAWORC_API_URL`: Raworc API base URL (default: `http://raworc.remoteagent.com:9000/api/v0`)
- `RAWORC_AUTH_TOKEN`: Authentication token
- `RAWORC_USERNAME`: Username for authentication
- `RAWORC_PASSWORD`: Password for authentication
- `RAWORC_DEFAULT_SPACE`: Default space to use (default: `default`)
- `RAWORC_TIMEOUT`: Request timeout in seconds (default: `30`)
- `LOG_LEVEL`: Logging level (default: `info`)

### Command Line Arguments

```bash
raworc-mcp --api-url https://your-raworc-instance.com/api/v0 \
           --username your-username \
           --password your-password \
           --default-space production \
           --timeout 60 \
           --log-level debug
```

## Quick Start Guide

### Step 1: Install the MCP Server

Choose one of these installation methods:

#### Option A: Quick Install with npx (Recommended)
```bash
# Install and run directly (no permanent installation)
npx @raworc/mcp-server --help

# Or install globally for repeated use
npm install -g @raworc/mcp-server
```

#### Option B: Clone and Install
```bash
# Clone the repository
git clone https://github.com/yourusername/raworc-mcp.git
cd raworc-mcp

# Install (this builds the Rust binary automatically)
npm install
```

### Step 2: Get Your Raworc Auth Token

You need an authentication token to use the MCP server:

```bash
# Get your auth token using curl
curl -X POST http://raworc.remoteagent.com:9000/api/v0/auth/login \
  -H "Content-Type: application/json" \
  -d '{"user": "your-username", "pass": "your-password"}'
```

The response will contain your auth token. Copy it for the next step.

### Step 3: Test the MCP Server

```bash
# Test with npx
npx @raworc/mcp-server --auth-token YOUR_TOKEN_HERE --log-level debug

# Or if installed globally
raworc-mcp --auth-token YOUR_TOKEN_HERE --log-level debug
```

### Step 4: Configure Claude Desktop

Add this to your Claude Desktop configuration file:

```json
{
  "mcpServers": {
    "raworc": {
      "command": "npx",
      "args": ["@raworc/mcp-server"],
      "env": {
        "RAWORC_API_URL": "http://raworc.remoteagent.com:9000/api/v0",
        "RAWORC_AUTH_TOKEN": "YOUR_TOKEN_HERE"
      }
    }
  }
}
```

### Step 5: Use in Claude Desktop

Restart Claude Desktop and try these commands:

```
@raworc health_check
@raworc list_spaces
@raworc list_sessions
@raworc create_session
```

## Usage Examples

### Command Line Usage

```bash
# Basic usage with auth token
raworc-mcp --auth-token your-token

# With custom configuration
raworc-mcp --api-url http://raworc.remoteagent.com:9000/api/v0 \
           --auth-token your-token \
           --default-space production \
           --timeout 60 \
           --log-level debug

# Using environment variables
export RAWORC_AUTH_TOKEN="your-token"
export RAWORC_DEFAULT_SPACE="production"
raworc-mcp
```

### Environment Variables

You can configure the server using environment variables:

```bash
export RAWORC_API_URL="http://raworc.remoteagent.com:9000/api/v0"
export RAWORC_AUTH_TOKEN="your-auth-token"
export RAWORC_DEFAULT_SPACE="production"
export RAWORC_TIMEOUT="60"
export LOG_LEVEL="debug"
```

### Testing Individual Commands

You can test the MCP server by sending JSON-RPC messages:

```bash
# Health check
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | raworc-mcp --auth-token your-token

# List spaces
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "list_spaces", "arguments": {}}}' | raworc-mcp --auth-token your-token

# Create a session
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "create_session", "arguments": {"space": "default", "metadata": {"purpose": "testing"}}}}' | raworc-mcp --auth-token your-token
```

### Integration with MCP Clients

The server implements the Model Context Protocol and can be used with any MCP-compatible client. Here's an example configuration for Claude Desktop:

#### Using npx (Recommended)
```json
{
  "mcpServers": {
    "raworc": {
      "command": "npx",
      "args": ["@raworc/mcp-server"],
      "env": {
        "RAWORC_API_URL": "http://raworc.remoteagent.com:9000/api/v0",
        "RAWORC_AUTH_TOKEN": "your-token"
      }
    }
  }
}
```

#### Using Global Installation
```json
{
  "mcpServers": {
    "raworc": {
      "command": "raworc-mcp",
      "args": [],
      "env": {
        "RAWORC_API_URL": "http://raworc.remoteagent.com:9000/api/v0",
        "RAWORC_AUTH_TOKEN": "your-token"
      }
    }
  }
}
```

## Available Tools

### Session Management

#### `list_sessions`
List all sessions in a space.

**Parameters:**
- `space` (optional): Space name (uses default if not provided)

**Example:**
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

**Parameters:**
- `space` (optional): Space name (uses default if not provided)
- `metadata` (optional): Additional metadata object

**Example:**
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

**Parameters:**
- `session_id` (required): Session ID

**Example:**
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

**Parameters:**
- `session_id` (required): Session ID
- `content` (required): Message content

**Example:**
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

**Parameters:**
- `session_id` (required): Session ID
- `limit` (optional): Maximum number of messages to return

**Example:**
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

**Parameters:**
- `session_id` (required): Session ID

**Example:**
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

**Parameters:**
- `session_id` (required): Session ID

**Example:**
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

**Parameters:**
- `session_id` (required): Session ID

**Example:**
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

**Parameters:** None

**Example:**
```json
{
  "name": "list_spaces",
  "arguments": {}
}
```

### Agent Management

#### `list_agents`
List agents in a space.

**Parameters:**
- `space` (optional): Space name (uses default if not provided)

**Example:**
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

**Parameters:**
- `space` (required): Space name
- `agent_name` (required): Agent name

**Example:**
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

**Parameters:**
- `space` (optional): Space name (uses default if not provided)

**Example:**
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

**Parameters:**
- `space` (required): Space name
- `key` (required): Secret key

**Example:**
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

**Parameters:**
- `space` (required): Space name
- `key` (required): Secret key
- `value` (required): Secret value

**Example:**
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

**Parameters:**
- `space` (required): Space name
- `key` (required): Secret key

**Example:**
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

**Parameters:** None

**Example:**
```json
{
  "name": "health_check",
  "arguments": {}
}
```

#### `get_version`
Get Raworc API version.

**Parameters:** None

**Example:**
```json
{
  "name": "get_version",
  "arguments": {}
}
```

## Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/raworc-mcp.git
cd raworc-mcp

# Build the project
cargo build --release

# Run tests
cargo test

# Install with npm (optional)
npm install
```

### Project Structure

```
raworc-mcp/
├── src/
│   ├── main.rs          # Entry point
│   ├── lib.rs           # Library exports
│   ├── client.rs        # Raworc API client
│   ├── error.rs         # Error handling
│   ├── models.rs        # Data models
│   ├── mcp.rs           # MCP server implementation
│   └── server.rs        # MCP protocol server
├── bin/
│   └── raworc-mcp.js    # JavaScript wrapper for npx
├── scripts/
│   └── postinstall.js   # Build script for npm installation
├── tests/
│   └── integration_test.rs  # Integration tests
├── examples/
│   ├── claude-desktop-config.json  # Example Claude Desktop config
│   ├── usage-example.md            # Usage examples
│   └── test-installation.js        # Installation test script
├── Cargo.toml           # Rust dependencies
├── package.json         # npm package configuration
├── install.sh           # Linux/macOS installation script
├── install.ps1          # Windows installation script
├── QUICKSTART.md        # Quick start guide
├── TESTING.md           # Testing guide
└── README.md           # This file
```

### Testing

```bash
# Run Rust tests
cargo test

# Run integration tests
cargo test --test integration
```

## Error Handling

The MCP server provides comprehensive error handling with detailed error messages. Common error types include:

- **Authentication Errors**: Invalid credentials or missing authentication
- **API Errors**: Raworc API errors with status codes and messages
- **Validation Errors**: Invalid input parameters
- **Network Errors**: Connection issues or timeouts
- **MCP Protocol Errors**: Protocol-related errors

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Support

- **Documentation**: [Raworc API Documentation](https://raworc.com/docs/api/rest-api)
- **Issues**: [GitHub Issues](https://github.com/yourusername/raworc-mcp/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/raworc-mcp/discussions)

## Changelog

### v0.1.0
- Initial release
- Full MCP protocol implementation
- Complete Raworc API integration
- Session, space, agent, and secret management
- Health monitoring and version information
