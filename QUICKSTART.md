# Quick Start Guide

Get up and running with the Raworc MCP Server in minutes!

## Prerequisites

- **Rust**: Version 1.70 or higher (for source builds)
- **Node.js**: Version 16.0 or higher (for npx installation)
- **Raworc Account**: Access to Raworc platform
- **Network Access**: Ability to reach `api.remoteagent.com`

## Option 1: Install via npx (Recommended)

### Step 1: Test the Installation

```bash
# Test that npx can run the server
npx @raworc/mcp-server --help
```

### Step 2: Get Your Authentication Token

First, you need to authenticate with Raworc and get a JWT token:

```bash
curl -X POST https://api.remoteagent.com/api/v0/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "user": "your-username",
    "pass": "your-password"
  }'
```

Response:
```json
{
  "token": "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9...",
  "token_type": "Bearer",
  "expires_at": "2024-01-15T11:30:00Z"
}
```

Save the token for the next steps.

### Step 3: Test the MCP Server

```bash
# Set your authentication token
export RAWORC_AUTH_TOKEN="your-jwt-token"

# Test health check
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | npx @raworc/mcp-server
```

Expected response:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "content": [
      {
        "type": "text",
        "text": ""
      }
    ]
  }
}
```

### Step 4: Configure Claude Desktop

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

### Step 5: Test with Claude Desktop

Restart Claude Desktop and test the integration:

```
@raworc health_check
@raworc list_spaces
@raworc get_version
```

## Option 2: Install from Source

### Step 1: Clone and Build

```bash
# Clone the repository
git clone https://github.com/harshapalnati/raworc-mcpserver.git
cd raworc-mcpserver

# Build the release version
cargo build --release
```

### Step 2: Get Authentication Token

Follow the same authentication steps as above.

### Step 3: Test the Installation

```bash
# Set your authentication token
export RAWORC_AUTH_TOKEN="your-jwt-token"

# Test health check
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | ./target/release/raworc-mcp
```

### Step 4: Configure Claude Desktop

```json
{
  "mcpServers": {
    "raworc": {
      "command": "/path/to/raworc-mcpserver/target/release/raworc-mcp",
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

## Troubleshooting

### Common Issues

1. **Authentication Failed**
   - Verify your username and password
   - Check that your token hasn't expired
   - Ensure you have the correct permissions

2. **Connection Issues**
   - Verify you can reach `api.remoteagent.com`
   - Check your network connectivity
   - Try accessing the web UI first: `https://api.remoteagent.com`

3. **Permission Errors**
   - Ensure your account has the necessary permissions
   - Check that you're using the correct space

### Debug Mode

Enable debug logging:

```bash
export LOG_LEVEL="debug"
export RAWORC_API_URL="https://api.remoteagent.com/api/v0"
export RAWORC_AUTH_TOKEN="your-token"
npx @raworc/mcp-server
```

### Manual Testing

Test individual endpoints:

```bash
# Health check
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/health

# List spaces
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/spaces

# Get version
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/version
```

## Next Steps

- **Create Sessions**: Start working with Raworc sessions
- **Manage Agents**: Deploy and monitor agents
- **Handle Secrets**: Store and retrieve sensitive data
- **Explore Spaces**: Organize your work with spaces

For detailed API documentation, see the [Raworc API Reference](https://raworc.com/docs/api/rest-api).
