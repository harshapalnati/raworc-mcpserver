# Usage Examples

This document provides practical examples of how to use the Raworc MCP Server.

## Basic Setup

### Environment Configuration

```bash
# Set your authentication token
export RAWORC_AUTH_TOKEN="your-jwt-token"

# Set the API URL (optional, has default)
export RAWORC_API_URL="https://api.remoteagent.com/api/v0"

# Set default space (optional)
export RAWORC_DEFAULT_SPACE="your-space"

# Set timeout (optional, default: 30 seconds)
export RAWORC_TIMEOUT="30"
```

### Claude Desktop Configuration

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

## Session Management

### Create a Session

```bash
# Using npx
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "create_session", "arguments": {"space": "default", "metadata": {"purpose": "testing"}}}}' | npx @raworc/mcp-server

# In Claude Desktop
@raworc create_session
```

### List Sessions

```bash
# List all sessions in default space
@raworc list_sessions

# List sessions in specific space
@raworc list_sessions --space production
```

### Send Messages

```bash
# Send a message to a session
@raworc send_message --session_id "session-uuid" --content "Hello, agent!"

# Get messages from a session
@raworc get_messages --session_id "session-uuid" --limit 10
```

### Session Control

```bash
# Pause a session
@raworc pause_session --session_id "session-uuid"

# Resume a session
@raworc resume_session --session_id "session-uuid"

# Terminate a session
@raworc terminate_session --session_id "session-uuid"
```

## Space Management

### List Spaces

```bash
# List all available spaces
@raworc list_spaces
```

### Create and Manage Spaces

```bash
# Create a new space (if API supports it)
curl -X POST https://api.remoteagent.com/api/v0/spaces \
  -H "Authorization: Bearer your-token" \
  -H "Content-Type: application/json" \
  -d '{"name": "new-space", "description": "My new space"}'
```

## Agent Management

### List Agents

```bash
# List agents in default space
@raworc list_agents

# List agents in specific space
@raworc list_agents --space production
```

### Get Agent Logs

```bash
# Get logs for a specific agent
@raworc get_agent_logs --space "default" --agent_name "my-agent"
```

## Secret Management

### Set Secrets

```bash
# Set a secret
@raworc set_secret --space "default" --key "api-key" --value "secret-value"

# Set multiple secrets
@raworc set_secret --space "default" --key "database-url" --value "postgresql://..."
@raworc set_secret --space "default" --key "redis-password" --value "redis-secret"
```

### Retrieve Secrets

```bash
# Get a specific secret
@raworc get_secret --space "default" --key "api-key"

# List all secrets in a space
@raworc list_secrets --space "default"
```

### Delete Secrets

```bash
# Delete a secret
@raworc delete_secret --space "default" --key "old-api-key"
```

## System Information

### Health Check

```bash
# Check API health
@raworc health_check
```

### Get Version

```bash
# Get API version information
@raworc get_version
```

## Advanced Examples

### Complete Workflow

Here's a complete example of creating a session, sending messages, and managing the workflow:

```bash
# 1. Create a new session
SESSION_ID=$(echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "create_session", "arguments": {"space": "default", "metadata": {"purpose": "automation"}}}}' | npx @raworc/mcp-server | jq -r '.result.content[0].text' | jq -r '.id')

# 2. Send a message to the session
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "send_message", "arguments": {"session_id": "'$SESSION_ID'", "content": "Hello, I need help with a task."}}}' | npx @raworc/mcp-server

# 3. Get messages from the session
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_messages", "arguments": {"session_id": "'$SESSION_ID'", "limit": 10}}}' | npx @raworc/mcp-server

# 4. Terminate the session when done
echo '{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "terminate_session", "arguments": {"session_id": "'$SESSION_ID'"}}}' | npx @raworc/mcp-server
```

### Error Handling

```bash
# Test with invalid session ID
@raworc get_session --session_id "invalid-uuid"

# Test with missing required parameters
@raworc send_message --content "This should fail"
```

### Batch Operations

```bash
# Create multiple sessions
for i in {1..5}; do
  echo "Creating session $i..."
  @raworc create_session --space "batch-test" --metadata "{\"batch_id\": $i}"
done

# List all sessions in the batch
@raworc list_sessions --space "batch-test"
```

## Integration Examples

### With Python Scripts

```python
import subprocess
import json

def call_raworc_tool(tool_name, arguments):
    """Call a Raworc MCP tool"""
    message = {
        "jsonrpc": "2.0",
        "id": 1,
        "method": "tools/call",
        "params": {
            "name": tool_name,
            "arguments": arguments
        }
    }
    
    result = subprocess.run(
        ["npx", "@raworc/mcp-server"],
        input=json.dumps(message),
        text=True,
        capture_output=True,
        env={
            "RAWORC_AUTH_TOKEN": "your-token",
            "RAWORC_API_URL": "https://api.remoteagent.com/api/v0"
        }
    )
    
    return json.loads(result.stdout)

# Example usage
spaces = call_raworc_tool("list_spaces", {})
print(f"Found {len(spaces['result']['content'])} spaces")
```

### With Shell Scripts

```bash
#!/bin/bash

# Configuration
RAWORC_TOKEN="your-token"
RAWORC_API_URL="https://api.remoteagent.com/api/v0"

# Function to call Raworc MCP
call_raworc() {
    local tool_name=$1
    local arguments=$2
    
    echo "{\"jsonrpc\": \"2.0\", \"id\": 1, \"method\": \"tools/call\", \"params\": {\"name\": \"$tool_name\", \"arguments\": $arguments}}" | \
    RAWORC_AUTH_TOKEN="$RAWORC_TOKEN" \
    RAWORC_API_URL="$RAWORC_API_URL" \
    npx @raworc/mcp-server
}

# Example: Monitor sessions
echo "Checking session health..."
sessions=$(call_raworc "list_sessions" "{}")
echo "Current sessions: $sessions"
```

## Troubleshooting

### Common Issues

1. **Authentication Errors**
   - Verify your token is valid and not expired
   - Check that you have the correct permissions

2. **Network Issues**
   - Ensure you can reach `api.remoteagent.com`
   - Check firewall and proxy settings

3. **Space Issues**
   - Verify the space exists and you have access
   - Check space permissions

### Debug Mode

Enable debug logging for troubleshooting:

```bash
export LOG_LEVEL="debug"
export RAWORC_API_URL="https://api.remoteagent.com/api/v0"
export RAWORC_AUTH_TOKEN="your-token"
npx @raworc/mcp-server
```

## Best Practices

1. **Token Management**
   - Store tokens securely (environment variables, not in code)
   - Rotate tokens regularly
   - Use different tokens for different environments

2. **Error Handling**
   - Always check for errors in responses
   - Implement retry logic for transient failures
   - Log errors for debugging

3. **Resource Management**
   - Clean up sessions when done
   - Monitor resource usage
   - Use appropriate timeouts

4. **Security**
   - Never log sensitive data
   - Use HTTPS for all communications
   - Validate all inputs

For more detailed information, see the [Raworc API Documentation](https://raworc.com/docs/api/rest-api).
