#!/bin/bash

# Cross-compilation script for Raspberry Pi with OpenCV sysroot
# Build rs_raspi_camera for ARM64 architecture using Raspberry Pi libraries
# Usage: ./build_for_pi.sh [--real-camera]

set -e

echo "🛠️  Building rs_raspi_camera for Raspberry Pi with OpenCV sysroot..."

# Get the absolute path to the project directory
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SYSROOT_DIR="${PROJECT_DIR}/sysroot"

echo "📁 Project directory: ${PROJECT_DIR}"
echo "📁 Sysroot directory: ${SYSROOT_DIR}"

# Check if sysroot exists
if [ ! -d "${SYSROOT_DIR}/usr" ]; then
    echo "❌ Sysroot not found. Extracting from raspi-sysroot.tar.gz..."
    if [ -f "${PROJECT_DIR}/raspi-sysroot.tar.gz" ]; then
        mkdir -p "${SYSROOT_DIR}"
        tar -xzf "${PROJECT_DIR}/raspi-sysroot.tar.gz" -C "${SYSROOT_DIR}/"
        echo "✅ Sysroot extracted successfully"
    else
        echo "❌ raspi-sysroot.tar.gz not found. Please ensure it's in the project root."
        exit 1
    fi
fi

# Check for real camera flag
REAL_CAMERA_FLAG=""
if [ "$1" = "--real-camera" ]; then
    REAL_CAMERA_FLAG="--features real_camera"
    echo "📹 Building with real camera support (OpenCV from sysroot)"
else
    echo "🎭 Building with simulation mode only"
fi

# Check if the ARM64 target is installed
if ! rustup target list --installed | grep -q "aarch64-unknown-linux-gnu"; then
    echo "📦 Installing ARM64 target for Raspberry Pi..."
    rustup target add aarch64-unknown-linux-gnu
fi

# Set up cross-compilation environment
export CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
export CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
export AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar

# OpenCV cross-compilation environment variables
# Use only pkg-config method for better reliability
export PKG_CONFIG_ALLOW_CROSS=1
export PKG_CONFIG_PATH=""
export PKG_CONFIG_LIBDIR="${SYSROOT_DIR}/usr/lib/aarch64-linux-gnu/pkgconfig:${SYSROOT_DIR}/usr/share/pkgconfig"
export PKG_CONFIG_SYSROOT_DIR="${SYSROOT_DIR}"
export OPENCV_PKGCONFIG_NAME="opencv4"
export OPENCV_DISABLE_PROBES="cmake,vcpkg,environment"  # Use only pkg-config

# Force opencv-rust to accept the available OpenCV version (4.10.0)
export OPENCV_LINK_LIBS="opencv_core,opencv_imgproc,opencv_imgcodecs,opencv_videoio,opencv_highgui"
export OPENCV_LINK_PATHS="${SYSROOT_DIR}/usr/lib/aarch64-linux-gnu"
export OPENCV_INCLUDE_PATHS="${SYSROOT_DIR}/usr/include/opencv4"

# Test pkg-config before building
echo "🔍 Testing pkg-config for OpenCV..."
if command -v pkg-config >/dev/null 2>&1; then
    if PKG_CONFIG_LIBDIR="${PKG_CONFIG_LIBDIR}" PKG_CONFIG_SYSROOT_DIR="${PKG_CONFIG_SYSROOT_DIR}" pkg-config --exists opencv4; then
        echo "  ✅ pkg-config found OpenCV4"
        echo "  📁 Include paths: $(PKG_CONFIG_LIBDIR="${PKG_CONFIG_LIBDIR}" PKG_CONFIG_SYSROOT_DIR="${PKG_CONFIG_SYSROOT_DIR}" pkg-config --cflags opencv4)"
        echo "  🔗 Library paths: $(PKG_CONFIG_LIBDIR="${PKG_CONFIG_LIBDIR}" PKG_CONFIG_SYSROOT_DIR="${PKG_CONFIG_SYSROOT_DIR}" pkg-config --libs opencv4 | head -c 100)..."
    else
        echo "  ❌ pkg-config cannot find OpenCV4"
        echo "  🔍 Available packages: $(PKG_CONFIG_LIBDIR="${PKG_CONFIG_LIBDIR}" pkg-config --list-all | grep -i opencv || echo 'None')"
    fi
