# 🍓 rs_raspi_camera

> A beautiful, lightweight Rust camera web application optimized for 64-bit Raspberry Pi devices with Korean language support.

<div align="center">

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Raspberry Pi](https://img.shields.io/badge/-RaspberryPi-C51A4A?style=for-the-badge&logo=Raspberry-Pi)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust Version](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Architecture](https://img.shields.io/badge/arch-ARM64-blue.svg)](https://en.wikipedia.org/wiki/ARM64)

</div>

## ✨ Features

- 🎨 **Beautiful Korean Interface** - Stunning gradient design with real-time clock
- 📹 **Real-time Camera Streaming** - Live video feed from Raspberry Pi camera module
- 🚀 **High Performance** - Built with Rust and Axum for maximum efficiency  
- 📱 **Responsive Design** - Perfect on mobile and desktop devices
- 🌐 **Network Accessible** - Binds to 0.0.0.0:3000 for external access
- 🔧 **Easy Deployment** - Cross-compile from macOS/Linux or build directly on Pi
- 📊 **System Monitoring** - Real-time system status and camera information
- 🛡️ **ARM64 Optimized** - Native performance on modern Raspberry Pi models
- 💾 **Minimal Resources** - Only ~2-3MB memory usage
- 📺 **MJPEG Streaming** - Browser-compatible video streaming protocol
- 🖥️ **Dynamic Platform Detection** - Automatically detects and displays platform-specific information

## 🎯 Supported Devices

This application is optimized for 64-bit ARM (AArch64) architecture:

- **Raspberry Pi 3** (with 64-bit OS)
- **Raspberry Pi 4** 
- **Raspberry Pi 5**
- **Raspberry Pi Zero 2 W**

## 🎥 Camera Modes

### ✅ Simulation Mode (Recommended)
- **Status**: Fully functional and cross-compilation ready
- **Description**: Generated test patterns with animations and dynamic gradients
- **Use Cases**: Development, testing, and demonstrations
- **Build**: `./scripts/build_for_pi.sh`
- **Advantages**: Always works, no hardware dependencies, perfect for development

### ⚠️ Real Camera Mode (Advanced)
- **Status**: Cross-compilation challenges with OpenCV Rust bindings
- **Description**: Uses actual Raspberry Pi camera module or USB camera
- **Current Limitation**: opencv-rust crate has trait implementation issues during cross-compilation
- **Alternative Solutions**:
  1. **Native Compilation**: Build directly on Raspberry Pi
  2. **Future Updates**: Waiting for opencv-rust cross-compilation fixes (see [Future Improvements](#future-improvements))

### 🛠️ Sysroot Configuration (Prepared)
- **OpenCV Version**: 4.10.0 libraries extracted and configured
- **Location**: `./sysroot/` with complete ARM64 environment
- **Status**: Ready for when OpenCV cross-compilation is resolved
- **Setup**: Run `./scripts/setup_sysroot.sh` to verify configuration

#### Obtaining Raspberry Pi Libraries
The sysroot directory contains the necessary Raspberry Pi libraries for cross-compilation and is not included in the GitHub repository due to its large size. To set up the sysroot:

1. **Create the sysroot directory structure**:
   ```bash
   mkdir -p sysroot/usr
   ```

2. **Copy required libraries from your Raspberry Pi**:
   ```bash
   # On your Raspberry Pi, copy the following directories to your development machine:
   # /usr/include → sysroot/usr/include
   # /usr/lib/aarch64-linux-gnu → sysroot/usr/lib/aarch64-linux-gnu
   # /lib/aarch64-linux-gnu → sysroot/lib/aarch64-linux-gnu
   
   # You can use rsync for this:
   rsync -avz pi@<PI_IP>:/usr/include/ sysroot/usr/include/
   rsync -avz pi@<PI_IP>:/usr/lib/aarch64-linux-gnu/ sysroot/usr/lib/aarch64-linux-gnu/
   rsync -avz pi@<PI_IP>:/lib/aarch64-linux-gnu/ sysroot/lib/aarch64-linux-gnu/
   ```

3. **Verify the setup**:
   ```bash
   ./scripts/setup_sysroot.sh
   ```

### 🔄 Recommended Development Workflow
1. **Development**: Use simulation mode for rapid development and testing
2. **Deployment**: Deploy simulation mode to Raspberry Pi for immediate functionality
3. **Real Camera**: Build natively on Raspberry Pi when actual camera is needed

## 🚀 Quick Start

### Option 1: Cross-Compile for Raspberry Pi (Recommended)

```bash
# 1. Clone the repository
git clone https://github.com/USERNAME/rs_raspi_camera.git
cd rs_raspi_camera

# 2. Build simulation mode (always works)
./scripts/build_for_pi.sh

# 3. Deploy to Raspberry Pi
scp target/aarch64-unknown-linux-gnu/release/rs_raspi_camera pi@<PI_IP>:~/

# 4. Run on Raspberry Pi
ssh pi@<PI_IP>
chmod +x rs_raspi_camera
./rs_raspi_camera
```

### Option 2: Native Build on Raspberry Pi (For Real Camera)

```bash
# 1. On Raspberry Pi - Install dependencies
sudo apt update
sudo apt install -y curl build-essential libopencv-dev pkg-config

# 2. Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 3. Clone and build with real camera support
git clone https://github.com/USERNAME/rs_raspi_camera.git
cd rs_raspi_camera
cargo build --release --features real_camera

# 4. Run
./target/release/rs_raspi_camera
```

## 🌐 Access Your Application

Once running, access your web application at:

- **Main Page**: `http://<PI_IP>:3000`
- **Camera Stream**: `http://<PI_IP>:3000/camera`
- **API Test**: `http://<PI_IP>:3000/api/hello`  
- **System Status**: `http://<PI_IP>:3000/status`

## 🏗️ Architecture

The application has been modularized into separate, focused modules:

```
src/
├── main.rs              # Entry point and application orchestration
├── camera/              # Camera processing module
│   └── mod.rs          # Camera capture, OpenCV integration, simulation
├── web/                 # Web page handlers module  
│   └── mod.rs          # HTML pages (home, camera, status)
├── api/                 # API endpoints module
│   └── mod.rs          # JSON API, camera stream, camera status
└── routes.rs           # Axum routing configuration
```

## 🔧 Tech Stack

- **Language**: Rust 2021 Edition
- **Web Framework**: [Axum](https://github.com/tokio-rs/axum) 0.8.4
- **Async Runtime**: [Tokio](https://tokio.rs/)
- **HTTP Server**: [Hyper](https://hyper.rs/)
- **Static File Serving**: Tower-HTTP
- **Target Architecture**: AArch64 (ARM64)

## 📊 Performance

- **Memory Usage**: ~2-3MB
- **CPU Usage**: Near 0% when idle
- **Startup Time**: <1 second
- **Binary Size**: ~2.3MB (optimized)
- **Network Throughput**: Excellent for IoT applications

## 🔍 API Reference

### GET `/`
Main "Hello World" page with Korean interface and real-time features

### GET `/camera`
Real-time camera streaming page with Korean interface and live video feed

### GET `/api/camera/stream`
MJPEG video stream endpoint for real-time camera feed

### GET `/api/camera/status`
```json
{
  "status": "success",
  "camera": {
    "streaming": true,
    "resolution": "640x480",
    "fps": "30",
    "format": "MJPEG",
    "backend": "Simulated Camera",
    "mode": "Development/Testing"
  },
  "server": "Rust Axum",
  "project": "rs_raspi_camera"
}
```

### GET `/api/hello`
```json
{
  "message": "Hello from Raspberry Pi!",
  "status": "success",
  "server": "Rust Axum",
  "korean_message": "라즈베리파이에서 안녕하세요!",
  "project": "rs_raspi_camera"
}
```

### GET `/status`
Comprehensive system status page with monitoring information

## ⚙️ Configuration

### Change Port
Edit `src/main.rs`:
```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 8080)); // Change from 3000 to 8080
```

### Auto-Start on Boot
```bash
sudo tee /etc/systemd/system/rs_raspi_camera.service > /dev/null <<EOF
[Unit]
Description=Raspberry Pi Camera Web Server
After=network.target

[Service]
Type=simple
User=pi
WorkingDirectory=/home/pi
ExecStart=/home/pi/rs_raspi_camera
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
EOF

sudo systemctl enable rs_raspi_camera.service
sudo systemctl start rs_raspi_camera.service
```

## 📚 Documentation

- **[FINAL_PROJECT_SUMMARY.md](file:///Users/hyeongabshin/Git_Room/Rust_src/rs_raspi_camera/FINAL_PROJECT_SUMMARY.md)** - Complete project summary
- **[DYNAMIC_SYSTEM_INFO.md](file:///Users/hyeongabshin/Git_Room/Rust_src/rs_raspi_camera/DYNAMIC_SYSTEM_INFO.md)** - Dynamic platform detection feature
- **[CLEANUP_SUMMARY.md](file:///Users/hyeongabshin/Git_Room/Rust_src/rs_raspi_camera/CLEANUP_SUMMARY.md)** - Project cleanup documentation

## 🔮 Future Improvements

### Cross-Compilation Enhancements
Cross-compilation support for real camera mode will be enhanced in future updates:
- Improved OpenCV cross-compilation with the opencv-rust crate
- Enhanced sysroot configuration and linking solutions
- Streamlined build process for ARM64 targets
- Better integration with Raspberry Pi camera modules

### Additional Features
- Configuration file support for easy customization
- Extended API endpoints for advanced camera control
- Additional camera backends for different hardware
- Enhanced error handling and logging
- Additional unit tests for comprehensive coverage

## 📃 License

This project is licensed under the MIT License - see the [LICENSE](file:///Users/hyeongabshin/Git_Room/Rust_src/rs_raspi_camera/LICENSE) file for details.

## 🙏 Acknowledgments

- Thanks to the Rust community for the excellent ecosystem
- Special thanks to the Axum and Tokio teams for their outstanding frameworks
- Inspired by various Raspberry Pi and computer vision projects