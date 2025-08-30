
# Raworc MCP Server Testing Guide

This guide will help you get a real auth token and test the Raworc MCP server with actual data.

## Prerequisites

1. **Raworc Account**: You need a valid account on the Raworc cloud platform
2. **Rust**: Version 1.70.0 or higher
3. **Network Access**: Ability to reach `raworc.remoteagent.com:9000`

## Step 1: Get Your Auth Token

### Option A: Using curl to get your auth token

You can get your auth token using curl:

```bash
# Authenticate and get token
curl -X POST http://raworc.remoteagent.com:9000/api/v0/auth/login \
  -H "Content-Type: application/json" \
  -d '{"user": "your-username", "pass": "your-password"}'

# The response will contain your auth token
```

### Option B: Manual Authentication

If you prefer to get the token manually:

```bash
# Using curl
curl -X POST http://raworc.remoteagent.com:9000/api/v0/auth/login \
  -H "Content-Type: application/json" \
  -d '{"user": "your-username", "pass": "your-password"}'

# Or using the MCP server directly
raworc-mcp --username your-username --password your-password --log-level debug
```

### Option C: Web UI

1. Go to `http://raworc.remoteagent.com:9000` in your browser
2. Log in with your credentials
3. Look for API tokens in your account settings or developer section

## Step 2: Test the MCP Server

### Basic Testing

```bash
# Set your auth token
export RAWORC_AUTH_TOKEN="your-actual-token-here"

# Test the MCP server
raworc-mcp --log-level debug
```

### Testing Individual Tools

You can test the MCP server by sending JSON-RPC messages to it. Here's how:

```bash
# Start the MCP server in one terminal
raworc-mcp --auth-token your-token --log-level debug

# In another terminal, send test messages
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | raworc-mcp --auth-token your-token
```

### Testing with Real Data

Once you have a valid token, you can test all the available tools:

#### 1. Health Check
```bash
# Test API health
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | raworc-mcp --auth-token your-token
```

#### 2. List Spaces
```bash
# List available spaces
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "list_spaces", "arguments": {}}}' | raworc-mcp --auth-token your-token
```

#### 3. Create a Session
```bash
# Create a new session
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "create_session", "arguments": {"space": "default", "metadata": {"purpose": "testing"}}}}' | raworc-mcp --auth-token your-token
```

#### 4. List Sessions
```bash
# List sessions in a space
echo '{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_sessions", "arguments": {"space": "default"}}}' | raworc-mcp --auth-token your-token
```

#### 5. Send a Message
```bash
# Send a message to a session (replace SESSION_ID with actual ID)
echo '{"jsonrpc": "2.0", "id": 5, "method": "tools/call", "params": {"name": "send_message", "arguments": {"session_id": "SESSION_ID", "content": "Hello from MCP server!"}}}' | raworc-mcp --auth-token your-token
```

#### 6. List Agents
```bash
# List agents in a space
echo '{"jsonrpc": "2.0", "id": 6, "method": "tools/call", "params": {"name": "list_agents", "arguments": {"space": "default"}}}' | raworc-mcp --auth-token your-token
```

#### 7. Manage Secrets
```bash
# Set a secret
echo '{"jsonrpc": "2.0", "id": 7, "method": "tools/call", "params": {"name": "set_secret", "arguments": {"space": "default", "key": "test-key", "value": "test-value"}}}' | raworc-mcp --auth-token your-token

# Get the secret
echo '{"jsonrpc": "2.0", "id": 8, "method": "tools/call", "params": {"name": "get_secret", "arguments": {"space": "default", "key": "test-key"}}}' | raworc-mcp --auth-token your-token

# List secrets
echo '{"jsonrpc": "2.0", "id": 9, "method": "tools/call", "params": {"name": "list_secrets", "arguments": {"space": "default"}}}' | raworc-mcp --auth-token your-token
```

## Step 3: Integration Testing

### Test with Claude Desktop

1. **Install the MCP server**:
    ```bash
    # Option A: Using npx (recommended)
    npx @raworc/mcp-server --help
    
    # Option B: Clone and install
    git clone https://github.com/yourusername/raworc-mcp.git
    cd raworc-mcp
    npm install
    ```

2. **Configure Claude Desktop**:
    Add this to your Claude Desktop configuration:
    ```json
    {
      "mcpServers": {
        "raworc": {
          "command": "npx",
          "args": ["@raworc/mcp-server"],
          "env": {
            "RAWORC_API_URL": "http://raworc.remoteagent.com:9000/api/v0",
            "RAWORC_AUTH_TOKEN": "your-actual-token-here"
          }
        }
      }
    }
    ```

3. **Test in Claude Desktop**:
   - Restart Claude Desktop
   - Try commands like: `@raworc health_check`, `@raworc list_sessions`

### Test with Other MCP Clients

You can test with any MCP-compatible client by configuring it to use the `raworc-mcp` command with your auth token.

## Step 4: Debugging

### Enable Debug Logging

```bash
# Run with debug logging
raworc-mcp --auth-token your-token --log-level debug
```

### Check Network Connectivity

```bash
# Test if you can reach the Raworc API
curl -I http://raworc.remoteagent.com:9000/api/v0/health
```

### Common Issues

1. **Authentication Failed**:
   - Check your username and password
   - Ensure your account is active
   - Verify the API URL is correct

2. **Network Issues**:
   - Check your internet connection
   - Verify firewall settings
   - Try accessing the web UI first

3. **Token Expired**:
   - Get a new token using the auth script
   - Tokens typically expire after a certain time

4. **Permission Issues**:
   - Ensure your account has the necessary permissions
   - Check if you can access the spaces you're trying to use

## Step 5: Performance Testing

### Load Testing

```bash
# Test multiple concurrent requests
for i in {1..10}; do
  echo '{"jsonrpc": "2.0", "id": '$i', "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | raworc-mcp --auth-token your-token &
done
wait
```

### Memory and CPU Monitoring

```bash
# Monitor the MCP server process
top -p $(pgrep raworc-mcp)
```

## Step 6: Real-World Testing Scenarios

### Scenario 1: Session Management Workflow

1. Create a session
2. Send multiple messages
3. Get message history
4. Pause/resume the session
5. Terminate the session

### Scenario 2: Agent Monitoring

1. List agents in a space
2. Get agent logs
3. Monitor agent status changes

### Scenario 3: Secret Management

1. Set multiple secrets
2. Retrieve secrets
3. Update secret values
4. Delete secrets

## Troubleshooting

### Log Analysis

The MCP server provides detailed logs. Look for:
- Authentication success/failure messages
- API request/response details
- Error messages with status codes
- Network connection issues

### Error Codes

Common HTTP status codes:
- `200`: Success
- `401`: Unauthorized (invalid token)
- `403`: Forbidden (insufficient permissions)
- `404`: Not found
- `500`: Internal server error

### Getting Help

If you encounter issues:
1. Check the logs with `--log-level debug`
2. Verify your auth token is valid
3. Test API connectivity directly
4. Check the Raworc documentation
5. Open an issue on the GitHub repository

## Security Notes

- **Never commit auth tokens** to version control
- **Use environment variables** for sensitive data
- **Rotate tokens regularly** for production use
- **Monitor token usage** for security
- **Use HTTPS** for all API communications

## Next Steps

Once you've successfully tested the MCP server:

1. **Deploy to production** if needed
2. **Set up monitoring** for the MCP server
3. **Configure automated testing** in your CI/CD pipeline
4. **Document your specific use cases**
5. **Contribute improvements** to the project

Happy testing! 🚀
