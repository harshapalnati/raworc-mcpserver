# Raworc MCP Server Usage Example

This example shows how to use the Raworc MCP server with Claude Desktop.

## Installation

### Quick Install with npx (Recommended)
```bash
# Install and run directly
npx @raworc/mcp-server --help

# Or install globally
npm install -g @raworc/mcp-server
raworc-mcp --help
```

### Manual Installation
```bash
# Clone and install
git clone https://github.com/yourusername/raworc-mcp.git
cd raworc-mcp
npm install  # This builds the Rust binary automatically
```

## Configuration

Add the following to your Claude Desktop configuration:

```json
{
  "mcpServers": {
    "raworc": {
      "command": "npx",
      "args": ["@raworc/mcp-server"],
      "env": {
        "RAWORC_API_URL": "http://raworc.remoteagent.com:9000/api/v0",
        "RAWORC_AUTH_TOKEN": "your-auth-token-here"
      }
    }
  }
}
```

## Usage Examples

Once configured, you can interact with Raworc through Claude:

### Check API Health
```
@raworc health_check
```

### List Sessions
```
@raworc list_sessions
```

### Create a New Session
```
@raworc create_session
```

### Send a Message to a Session
```
@raworc send_message
```

### List Spaces
```
@raworc list_spaces
```

### List Agents
```
@raworc list_agents
```

### Manage Secrets
```
@raworc list_secrets
@raworc set_secret
@raworc get_secret
```

## Command Line Usage

You can also run the MCP server directly:

```bash
# Basic usage
raworc-mcp

# With custom configuration
raworc-mcp --api-url http://raworc.remoteagent.com:9000/api/v0 \
           --auth-token your-token \
           --default-space production \
           --timeout 60 \
           --log-level debug
```

## Environment Variables

You can also configure the server using environment variables:

```bash
export RAWORC_API_URL="http://raworc.remoteagent.com:9000/api/v0"
export RAWORC_AUTH_TOKEN="your-auth-token"
export RAWORC_DEFAULT_SPACE="production"
export RAWORC_TIMEOUT="60"
export LOG_LEVEL="debug"

raworc-mcp
```
