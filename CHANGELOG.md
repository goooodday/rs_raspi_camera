# Changelog

All notable changes to rs_raspi_camera will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-09-14

### Added
- Initial release of rs_raspi_camera
- Real-time camera streaming functionality with MJPEG protocol
- Beautiful Korean "Hello World" interface with gradient design
- Live camera feed page with responsive design and Korean interface
- Camera stream endpoint (/api/camera/stream) for MJPEG video streaming
- Camera status API (/api/camera/status) with detailed camera information
- Simulated camera mode for development and testing
- Real-time clock display with Korean locale
- Responsive design optimized for mobile and desktop
- System status monitoring page with comprehensive information
- JSON API endpoint (`/api/hello`) with Korean message support
- Cross-compilation support for ARM64 (AArch64) architecture
- Automated build and deployment scripts
- Comprehensive documentation and setup guides

### Features
- **Web Framework**: Axum 0.8.4 with Tokio async runtime
- **Performance**: ~2-3MB memory usage, <1s startup time
- **Architecture**: Optimized for 64-bit Raspberry Pi models (3/4/5/Zero 2W)
- **Network Access**: Binds to 0.0.0.0:3000 for external connectivity
- **Korean Support**: Full Korean language interface
- **Real-time Updates**: Live clock and system information

### Technical Stack
- **Language**: Rust 2021 Edition
- **Web Framework**: Axum 0.8.4
- **Async Runtime**: Tokio with multi-thread support
- **Static Serving**: Tower-HTTP for file serving
- **Target Architecture**: aarch64-unknown-linux-gnu (ARM64)

### Deployment
- Cross-compilation from macOS/Linux to ARM64
- Automated deployment scripts with SSH integration
- systemd service configuration for auto-start
- Comprehensive build and deployment documentation

### API Endpoints
- `GET /` - Main Korean Hello World interface
- `GET /api/hello` - JSON API with Korean and English messages
- `GET /status` - System status and monitoring page

### Supported Devices
- Raspberry Pi 3 (64-bit OS)
- Raspberry Pi 4
- Raspberry Pi 5
- Raspberry Pi Zero 2 W