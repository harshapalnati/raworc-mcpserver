#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Get the binary path based on platform and architecture
function getBinaryPath() {
  const platform = process.platform;
  const arch = process.arch;
  
  // Determine binary name
  let binaryName = 'raworc-mcp';
  if (platform === 'win32') {
    binaryName += '.exe';
  }
  
  // Check if we're in development (local build)
  const localPath = path.join(__dirname, '..', 'target', 'release', binaryName);
  if (fs.existsSync(localPath)) {
    return localPath;
  }
  
  // Check if we're in npm package
  const packagePath = path.join(__dirname, '..', 'target', 'release', binaryName);
  if (fs.existsSync(packagePath)) {
    return packagePath;
  }
  
  throw new Error(`Raworc MCP binary not found. Please run 'cargo build --release' first.`);
}

// Main function
function main() {
  try {
    const binaryPath = getBinaryPath();
    
    // Spawn the Rust binary with all arguments
    const child = spawn(binaryPath, process.argv.slice(2), {
      stdio: 'inherit',
      cwd: process.cwd()
    });
    
    // Handle process exit
    child.on('close', (code) => {
      process.exit(code);
    });
    
    // Handle process errors
    child.on('error', (error) => {
      console.error('Failed to start Raworc MCP server:', error.message);
      process.exit(1);
    });
    
  } catch (error) {
    console.error('Error:', error.message);
    process.exit(1);
  }
}

// Run if this file is executed directly
if (require.main === module) {
  main();
}
