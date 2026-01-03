#!/bin/bash
# BizHawk Development Environment Setup Script
#
# This script sets up the development environment for building the OotAutoTracker
# BizHawk plugin. It links your BizHawk installation to the build location.
#
# Usage: ./scripts/setup-bizhawk-dev.sh /path/to/BizHawk
#
# Requirements:
# - BizHawk 2.9+ installed
# - .NET SDK 6.0+ (for building the C# plugin)
# - Rust toolchain (for building oottracker.dll)

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
BIZHAWK_TARGET="$PROJECT_ROOT/crate/oottracker-bizhawk/OotAutoTracker/BizHawk"

# Color output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

info() { echo -e "${GREEN}[INFO]${NC} $1"; }
warn() { echo -e "${YELLOW}[WARN]${NC} $1"; }
error() { echo -e "${RED}[ERROR]${NC} $1"; exit 1; }

# Check arguments
if [ -z "$1" ]; then
    echo "Usage: $0 /path/to/BizHawk"
    echo ""
    echo "This script sets up the development environment for building the"
    echo "OotAutoTracker BizHawk plugin."
    echo ""
    echo "Arguments:"
    echo "  /path/to/BizHawk  Path to your BizHawk installation directory"
    echo "                    (the folder containing EmuHawk.exe)"
    echo ""
    echo "Example:"
    echo "  $0 /opt/BizHawk"
    echo "  $0 ~/Games/BizHawk-2.9.1"
    echo "  $0 'C:\\BizHawk'  # Windows path (use quotes)"
    exit 1
fi

BIZHAWK_PATH="$1"

# Validate BizHawk installation
info "Validating BizHawk installation at: $BIZHAWK_PATH"

if [ ! -d "$BIZHAWK_PATH" ]; then
    error "Directory not found: $BIZHAWK_PATH"
fi

# Check for key BizHawk files
if [ ! -f "$BIZHAWK_PATH/EmuHawk.exe" ] && [ ! -f "$BIZHAWK_PATH/EmuHawk" ]; then
    error "EmuHawk executable not found. Is this a valid BizHawk installation?"
fi

if [ ! -d "$BIZHAWK_PATH/dll" ]; then
    error "dll/ directory not found. Is this a valid BizHawk installation?"
fi

# Check for required DLLs
REQUIRED_DLLS=(
    "BizHawk.Client.Common.dll"
    "BizHawk.Common.dll"
    "BizHawk.Emulation.Common.dll"
)

for dll in "${REQUIRED_DLLS[@]}"; do
    if [ ! -f "$BIZHAWK_PATH/dll/$dll" ]; then
        error "Required DLL not found: dll/$dll"
    fi
done

info "BizHawk installation validated successfully"

# Create or update symlink
if [ -L "$BIZHAWK_TARGET" ]; then
    info "Removing existing symlink..."
    rm "$BIZHAWK_TARGET"
elif [ -d "$BIZHAWK_TARGET" ]; then
    warn "Existing directory found at target location"
    read -p "Remove existing directory and create symlink? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$BIZHAWK_TARGET"
    else
        error "Cannot proceed with existing directory"
    fi
fi

info "Creating symlink: $BIZHAWK_TARGET -> $BIZHAWK_PATH"
ln -s "$BIZHAWK_PATH" "$BIZHAWK_TARGET"

# Verify symlink
if [ ! -L "$BIZHAWK_TARGET" ]; then
    error "Failed to create symlink"
fi

info "Symlink created successfully"

# Check for .NET SDK
if command -v dotnet &> /dev/null; then
    DOTNET_VERSION=$(dotnet --version)
    info ".NET SDK found: $DOTNET_VERSION"
else
    warn ".NET SDK not found. You'll need it to build the C# plugin."
    warn "Install from: https://dotnet.microsoft.com/download"
fi

# Check for Rust
if command -v cargo &> /dev/null; then
    RUST_VERSION=$(cargo --version)
    info "Rust found: $RUST_VERSION"
else
    warn "Rust not found. You'll need it to build oottracker.dll"
    warn "Install from: https://rustup.rs"
fi

echo ""
info "Setup complete! You can now build the BizHawk plugin:"
echo ""
echo "  # Build the Rust FFI library"
echo "  cargo build --release -p oottracker-csharp"
echo ""
echo "  # Build the BizHawk plugin (includes C# build)"
echo "  cargo build --release -p oottracker-bizhawk"
echo ""
echo "  # Output files will be in:"
echo "  #   $BIZHAWK_TARGET/ExternalTools/OotAutoTracker.dll"
echo "  #   $BIZHAWK_TARGET/ExternalTools/oottracker.dll"
echo ""
