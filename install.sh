#!/bin/bash

# Raworc MCP Server Installation Script
# This script builds and installs the Raworc MCP server

set -e

echo "🚀 Installing Raworc MCP Server..."

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust first:"
    echo "   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Build the release version
echo "📦 Building Raworc MCP Server..."
cargo build --release

# Create installation directory
INSTALL_DIR="/usr/local/bin"
if [ "$EUID" -ne 0 ]; then
    INSTALL_DIR="$HOME/.local/bin"
    mkdir -p "$INSTALL_DIR"
fi

# Copy the binary
echo "📋 Installing binary to $INSTALL_DIR..."
cp target/release/raworc-mcp "$INSTALL_DIR/"

# Make it executable
chmod +x "$INSTALL_DIR/raworc-mcp"

echo "✅ Raworc MCP Server installed successfully!"
echo ""
echo "📝 Usage:"
echo "   raworc-mcp --help"
echo ""
echo "🔧 Configuration:"
echo "   export RAWORC_AUTH_TOKEN=\"your-token\""
echo "   raworc-mcp --auth-token your-token"
echo ""
echo "📚 Documentation: README.md"
