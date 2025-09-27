#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

// Skip building in CI/Smithery TypeScript deployments or when explicitly requested
const isCI = process.env.CI === 'true' || process.env.CI === '1';
const skipRust = !!process.env.SKIP_RUST_BUILD || process.env.SMITHERY_RUNTIME === 'typescript' || process.env.SMITHERY === 'true';

if (isCI || skipRust) {
  console.log('ℹ️  Skipping Rust build (CI/Smithery or SKIP_RUST_BUILD set).');
  process.exit(0);
}

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
      cwd: path.join(__dirname, '..')
    });
    cargo.on('close', (code) => {
      if (code === 0) {
        console.log('✅ Build completed successfully!');
        resolve();
      } else {
        reject(new Error(`Build failed with exit code ${code}`));
      }
    });
    cargo.on('error', (error) => reject(error));
  });
}

async function main() {
  try {
    // On Windows, proactively kill any running raworc-mcp.exe to avoid file lock (os error 5)
    if (process.platform === 'win32') {
      try {
        const killer = spawn('taskkill', ['/F', '/IM', 'raworc-mcp.exe'], { stdio: 'ignore' });
        killer.on('close', () => {});
      } catch (_) {}
    }

    const hasRust = await checkRust();
    if (!hasRust) {
      console.error('❌ Rust is not installed. Please install Rust first:');
      console.error('   Visit: https://rustup.rs/');
      process.exit(1);
    }
    await buildBinary();
    console.log('🎉 Raworc MCP Server is ready to use!');
    console.log('💡 Run: npx @raworc/mcp-server --help');
  } catch (error) {
    console.error('❌ Build failed:', error.message);
    process.exit(1);
  }
}

main();


