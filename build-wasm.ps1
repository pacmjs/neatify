# Build script for generating WebAssembly bindings (PowerShell version)
# This script builds the Rust library as WebAssembly and moves it to the npm package

param(
    [switch]$Force
)

function Write-Info {
    param($Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warning {
    param($Message)
    Write-Host "[WARNING] $Message" -ForegroundColor Yellow
}

function Write-Error {
    param($Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
}

Write-Info "Building neatify WebAssembly bindings..."

# Check if wasm-pack is installed
try {
    wasm-pack --version | Out-Null
} catch {
    Write-Warning "wasm-pack not found. Please install it first:"
    Write-Host "  cargo install wasm-pack"
    exit 1
}

# Create npm/bin directory if it doesn't exist
if (-not (Test-Path "npm/bin")) {
    New-Item -ItemType Directory -Path "npm/bin" -Force
}

# Build the WebAssembly package
Write-Info "Building WebAssembly package..."
wasm-pack build --target nodejs --features wasm

# Create npm/bin directory if it doesn't exist
if (-not (Test-Path "npm/bin")) {
    New-Item -ItemType Directory -Path "npm/bin" -Force
}

# Move the generated files from pkg to npm/bin
Write-Info "Moving generated files..."
if (Test-Path "pkg") {
    # Copy files from pkg to npm/bin
    Copy-Item "pkg/neatify.js" "npm/bin/neatify_wasm.js" -Force
    Copy-Item "pkg/neatify_bg.wasm" "npm/bin/neatify_bg.wasm" -Force
    Copy-Item "pkg/neatify.d.ts" "npm/bin/neatify_wasm.d.ts" -Force
    
    # Clean up the pkg directory
    Remove-Item "pkg" -Recurse -Force
}

Write-Info "WebAssembly bindings built successfully!"
Write-Info "Generated files in npm/bin:"
if (Test-Path "npm/bin") {
    Get-ChildItem "npm/bin" -Name "neatify*"
}
