# Raworc MCP Server Installation Script for Windows
# This script builds and installs the Raworc MCP server

param(
    [string]$InstallPath = "$env:USERPROFILE\.local\bin"
)

Write-Host "🚀 Installing Raworc MCP Server..." -ForegroundColor Green

# Check if Rust is installed
try {
    $null = Get-Command cargo -ErrorAction Stop
} catch {
    Write-Host "❌ Rust is not installed. Please install Rust first:" -ForegroundColor Red
    Write-Host "   Visit: https://rustup.rs/" -ForegroundColor Yellow
    exit 1
}

# Build the release version
Write-Host "📦 Building Raworc MCP Server..." -ForegroundColor Green
cargo build --release

# Create installation directory
if (!(Test-Path $InstallPath)) {
    New-Item -ItemType Directory -Path $InstallPath -Force | Out-Null
}

# Copy the binary
Write-Host "📋 Installing binary to $InstallPath..." -ForegroundColor Green
Copy-Item "target\release\raworc-mcp.exe" "$InstallPath\"

# Add to PATH if not already there
$currentPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($currentPath -notlike "*$InstallPath*") {
    [Environment]::SetEnvironmentVariable("PATH", "$currentPath;$InstallPath", "User")
    Write-Host "✅ Added $InstallPath to PATH" -ForegroundColor Green
}

Write-Host "✅ Raworc MCP Server installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "📝 Usage:" -ForegroundColor Cyan
Write-Host "   raworc-mcp --help" -ForegroundColor White
Write-Host ""
Write-Host "🔧 Configuration:" -ForegroundColor Cyan
Write-Host "   `$env:RAWORC_AUTH_TOKEN = 'your-token'" -ForegroundColor White
Write-Host "   raworc-mcp --auth-token your-token" -ForegroundColor White
Write-Host ""
Write-Host "📚 Documentation: README.md" -ForegroundColor Cyan
Write-Host ""
Write-Host "🔄 Please restart your terminal to use the new PATH" -ForegroundColor Yellow
