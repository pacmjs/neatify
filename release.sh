#!/bin/bash

# Release script for neatify
# Usage: ./release.sh [major|minor|patch]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    print_error "Not in a git repository"
    exit 1
fi

# Check if working directory is clean
if ! git diff-index --quiet HEAD --; then
    print_error "Working directory is not clean. Please commit or stash changes."
    exit 1
fi

# Check if we're on main branch
current_branch=$(git rev-parse --abbrev-ref HEAD)
if [ "$current_branch" != "main" ]; then
    print_error "Not on main branch. Currently on: $current_branch"
    exit 1
fi

# Pull latest changes
print_info "Pulling latest changes..."
git pull origin main

# Get current version from Cargo.toml
current_version=$(cargo metadata --format-version 1 --no-deps | jq -r '.packages[0].version')
print_info "Current version: $current_version"

# Parse version parts
IFS='.' read -ra VERSION_PARTS <<< "$current_version"
major=${VERSION_PARTS[0]}
minor=${VERSION_PARTS[1]}
patch=${VERSION_PARTS[2]}

# Determine new version based on argument
case ${1:-patch} in
    major)
        new_major=$((major + 1))
        new_minor=0
        new_patch=0
        ;;
    minor)
        new_major=$major
        new_minor=$((minor + 1))
        new_patch=0
        ;;
    patch)
        new_major=$major
        new_minor=$minor
        new_patch=$((patch + 1))
        ;;
    *)
        print_error "Invalid version bump type: $1. Use major, minor, or patch."
        exit 1
        ;;
esac

new_version="$new_major.$new_minor.$new_patch"
print_info "New version will be: $new_version"

# Confirm with user
read -p "Continue with release $new_version? (y/N): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    print_info "Release cancelled."
    exit 0
fi

# Run tests before proceeding
print_info "Running tests..."
cargo test --verbose

# Run quality checks
print_info "Running quality checks..."
cargo fmt --all -- --check

# Update version in Cargo.toml
print_info "Updating Cargo.toml version..."
sed -i.bak "s/^version = \".*\"/version = \"$new_version\"/" Cargo.toml
rm Cargo.toml.bak 2>/dev/null || true  # Remove backup file (macOS creates .bak files)

# Update Cargo.lock
print_info "Updating Cargo.lock..."
cargo check

# Commit version bump
print_info "Committing version bump..."
git add Cargo.toml Cargo.lock
git commit -m "Bump version to $new_version"

# Create and push tag
print_info "Creating and pushing tag v$new_version..."
git tag "v$new_version"
git push origin main
git push origin "v$new_version"

print_info "Release process initiated!"
print_info "Check GitHub Actions for the publishing status: https://github.com/pacmjs/neatify/actions"
print_warning "Make sure the CRATES_KEY repository secret is properly configured."
