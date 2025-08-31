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

- **Complete API Coverage**: Full access to all Raworc REST API endpoints
- **Service Account Management**: Create, update, and manage service accounts with role-based access
- **Role-Based Access Control (RBAC)**: Manage roles and role bindings for fine-grained permissions
- **Space Management**: Create, update, and manage isolated workspaces
- **Session Management**: Create, pause, resume, terminate, and fork sessions
- **Message Handling**: Send, retrieve, and manage session messages
- **Agent Operations**: Deploy, monitor, control, and manage agents with full lifecycle support
- **Secret Management**: Secure storage and retrieval of secrets with proper access controls
- **Build Management**: Trigger and monitor space builds for agent deployment
- **Real-time Communication**: Send messages and receive responses from agents
- **Health Monitoring**: Check API health and version information
- **Production Ready**: Robust error handling, logging, and MCP protocol compliance

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

The MCP server provides comprehensive access to all Raworc API endpoints. Here are the available tools organized by category:

### System Information

#### `health_check`
Check Raworc API health status.

```json
{
  "name": "health_check",
  "arguments": {}
}
```

#### `get_version`
Get Raworc API version information.

```json
{
  "name": "get_version",
  "arguments": {}
}
```

### Service Account Management

#### `list_service_accounts`
List all service accounts.

```json
{
  "name": "list_service_accounts",
  "arguments": {}
}
```

#### `create_service_account`
Create a new service account.

```json
{
  "name": "create_service_account",
  "arguments": {
    "user": "api-user",
    "pass": "secure-password",
    "space": "production",
    "description": "API access user"
  }
}
```

#### `get_service_account`
Get a specific service account by ID.

