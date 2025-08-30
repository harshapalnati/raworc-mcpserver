#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

console.log('🚀 Building Raworc MCP Server...');

// Check if Rust is available
function checkRust() {
  return new Promise((resolve) => {
    const rustc = spawn('rustc', ['--version'], { stdio: 'ignore' });
    rustc.on('close', (code) => {
      resolve(code === 0);
    });
  });
}

// Build the Rust binary
function buildBinary() {
  return new Promise((resolve, reject) => {
    console.log('📦 Running cargo build --release...');
    
    const cargo = spawn('cargo', ['build', '--release'], {
      stdio: 'inherit',
      cwd: __dirname + '/..'
    });
    
    cargo.on('close', (code) => {
      if (code === 0) {
        console.log('✅ Build completed successfully!');
        resolve();
      } else {
        reject(new Error(`Build failed with exit code ${code}`));
      }
    });
    
    cargo.on('error', (error) => {
      reject(error);
    });
  });
}

// Main function
async function main() {
  try {
    // Check if Rust is installed
    const hasRust = await checkRust();
    if (!hasRust) {
      console.error('❌ Rust is not installed. Please install Rust first:');
      console.error('   Visit: https://rustup.rs/');
      process.exit(1);
    }
    
    // Build the binary
    await buildBinary();
    
    console.log('🎉 Raworc MCP Server is ready to use!');
    console.log('💡 Run: npx @raworc/mcp-server --help');
    
  } catch (error) {
    console.error('❌ Build failed:', error.message);
    process.exit(1);
  }
}

// Run the script
main();
