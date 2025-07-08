# Release script for neatify (PowerShell version)
# Usage: .\release.ps1 [major|minor|patch]

param(
    [ValidateSet("major", "minor", "patch")]
    [string]$BumpType = "patch"
)

# Function to write colored output
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

# Check if we're in a git repository
try {
    git rev-parse --git-dir | Out-Null
} catch {
    Write-Error "Not in a git repository"
    exit 1
}

# Check if working directory is clean
$gitStatus = git status --porcelain
if ($gitStatus) {
    Write-Error "Working directory is not clean. Please commit or stash changes."
    exit 1
}

# Check if we're on main branch
$currentBranch = git rev-parse --abbrev-ref HEAD
if ($currentBranch -ne "main") {
    Write-Error "Not on main branch. Currently on: $currentBranch"
    exit 1
}

# Pull latest changes
Write-Info "Pulling latest changes..."
git pull origin main

# Check if cargo and jq are available
try {
    cargo --version | Out-Null
} catch {
    Write-Error "Cargo not found. Please install Rust and Cargo."
    exit 1
}

$cargoToml = Get-Content "Cargo.toml"
$inPackageSection = $false
$versionLine = $null

foreach ($line in $cargoToml) {
    if ($line -match '^\[package\]') {
        $inPackageSection = $true
        continue
    }
    elseif ($line -match '^\[.*\]') {
        $inPackageSection = $false
        continue
    }
    
    if ($inPackageSection -and $line -match '^version\s*=\s*"([^"]+)"') {
        $versionLine = $line
        break
    }
}

if (-not $versionLine) {
    Write-Error "Could not find version in [package] section of Cargo.toml"
    exit 1
}

$currentVersion = $matches[1]
Write-Info "Current version: $currentVersion"

# Parse version parts
$versionParts = $currentVersion.Split('.')
$major = [int]$versionParts[0]
$minor = [int]$versionParts[1]
$patch = [int]$versionParts[2]

# Determine new version based on argument
switch ($BumpType) {
    "major" {
        $newMajor = $major + 1
        $newMinor = 0
        $newPatch = 0
    }
    "minor" {
        $newMajor = $major
        $newMinor = $minor + 1
        $newPatch = 0
    }
    "patch" {
        $newMajor = $major
        $newMinor = $minor
        $newPatch = $patch + 1
    }
}

$newVersion = "$newMajor.$newMinor.$newPatch"
Write-Info "New version will be: $newVersion"

# Confirm with user
$confirmation = Read-Host "Continue with release $newVersion? (y/N)"
if ($confirmation -notmatch '^[Yy]$') {
    Write-Info "Release cancelled."
    exit 0
}

# Run tests before proceeding
Write-Info "Running tests..."
cargo test --verbose
if ($LASTEXITCODE -ne 0) {
    Write-Error "Tests failed"
    exit 1
}

# Run quality checks
Write-Info "Running quality checks..."
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) {
    Write-Error "Code formatting check failed"
    exit 1
}

# Build WebAssembly bindings
Write-Info "Building WebAssembly bindings..."
if (-not (Get-Command "wasm-pack" -ErrorAction SilentlyContinue)) {
    Write-Info "Installing wasm-pack..."
    cargo install wasm-pack
}

wasm-pack build --target nodejs --features wasm --out-dir npm/bin
if ($LASTEXITCODE -ne 0) {
    Write-Error "WebAssembly build failed"
    exit 1
}

# Build npm package
Write-Info "Building npm package..."
Set-Location npm
npm run build
if ($LASTEXITCODE -ne 0) {
    Write-Error "npm build failed"
    exit 1
}
Set-Location ..

# Update version in Cargo.toml [package] section
Write-Info "Updating Cargo.toml package version..."
$cargoTomlContent = Get-Content "Cargo.toml"
$updatedLines = @()
$inPackageSection = $false

foreach ($line in $cargoTomlContent) {
    if ($line -match '^\[package\]') {
        $inPackageSection = $true
        $updatedLines += $line
    }
    elseif ($line -match '^\[.*\]') {
        $inPackageSection = $false
        $updatedLines += $line
    }
    elseif ($inPackageSection -and $line -match '^version\s*=\s*"[^"]+"') {
        $updatedLines += "version = `"$newVersion`""
    }
    else {
        $updatedLines += $line
    }
}

Set-Content "Cargo.toml" $updatedLines

# Update Cargo.lock
Write-Info "Updating Cargo.lock..."
cargo check

# Commit version bump
Write-Info "Committing version bump..."
git add Cargo.toml Cargo.lock
git commit -m "Bump version to $newVersion"

# Create and push tag
Write-Info "Creating and pushing tag v$newVersion..."
git tag "v$newVersion"
git push origin main
git push origin "v$newVersion"

Write-Info "Release process initiated!"
Write-Info "Check GitHub Actions for the publishing status: https://github.com/pacmjs/neatify/actions"
Write-Warning "Make sure the CRATES_KEY repository secret is properly configured."