```json
{
  "name": "get_service_account",
  "arguments": {
    "id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

#### `update_service_account`
Update a service account.

```json
{
  "name": "update_service_account",
  "arguments": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "space": "production",
    "description": "Updated description",
    "active": true
  }
}
```

#### `delete_service_account`
Delete a service account.

```json
{
  "name": "delete_service_account",
  "arguments": {
    "id": "550e8400-e29b-41d4-a716-446655440000"
  }
}
```

#### `update_service_account_password`
Update service account password.

```json
{
  "name": "update_service_account_password",
  "arguments": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "current_password": "old-password",
    "new_password": "new-secure-password"
  }
}
```

### Role Management

#### `list_roles`
List all RBAC roles.

```json
{
  "name": "list_roles",
  "arguments": {}
}
```

#### `create_role`
Create a new role.

```json
{
  "name": "create_role",
  "arguments": {
    "id": "developer",
    "description": "Developer role",
    "rules": [
      {
        "apiGroups": [""],
        "resources": ["sessions", "messages"],
        "verbs": ["get", "list", "create"]
      }
    ]
  }
}
```

#### `get_role`
Get a specific role by ID.

```json
{
  "name": "get_role",
  "arguments": {
    "id": "developer"
  }
}
```

#### `delete_role`
Delete a role.

```json
{
  "name": "delete_role",
  "arguments": {
    "id": "developer"
  }
}
```

### Role Binding Management

#### `list_role_bindings`
List all role bindings.

```json
{
  "name": "list_role_bindings",
  "arguments": {}
}
```

#### `create_role_binding`
Create a new role binding.

```json
{
  "name": "create_role_binding",
  "arguments": {
    "subject": "api-user",
    "role_ref": "developer",
    "space": "staging"
  }
}
```

#### `get_role_binding`
Get a specific role binding by ID.

```json
{
  "name": "get_role_binding",
  "arguments": {
    "id": "admin-binding"
  }
}
```

#### `delete_role_binding`
Delete a role binding.

```json
{
  "name": "delete_role_binding",
  "arguments": {
    "id": "admin-binding"
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

#### `create_space`
Create a new space.

```json
{
  "name": "create_space",
  "arguments": {
    "name": "staging",
    "description": "Staging environment",
    "settings": {
      "environment": "staging"
    }
  }
}
```

#### `get_space`
Get a specific space by name.

```json
{
  "name": "get_space",
  "arguments": {
    "name": "staging"
  }
}
```

#### `update_space`
Update a space.

```json
{
  "name": "update_space",
  "arguments": {
    "name": "staging",
    "description": "Updated staging space"
  }
}
```

#### `delete_space`
Delete a space.

```json
{
  "name": "delete_space",
  "arguments": {
    "name": "staging"
  }
}
```

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
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "production"
  }
}
```

#### `update_session`
Update session details.

```json
{
  "name": "update_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "production"
  }
}
```

#### `update_session_state`
Update session state.

```json
{
  "name": "update_session_state",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "state": "closed"
  }
}
```

#### `close_session`
Close a session.

```json
{
  "name": "close_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d"
  }
}
```

#### `restore_session`
Restore a closed session.

```json
{
  "name": "restore_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d"
  }
}
```

#### `remix_session`
Fork a session.

```json
{
  "name": "remix_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "development"
  }
}
```

#### `pause_session`
Pause a session.

```json
{
  "name": "pause_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "production"
  }
}
```

#### `resume_session`
Resume a paused session.

```json
{
  "name": "resume_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "production"
  }
}
```

#### `terminate_session`
Terminate a session.

```json
{
  "name": "terminate_session",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "production"
  }
}
```

### Session Message Management

#### `get_messages`
Get messages from a session.

```json
{
  "name": "get_messages",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "production",
    "limit": 10
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
    "space": "production",
    "content": "Generate a Python script to calculate fibonacci numbers"
  }
}
```

#### `get_message_count`
Get message count for a session.

```json
{
  "name": "get_message_count",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "production"
  }
}
```

#### `clear_messages`
Clear all messages from a session.

```json
{
  "name": "clear_messages",
  "arguments": {
    "session_id": "61549530-3095-4cbf-b379-cd32416f626d",
    "space": "production"
  }
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

#### `create_agent`
Create a new agent.

```json
{
  "name": "create_agent",
  "arguments": {
    "space": "production",
    "name": "data-analyzer",
    "description": "Data analysis specialist",
    "purpose": "analyze data, create visualizations, statistical analysis",
    "source_repo": "Raworc/raworc-agent-python-demo",
    "source_branch": "main"
  }
}
```

#### `get_agent`
Get a specific agent.

```json
{
  "name": "get_agent",
  "arguments": {
    "space": "production",
    "agent_name": "data-analyzer"
  }
}
```

#### `update_agent`
Update an agent.

```json
{
  "name": "update_agent",
  "arguments": {
    "space": "production",
    "agent_name": "data-analyzer",
    "description": "Updated data analysis specialist",
    "purpose": "enhanced data analysis and visualization"
  }
}
```

#### `delete_agent`
Delete an agent.

```json
{
  "name": "delete_agent",
  "arguments": {
    "space": "production",
    "agent_name": "data-analyzer"
  }
}
```

#### `update_agent_status`
Update agent status.

```json
{
  "name": "update_agent_status",
  "arguments": {
    "space": "production",
    "agent_name": "data-analyzer",
    "status": "inactive"
  }
}
```

#### `deploy_agent`
Deploy an agent.

```json
{
  "name": "deploy_agent",
  "arguments": {
    "space": "production",
    "agent_name": "data-analyzer"
  }
}
```

#### `stop_agent`
Stop an agent.

```json
{
  "name": "stop_agent",
  "arguments": {
    "space": "production",
    "agent_name": "data-analyzer"
  }
}
```

#### `list_running_agents`
List running agents in a space.

```json
{
  "name": "list_running_agents",
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
    "agent_name": "data-analyzer",
    "limit": 100,
    "follow": false
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
    "space": "production",
    "show_values": false
  }
}
```

#### `create_secret`
Create a new secret.

```json
{
  "name": "create_secret",
  "arguments": {
    "space": "production",
    "key_name": "ANTHROPIC_API_KEY",
    "value": "sk-ant-your-actual-key",
    "description": "Claude API key"
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
    "key": "ANTHROPIC_API_KEY",
    "show_values": true
  }
}
```

#### `update_secret`
Update a secret.

```json
{
  "name": "update_secret",
  "arguments": {
    "space": "production",
    "key": "ANTHROPIC_API_KEY",
    "value": "new-secret-value",
    "description": "Updated description"
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
    "key": "ANTHROPIC_API_KEY"
  }
}
```

### Build Management

#### `create_build`
Trigger a space build.

```json
{
  "name": "create_build",
  "arguments": {
    "space": "production",
    "dockerfile": "Dockerfile"
  }
}
```

#### `get_latest_build`
Get latest build status.

```json
{
  "name": "get_latest_build",
  "arguments": {
    "space": "production"
  }
}
```

#### `get_build`
Get specific build status.

```json
{
  "name": "get_build",
  "arguments": {
    "space": "production",
    "build_id": "build-550e8400-e29b-41d4-a716-446655440000"
  }
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

### v0.2.0

- **Complete API Coverage**: Added all Raworc REST API endpoints
- **Service Account Management**: Full CRUD operations for service accounts
- **Role-Based Access Control**: Complete RBAC implementation with roles and role bindings
- **Enhanced Session Management**: Added session forking, state management, and message operations
- **Advanced Agent Operations**: Full agent lifecycle management with deployment and monitoring
- **Comprehensive Secret Management**: Enhanced secret operations with proper access controls
- **Build Management**: Space build triggering and monitoring capabilities
- **Production Enhancements**: Improved error handling, logging, and MCP protocol compliance

### v0.1.0

- Initial release
- Basic MCP protocol implementation
- Core Raworc API integration
- Session, space, agent, and secret management
- Health monitoring and version information
- npx installation support
- Production-ready error handling

## 🔗 Links

- **Repository**: [https://github.com/harshapalnati/raworc-mcpserver](https://github.com/harshapalnati/raworc-mcpserver)
- **npm Package**: [@raworc/mcp-server](https://www.npmjs.com/package/@raworc/mcp-server)
- **Raworc API**: [https://raworc.com/docs/api/rest-api](https://raworc.com/docs/api/rest-api)
