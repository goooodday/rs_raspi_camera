#!/bin/bash

# Setup script for Raspberry Pi sysroot with OpenCV libraries
# This script verifies the Raspberry Pi sysroot for cross-compilation
# Note: The sysroot directory is not included in the repository due to its large size.
# You need to obtain the required libraries from your Raspberry Pi.

set -e

echo "🔧 Setting up Raspberry Pi sysroot with OpenCV libraries..."

# Get the absolute path to the project directory
PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SYSROOT_DIR="${PROJECT_DIR}/sysroot"

echo "📁 Project directory: ${PROJECT_DIR}"
echo "📁 Sysroot directory: ${SYSROOT_DIR}"

# Check if sysroot exists
if [ ! -d "${SYSROOT_DIR}/usr" ]; then
    echo "❌ Sysroot not found. Please set up the sysroot first."
    echo ""
    echo "💡 To set up the sysroot, you need to copy the required directories from your Raspberry Pi:"
    echo "   1. Create the sysroot directory structure:"
    echo "      mkdir -p sysroot/usr sysroot/lib"
    echo ""
    echo "   2. Copy required directories from your Raspberry Pi:"
    echo "      rsync -avz pi@<PI_IP>:/usr/include/ sysroot/usr/include/"
    echo "      rsync -avz pi@<PI_IP>:/usr/lib/aarch64-linux-gnu/ sysroot/usr/lib/aarch64-linux-gnu/"
    echo "      rsync -avz pi@<PI_IP>:/lib/aarch64-linux-gnu/ sysroot/lib/aarch64-linux-gnu/"
    echo ""
    echo "   3. Run this script again to verify the setup."
    echo ""
    exit 1
fi

echo "✅ Sysroot directory found"

# Verify required directories exist
REQUIRED_DIRS=(
    "${SYSROOT_DIR}/usr/include"
    "${SYSROOT_DIR}/usr/lib/aarch64-linux-gnu"
    "${SYSROOT_DIR}/lib/aarch64-linux-gnu"
)

for dir in "${REQUIRED_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then
        echo "❌ Required directory not found: $dir"
        echo "💡 Please ensure all required directories are copied from your Raspberry Pi."
        exit 1
    fi
done

echo "✅ All required sysroot directories found"

# Verify OpenCV installation
echo "🔍 Verifying OpenCV installation in sysroot..."

# Check for OpenCV headers
OPENCV_HEADERS=$(find "${SYSROOT_DIR}/usr/include" -name "opencv2" -type d 2>/dev/null | wc -l)
if [ "${OPENCV_HEADERS}" -gt 0 ]; then
    echo "✅ OpenCV headers found"
    find "${SYSROOT_DIR}/usr/include" -name "opencv2" -type d | head -3
else
    echo "⚠️  OpenCV headers not found"
    echo "💡 OpenCV headers are required for real camera support."
    echo "💡 You can still build in simulation mode: ./scripts/build_for_pi.sh"
fi

# Check for OpenCV libraries
OPENCV_LIBS=$(find "${SYSROOT_DIR}/usr/lib" -name "libopencv_*.so*" 2>/dev/null | wc -l)
if [ "${OPENCV_LIBS}" -gt 0 ]; then
    echo "✅ OpenCV libraries found (${OPENCV_LIBS} files)"
    echo "📚 Core OpenCV libraries:"
    find "${SYSROOT_DIR}/usr/lib" -name "libopencv_core.so*" | head -3
    find "${SYSROOT_DIR}/usr/lib" -name "libopencv_imgproc.so*" | head -3
    find "${SYSROOT_DIR}/usr/lib" -name "libopencv_videoio.so*" | head -3
    
    # Detect OpenCV version
    OPENCV_VERSION=$(find "${SYSROOT_DIR}/usr/lib" -name "libopencv_core.so.*" | head -1 | grep -o '[0-9]\+\.[0-9]\+\.[0-9]\+' || echo 'Unknown')
    echo "📊 OpenCV version: ${OPENCV_VERSION}"
else
    echo "❌ OpenCV libraries not found"
    echo "💡 OpenCV libraries are required for real camera support."
    echo "💡 You can still build in simulation mode: ./scripts/build_for_pi.sh"
    
    echo "🔍 Searching for any OpenCV files..."
    find "${SYSROOT_DIR}" -name "*opencv*" | head -10
fi

# Check for pkg-config files
PKG_CONFIG_FILES=$(find "${SYSROOT_DIR}/usr" -name "opencv*.pc" 2>/dev/null | wc -l)
if [ "${PKG_CONFIG_FILES}" -gt 0 ]; then
    echo "✅ OpenCV pkg-config files found"
    find "${SYSROOT_DIR}/usr" -name "opencv*.pc" | head -3
else
    echo "⚠️  OpenCV pkg-config files not found"
fi

# Summary
echo ""
echo "📋 Sysroot setup summary:"
echo "   📁 Sysroot location: ${SYSROOT_DIR}"
echo "   📚 OpenCV headers: $([ "${OPENCV_HEADERS}" -gt 0 ] && echo "✅ Available" || echo "❌ Missing")"
echo "   🔗 OpenCV libraries: $([ "${OPENCV_LIBS}" -gt 0 ] && echo "✅ Available (${OPENCV_LIBS} files)" || echo "❌ Missing")"
echo "   📦 OpenCV version: ${OPENCV_VERSION:-'Unknown'}"
echo ""

if [ "${OPENCV_LIBS}" -gt 0 ] && [ "${OPENCV_HEADERS}" -gt 0 ]; then
    echo "🎉 Sysroot setup completed successfully!"
    echo "💡 You can now build with real camera support:"
    echo "   ./scripts/build_for_pi.sh --real-camera"
else
    echo "⚠️  Sysroot setup completed with warnings"
    echo "💡 OpenCV may not be fully available for cross-compilation"
    echo "   You can still build in simulation mode:"
    echo "   ./scripts/build_for_pi.sh"
fi