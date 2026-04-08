#!/bin/bash
# Setup script for Veloren Android
# Installs all required dependencies

set -e

echo "=== Veloren Android Setup ==="

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source "$HOME/.cargo/env"
else
    echo "Rust already installed"
fi

# Add Android targets
echo "Adding Android Rust targets..."
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi

# Check if Android SDK/NDK are installed
if [ -z "$ANDROID_HOME" ]; then
    echo "WARNING: ANDROID_HOME not set"
    echo "Please set ANDROID_HOME to your Android SDK path"
    echo "Example: export ANDROID_HOME=\$HOME/Android/Sdk"
fi

if [ -z "$ANDROID_NDK_HOME" ]; then
    echo "WARNING: ANDROID_NDK_HOME not set"
    echo "Please set ANDROID_NDK_HOME to your Android NDK path"
    echo "Example: export ANDROID_NDK_HOME=\$HOME/Android/Sdk/ndk/26.0.10792818"
fi

# Install cargo-ndk (optional but recommended)
if ! command -v cargo-ndk &> /dev/null; then
    echo "Installing cargo-ndk..."
    cargo install cargo-ndk
fi

# Make build script executable
chmod +x build.sh

echo ""
echo "=== Setup Complete ==="
echo ""
echo "Next steps:"
echo "1. Set ANDROID_HOME and ANDROID_NDK_HOME environment variables"
echo "2. Run: ./build.sh"
echo "3. Install APK: adb install app/build/outputs/apk/debug/app-debug.apk"
echo ""
echo "For more info, see README.md"
