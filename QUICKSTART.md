# Raworc MCP Server - Quick Start Guide

This guide will help you install and use the Raworc MCP server in just a few minutes.

## Prerequisites

- **Node.js** (version 16.0.0 or higher)
- **Rust** (version 1.70.0 or higher) - for building the binary
- **Raworc Account** - you need credentials for the Raworc platform

## Step 1: Install the MCP Server

### Option A: Quick Install (Recommended)

```bash
# Install and run directly with npx
npx @raworc/mcp-server --help
```

This will automatically download, build, and run the MCP server. No permanent installation needed!

### Option B: Global Installation

```bash
# Install globally for repeated use
npm install -g @raworc/mcp-server

# Verify installation
raworc-mcp --help
```

## Step 2: Get Your Authentication Token

You need a token to authenticate with the Raworc platform:

```bash
# Get your auth token
curl -X POST http://raworc.remoteagent.com:9000/api/v0/auth/login \
  -H "Content-Type: application/json" \
  -d '{"user": "your-username", "pass": "your-password"}'
```

**Example Response:**
```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_at": "2024-12-31T23:59:59Z"
}
```

Copy the `token` value - you'll need it for the next steps.

## Step 3: Test the MCP Server

### Quick Test (No Auth Token Required)

First, test that the installation works:

```bash
# Test the installation
node examples/test-installation.js

# Or test manually
npx @raworc/mcp-server --help
```

### Full Test (Requires Auth Token)

Once you have your auth token, test the full functionality:

```bash
# Test with npx
npx @raworc/mcp-server --auth-token YOUR_TOKEN_HERE --log-level debug

# Or if installed globally
raworc-mcp --auth-token YOUR_TOKEN_HERE --log-level debug
```

You should see initialization messages and the server should start successfully.

## Step 4: Configure Claude Desktop

Add this configuration to your Claude Desktop settings:

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

**Important:** Replace `YOUR_TOKEN_HERE` with the actual token you got in Step 2.

## Step 5: Use in Claude Desktop

1. **Restart Claude Desktop** to load the new configuration
2. **Test the connection** by typing:
   ```
   @raworc health_check
   ```
3. **Try other commands**:
   ```
   @raworc list_spaces
   @raworc list_sessions
   @raworc create_session
   ```

## Common Commands

### Session Management
```
@raworc list_sessions
@raworc create_session
@raworc send_message
@raworc get_messages
@raworc pause_session
@raworc resume_session
@raworc terminate_session
```

### Space and Agent Management
```
@raworc list_spaces
@raworc list_agents
@raworc get_agent_logs
```

### Secret Management
```
@raworc list_secrets
@raworc set_secret
@raworc get_secret
@raworc delete_secret
```

### System Information
```
@raworc health_check
@raworc get_version
```

## Troubleshooting

### "Command not found" Error
```bash
# Make sure Node.js is installed
node --version

# Try installing globally
npm install -g @raworc/mcp-server
```

### "Authentication failed" Error
- Check your username and password
- Make sure your Raworc account is active
- Verify the API URL is correct: `http://raworc.remoteagent.com:9000/api/v0`

### "Binary not found" Error
```bash
# The package needs to build the Rust binary
# This should happen automatically, but you can force it:
npm install -g @raworc/mcp-server
```

### "Network error" Error
- Check your internet connection
- Verify you can reach `raworc.remoteagent.com:9000`
- Try accessing the web UI first: `http://raworc.remoteagent.com:9000`

## Environment Variables

You can also configure the server using environment variables:

```bash
export RAWORC_API_URL="http://raworc.remoteagent.com:9000/api/v0"
export RAWORC_AUTH_TOKEN="your-token-here"
export RAWORC_DEFAULT_SPACE="default"
export RAWORC_TIMEOUT="30"
export LOG_LEVEL="info"
```

## Advanced Usage

### Command Line Testing
```bash
# Test individual commands
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | raworc-mcp --auth-token your-token

# Test with custom configuration
raworc-mcp --auth-token your-token --default-space production --timeout 60 --log-level debug
```

### Custom Configuration
```bash
# Use a different API URL
raworc-mcp --api-url https://custom-raworc-instance.com/api/v0 --auth-token your-token

# Use username/password instead of token
raworc-mcp --username your-username --password your-password
```

## Need Help?

- **Documentation**: [README.md](README.md)
- **Testing Guide**: [TESTING.md](TESTING.md)
- **Raworc API Docs**: [https://raworc.com/docs/api/rest-api](https://raworc.com/docs/api/rest-api)
- **Issues**: [GitHub Issues](https://github.com/yourusername/raworc-mcp/issues)

## Next Steps

Once you have the basic setup working:

1. **Explore the available tools** - try different commands
2. **Set up your spaces and sessions** - organize your work
3. **Configure secrets** - store sensitive data securely
4. **Monitor agents** - track your agent performance
5. **Integrate with your workflow** - use the MCP server in your daily tasks

Happy coding! 🚀
