#!/bin/bash

# Build script for generating WebAssembly bindings
# This script builds the Rust library as WebAssembly and moves it to the npm package

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_info "Building neatify WebAssembly bindings..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    print_warning "wasm-pack not found. Installing..."
    curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
fi

# Build the WebAssembly package
print_info "Building WebAssembly package..."
wasm-pack build --target nodejs --features wasm

# Create npm/bin directory if it doesn't exist
mkdir -p npm/bin

# Move the generated files to the correct location
print_info "Moving generated files..."
if [ -d "pkg" ]; then
    # Copy files from pkg to npm/bin with renamed files
    if [ -f "pkg/neatify.js" ]; then
        mv pkg/neatify.js npm/bin/neatify_wasm.js
    fi

    if [ -f "pkg/neatify_bg.wasm" ]; then
        mv pkg/neatify_bg.wasm npm/bin/neatify_bg.wasm
    fi

    if [ -f "pkg/neatify.d.ts" ]; then
        mv pkg/neatify.d.ts npm/bin/neatify_wasm.d.ts
    fi
    
    # Clean up the pkg directory
    rm -rf pkg
fi

print_info "WebAssembly bindings built successfully!"
print_info "Generated files in npm/bin:"
if [ -d "npm/bin" ]; then
    ls -la npm/bin/neatify*
fi
