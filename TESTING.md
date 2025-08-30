
# Testing Guide

This guide covers testing the Raworc MCP Server to ensure it's working correctly in production.

## Prerequisites

1. **Rust Environment**: Version 1.70 or higher (for source builds)
2. **Node.js**: Version 16.0 or higher (for npx testing)
3. **Network Access**: Ability to reach `api.remoteagent.com`
4. **Authentication**: Valid Raworc credentials

## Quick Test

### Option 1: Test with npx (Recommended)

```bash
# Test the installation
npx @raworc/mcp-server --help

# Test health check
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | npx @raworc/mcp-server

# Test list spaces
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "list_spaces", "arguments": {}}}' | npx @raworc/mcp-server
```

### Option 2: Test from Source

```bash
# Build the project
cargo build --release

# Test health check
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | ./target/release/raworc-mcp

# Test list spaces
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "list_spaces", "arguments": {}}}' | ./target/release/raworc-mcp
```

## Step-by-Step Testing

### Step 1: Get Authentication Token

```bash
curl -X POST https://api.remoteagent.com/api/v0/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "user": "your-username",
    "pass": "your-password"
  }'
```

### Step 2: Set Environment Variables

```bash
export RAWORC_AUTH_TOKEN="your-jwt-token"
export RAWORC_API_URL="https://api.remoteagent.com/api/v0"
```

### Step 3: Test Basic Functionality

```bash
# Test health check
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | npx @raworc/mcp-server

# Test get version
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "get_version", "arguments": {}}}' | npx @raworc/mcp-server

# Test list spaces
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "list_spaces", "arguments": {}}}' | npx @raworc/mcp-server
```

## Manual API Testing

Before testing the MCP server, verify the API endpoints directly:

```bash
# Health check
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/health

# Get version
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/version

# List spaces
curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/spaces
```

## MCP Protocol Testing

### Test Individual Tools

```bash
# Health check
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | npx @raworc/mcp-server

# List spaces
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/call", "params": {"name": "list_spaces", "arguments": {}}}' | npx @raworc/mcp-server

# Get version
echo '{"jsonrpc": "2.0", "id": 3, "method": "tools/call", "params": {"name": "get_version", "arguments": {}}}' | npx @raworc/mcp-server

# List sessions
echo '{"jsonrpc": "2.0", "id": 4, "method": "tools/call", "params": {"name": "list_sessions", "arguments": {"space": "default"}}}' | npx @raworc/mcp-server
```

### Test MCP Handshake

```bash
# Initialize
echo '{"jsonrpc": "2.0", "id": 1, "method": "initialize", "params": {"protocolVersion": "2024-11-05", "capabilities": {}, "clientInfo": {"name": "test-client", "version": "1.0.0"}}}' | npx @raworc/mcp-server

# List tools
echo '{"jsonrpc": "2.0", "id": 2, "method": "tools/list", "params": {}}' | npx @raworc/mcp-server
```

## Integration Testing

### Test with Claude Desktop

1. **Configure Claude Desktop**:
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

2. **Test Commands**:
   ```
   @raworc health_check
   @raworc list_spaces
   @raworc get_version
   @raworc list_sessions
   ```

## Automated Testing

### Run Test Suite

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_health_check

# Run with output
cargo test -- --nocapture
```

### Integration Tests

```bash
# Run integration tests
cargo test --test integration_test
```

## Debugging

### Enable Debug Logging

```bash
export LOG_LEVEL="debug"
export RAWORC_API_URL="https://api.remoteagent.com/api/v0"
export RAWORC_AUTH_TOKEN="your-token"
npx @raworc/mcp-server
```

### Common Issues

1. **Authentication Errors**
   - Verify token is valid and not expired
   - Check username/password credentials
   - Ensure proper permissions

2. **Network Errors**
   - Verify connectivity to `api.remoteagent.com`
   - Check firewall settings
   - Try accessing web UI first

3. **MCP Protocol Errors**
   - Check JSON-RPC message format
   - Verify tool names match exactly
   - Ensure proper error handling

### Troubleshooting Steps

1. **Check API Connectivity**:
   ```bash
   curl -I https://api.remoteagent.com/api/v0/health
   ```

2. **Verify Authentication**:
   ```bash
   curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/auth/me
   ```

3. **Test Individual Endpoints**:
   ```bash
   curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/spaces
   curl -H "Authorization: Bearer your-token" https://api.remoteagent.com/api/v0/sessions
   ```

## Performance Testing

### Load Testing

```bash
# Test multiple concurrent requests
for i in {1..10}; do
  echo '{"jsonrpc": "2.0", "id": '$i', "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | npx @raworc/mcp-server &
done
wait
```

### Memory Usage

```bash
# Monitor memory usage (for source builds)
valgrind --tool=massif ./target/release/raworc-mcp
```

## Security Testing

### Token Validation

```bash
# Test with invalid token
export RAWORC_AUTH_TOKEN="invalid-token"
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "health_check", "arguments": {}}}' | npx @raworc/mcp-server
```

### Input Validation

```bash
# Test with malformed JSON
echo '{"invalid": "json"' | npx @raworc/mcp-server

# Test with missing required fields
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/call", "params": {"name": "get_session", "arguments": {}}}' | npx @raworc/mcp-server
```

## Continuous Integration

### GitHub Actions Example

```yaml
name: Test MCP Server

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      - run: cargo test
      - run: cargo build --release
      - name: Test npx installation
        run: |
          npm install
          npx @raworc/mcp-server --help
```

## Reporting Issues

When reporting issues, include:

1. **Environment**: OS, Rust version, Node.js version, Raworc version
2. **Configuration**: API URL, authentication method
3. **Error Messages**: Full error output and logs
4. **Steps to Reproduce**: Detailed reproduction steps
5. **Expected vs Actual**: What you expected vs what happened

## Support

- **Documentation**: [Raworc API Docs](https://raworc.com/docs/api/rest-api)
- **Issues**: [GitHub Issues](https://github.com/harshapalnati/raworc-mcpserver/issues)
- **Discussions**: [GitHub Discussions](https://github.com/harshapalnati/raworc-mcpserver/discussions)
