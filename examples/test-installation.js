#!/usr/bin/env node

const { spawn } = require('child_process');

console.log('🧪 Testing Raworc MCP Server Installation...\n');

// Test 1: Check if npx can run the server
console.log('1️⃣ Testing npx installation...');
const npxTest = spawn('npx', ['@raworc/mcp-server', '--help'], {
  stdio: ['pipe', 'pipe', 'pipe'],
  timeout: 30000
});

let npxOutput = '';
let npxError = '';

npxTest.stdout.on('data', (data) => {
  npxOutput += data.toString();
});

npxTest.stderr.on('data', (data) => {
  npxError += data.toString();
});

npxTest.on('close', (code) => {
  if (code === 0) {
    console.log('✅ npx installation works!');
    console.log('📋 Help output:');
    console.log(npxOutput.split('\n').slice(0, 10).join('\n'));
    console.log('...\n');
  } else {
    console.log('❌ npx installation failed');
    console.log('Error:', npxError);
    console.log('\n💡 Try running: npm install -g @raworc/mcp-server\n');
  }
  
  // Test 2: Check if global installation works (if available)
  console.log('2️⃣ Testing global installation...');
  const globalTest = spawn('raworc-mcp', ['--help'], {
    stdio: ['pipe', 'pipe', 'pipe'],
    timeout: 10000
  });
  
  globalTest.on('close', (code) => {
    if (code === 0) {
      console.log('✅ Global installation works!');
    } else {
      console.log('⚠️  Global installation not found (this is okay if you only use npx)');
      console.log('💡 To install globally: npm install -g @raworc/mcp-server\n');
    }
    
    console.log('🎉 Installation test completed!');
    console.log('\n📝 Next steps:');
    console.log('1. Get your auth token: curl -X POST http://raworc.remoteagent.com:9000/api/v0/auth/login -H "Content-Type: application/json" -d \'{"user": "your-username", "pass": "your-password"}\'');
    console.log('2. Test with token: npx @raworc/mcp-server --auth-token YOUR_TOKEN --log-level debug');
    console.log('3. Configure Claude Desktop: See QUICKSTART.md for details');
    console.log('\n📚 Documentation: README.md, QUICKSTART.md');
  });
});

npxTest.on('error', (error) => {
  console.log('❌ Failed to run npx test:', error.message);
  console.log('💡 Make sure Node.js is installed and npx is available');
});