else
    echo "  ❌ pkg-config not found - installing..."
    if command -v brew >/dev/null 2>&1; then
        brew install pkg-config
    fi
fi

echo "🌍 Environment variables set:"
echo "  PKG_CONFIG_LIBDIR: ${PKG_CONFIG_LIBDIR}"
echo "  PKG_CONFIG_SYSROOT_DIR: ${PKG_CONFIG_SYSROOT_DIR}"
echo "  OPENCV_PKGCONFIG_NAME: ${OPENCV_PKGCONFIG_NAME}"

# Build for Raspberry Pi (ARM64 architecture)
echo "🔨 Cross-compiling for Raspberry Pi (aarch64-unknown-linux-gnu) with sysroot..."
echo "  Using sysroot: ${SYSROOT_DIR}"
echo "  OpenCV version: $(find "${SYSROOT_DIR}/usr/lib/aarch64-linux-gnu" -name "libopencv_core.so.*" | head -1 | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' || echo 'Unknown')"
echo ""

# Change to project directory
cd "${PROJECT_DIR}"

cargo build --release --target aarch64-unknown-linux-gnu $REAL_CAMERA_FLAG

# Check if build was successful
if [ $? -eq 0 ]; then
    echo "✅ Build successful with sysroot!"
    echo "📁 Binary location: target/aarch64-unknown-linux-gnu/release/rs_raspi_camera"
    echo "📁 Sysroot used: ${SYSROOT_DIR}"
    
    # Show binary size and dependencies info
    BINARY_PATH="target/aarch64-unknown-linux-gnu/release/rs_raspi_camera"
    if [ -f "${BINARY_PATH}" ]; then
        echo "📊 Binary size: $(ls -lh "${BINARY_PATH}" | awk '{print $5}')"
        echo "🔗 Checking OpenCV library dependencies..."
        if command -v aarch64-linux-gnu-objdump >/dev/null 2>&1; then
            echo "  Required OpenCV libraries:"
            aarch64-linux-gnu-objdump -p "${BINARY_PATH}" 2>/dev/null | grep -i opencv | head -5 || echo "  (OpenCV dependencies check requires aarch64-linux-gnu-objdump)"
        fi
    fi
    echo ""
    echo "📋 Deployment Instructions:"
    echo "1. Copy the binary to your Raspberry Pi:"
    echo "   scp target/aarch64-unknown-linux-gnu/release/rs_raspi_camera pi@<PI_IP>:~/"
    echo ""
    echo "2. On the Raspberry Pi, make it executable and run:"
    echo "   chmod +x rs_raspi_camera"
    echo "   ./rs_raspi_camera"
    echo ""
    echo "3. Access the web application:"
    echo "   Main page: http://<PI_IP>:3000"
    echo "   API test:  http://<PI_IP>:3000/api/hello"
    echo "   Status:    http://<PI_IP>:3000/status"
    echo ""
    echo "💡 Replace <PI_IP> with your Raspberry Pi's actual IP address"
    echo "📹 Note: Real camera mode requires OpenCV libraries on the Raspberry Pi"
else
    echo "❌ Build failed!"
    echo "💡 Troubleshooting:"
    echo "   - Ensure sysroot is properly extracted: ${SYSROOT_DIR}"
    echo "   - Check OpenCV libraries in sysroot: find ${SYSROOT_DIR} -name '*opencv*'"
    echo "   - Verify cross-compilation toolchain is installed:"
    echo "     brew install aarch64-unknown-linux-gnu (macOS)"
    echo "     sudo apt install gcc-aarch64-linux-gnu (Ubuntu/Debian)"
    exit 1
fi