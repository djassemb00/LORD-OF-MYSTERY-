#!/bin/bash
# Build script for Veloren Android

set -e

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "========================================="
echo "  Veloren Android Build Script"
echo "========================================="
echo "  Project: $SCRIPT_DIR"
echo "========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Unset problematic LD_LIBRARY_PATH (from Termux/IDE)
unset LD_LIBRARY_PATH

# Source Rust environment
if [ -f "$HOME/.cargo/env" ]; then
    . "$HOME/.cargo/env"
fi

# Check Rust
if ! command -v rustc &> /dev/null; then
    echo -e "${RED}Error: Rust not installed${NC}"
    echo "Run: ./setup.sh"
    exit 1
fi

echo -e "${GREEN}✓${NC} Rust: $(rustc --version)"

# Set Android SDK/NDK paths
export ANDROID_HOME="/opt/android_sdk"
export ANDROID_NDK_HOME="/opt/android_sdk/ndk/25.2.9519653"

echo "  Android SDK: $ANDROID_HOME"
echo "  Android NDK: $ANDROID_NDK_HOME"
echo ""

# Add Android targets if not already added
echo "Checking Android targets..."
if ! rustup target list --installed | grep -q "aarch64-linux-android"; then
    echo "Adding aarch64-linux-android target..."
    rustup target add aarch64-linux-android
fi

if ! rustup target list --installed | grep -q "armv7-linux-androideabi"; then
    echo "Adding armv7-linux-androideabi target..."
    rustup target add armv7-linux-androideabi
fi

echo -e "${GREEN}✓${NC} Android targets installed"
echo ""

# Clean previous build
echo "Cleaning previous build..."
cd "$SCRIPT_DIR/app/src/main/rust"
cargo clean 2>/dev/null || true
cd "$SCRIPT_DIR"

# Build Rust library for Android
echo ""
echo "========================================="
echo "  Building Rust Library"
echo "========================================="

echo ""
echo -e "${YELLOW}Building for arm64-v8a...${NC}"
cd "$SCRIPT_DIR/app/src/main/rust"
cargo build --target aarch64-linux-android --release
echo -e "${GREEN}✓${NC} arm64-v8a build complete"

echo ""
echo -e "${YELLOW}Building for armeabi-v7a...${NC}"
cargo build --target armv7-linux-androideabi --release
echo -e "${GREEN}✓${NC} armeabi-v7a build complete"

cd "$SCRIPT_DIR"

# Copy built libraries to jniLibs
echo ""
echo "========================================="
echo "  Copying Native Libraries"
echo "========================================="

mkdir -p "$SCRIPT_DIR/app/src/main/jniLibs/arm64-v8a"
mkdir -p "$SCRIPT_DIR/app/src/main/jniLibs/armeabi-v7a"

cp "$SCRIPT_DIR/app/src/main/rust/target/aarch64-linux-android/release/libveloren_android.so" \
   "$SCRIPT_DIR/app/src/main/jniLibs/arm64-v8a/"
echo -e "${GREEN}✓${NC} Copied arm64-v8a library"

cp "$SCRIPT_DIR/app/src/main/rust/target/armv7-linux-androideabi/release/libveloren_android.so" \
   "$SCRIPT_DIR/app/src/main/jniLibs/armeabi-v7a/"
echo -e "${GREEN}✓${NC} Copied armeabi-v7a library"

# Build Android APK
echo ""
echo "========================================="
echo "  Building Android APK"
echo "========================================="

cd "$SCRIPT_DIR"
./gradlew assembleDebug

echo ""
echo "========================================="
echo -e "${GREEN}  Build Complete!${NC}"
echo "========================================="
echo ""
echo "APK location: app/build/outputs/apk/debug/app-debug.apk"
echo ""
echo "To install:"
echo "  adb install app/build/outputs/apk/debug/app-debug.apk"
echo ""
echo "To run:"
echo "  adb shell am start -n djb1.com.veloren/.GameActivity"
echo ""
echo "To view logs:"
echo "  adb logcat | grep -i veloren"
echo ""
