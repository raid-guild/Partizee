#!/usr/bin/env bash
set -e

echo "Building and installing partizee..."

# Debug information
echo "Detected OS type: $OSTYPE"

# Detect platform - improved Linux detection
if [[ "$OSTYPE" == "linux"* ]]; then
    PLATFORM="linux"
    BIN_DIR="$HOME/.local/bin"
    echo "Identified as Linux-based system"
elif [[ "$OSTYPE" == "darwin"* ]]; then
    PLATFORM="macos"
    BIN_DIR="$HOME/.local/bin"
elif [[ "$OSTYPE" == "msys" || "$OSTYPE" == "win32" ]]; then
    PLATFORM="windows"
    BIN_DIR="$HOME/.partizee/bin"
    EXE_EXT=".exe"
else
    echo "Unknown platform: $OSTYPE"
    echo "Assuming Linux-compatible..."
    PLATFORM="linux"
    BIN_DIR="$HOME/.local/bin"
fi

# Check for Java (OpenJDK 17)
JAVA_VERSION=$(java -version 2>&1 | awk -F[\".] '/version/ {print $2}')
if ! command -v java &> /dev/null; then
    echo "Java (OpenJDK 17) is required but not installed."
    echo "Please install OpenJDK 17: https://openjdk.org/install/"
    exit 1
elif [ "$JAVA_VERSION" != "17" ]; then
    echo "Java version 17 is required. Detected version: $JAVA_VERSION"
    echo "Please install OpenJDK 17: https://openjdk.org/install/"
    exit 1
else
    echo "Java (OpenJDK 17) is installed."
fi

#check if Git is installed
if ! command -v git &> /dev/null; then
    echo "Git is required but not installed."
    echo "Please install Git: https://git-scm.com/downloads"
    exit 1
else
    echo "Git is installed."
fi

#check if Rust is installed and if the version is 1.86
if ! command -v rustup &> /dev/null; then
    echo "Rust is required but not installed."
    echo "Please install Rust: https://rustup.rs/"
    exit 1
else
    echo "Rust is installed."
    if ! rustup show | grep -q "1.86"; then
        echo "Rust version is not 1.86."
        echo "Please downgrade to 1.86: rustup install 1.86"
        exit 1
    else
        echo "Rust version is 1.86."
    fi
fi

#check if wasm32-unknown-unknown is set for rust
if ! rustup target list | grep -q "wasm32-unknown-unknown"; then
    echo "wasm32-unknown-unknown is not set for Rust."
    echo "Please add the target: rustup target add wasm32-unknown-unknown"
    exit 1
else
    echo "wasm32-unknown-unknown is set for Rust."
fi

# Install cargo-partisia-contract if not already installed
if ! command -v cargo pbc &> /dev/null; then
    echo "Installing cargo-partisia-contract..."
    cargo install cargo-partisia-contract
else
    echo "cargo-partisia-contract already installed."
fi



echo "Building partizee..."
# Build
cargo build --release --manifest-path Cargo.toml

# Install partizee
mkdir -p "$BIN_DIR"
cp "target/release/partizee$EXE_EXT" "$BIN_DIR/partizee$EXE_EXT"
chmod +x "$BIN_DIR/partizee$EXE_EXT"

# Verify installation
if [ -x "$BIN_DIR/partizee$EXE_EXT" ]; then
  echo "Installation verified ✓"
else
  echo "Warning: Installation may have failed - check permissions"
fi

echo "Installed partizee to $BIN_DIR/"

# Immediate use instructions
echo "To use partizee immediately without restarting your terminal:"
echo "  export PATH=\"\$PATH:$BIN_DIR\""

# Path setup instructions
echo "For permanent installation, make sure $BIN_DIR is in your PATH"

# Add instructions for adding to PATH based on platform
if [[ "$PLATFORM" == "windows" ]]; then
    echo "Add to PATH in Windows by: setx PATH \"%PATH%;$BIN_DIR\""
elif [[ "$PLATFORM" == "macos" || "$PLATFORM" == "linux" ]]; then
    echo "Add to PATH by adding this to your profile (~/.bashrc, ~/.zshrc, etc.):"
    echo "export PATH=\"\$PATH:$BIN_DIR\""
fi

# Add completion message
echo "Run 'partizee --help' to get started" 