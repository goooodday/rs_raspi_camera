# Dynamic System Information Feature

## 🎯 Purpose

The system status information has been enhanced to dynamically detect and display the actual compilation environment instead of hardcoded "Raspberry Pi" information. This makes the application more informative and accurate when running on different platforms.

## 🔄 Changes Made

### 1. New System Information Module (`src/system_info.rs`)

Created a dedicated module for system detection with:

- **Dynamic Platform Detection**: Automatically detects the target platform based on compilation environment
- **Cross-compilation Awareness**: Shows whether the binary was cross-compiled or natively compiled
- **Platform-specific Information**: Displays appropriate platform names, icons, and network information
- **Extensible Design**: Easy to add support for new platforms

### 2. Enhanced Web Module (`src/web/mod.rs`)

Updated the system status page to dynamically generate content:

- **Dynamic HTML Generation**: System status page now returns `Html<String>` instead of static HTML
- **Platform-specific UI**: Shows different titles, icons, and information based on detected platform
- **Visual Indicators**: Color-coded badges for cross-compilation vs native compilation
- **Network Information**: Shows appropriate network access information based on platform

### 3. New API Endpoint (`src/api/mod.rs`)

Added a new API endpoint for system information:

- **Endpoint**: `/api/system-info`
- **Response**: JSON with platform, architecture, OS, and target information
- **Usage**: Used by camera page to display platform information

### 4. Updated Routing (`src/routes.rs`)

Added the new API endpoint to the routing configuration.

## 🖥️ Platform Detection

The system automatically detects and displays information for:

| Architecture | OS      | Platform Name    | Icon | Network Access     |
|--------------|---------|------------------|------|-------------------|
| aarch64      | linux   | 라즈베리파이     | 🍇   | 0.0.0.0:3000      |
| arm          | linux   | 라즈베리파이     | 🍇   | 0.0.0.0:3000      |
| x86_64       | linux   | Linux PC         | 🐧   | 127.0.0.1:3000    |
| x86_64       | macos   | Mac              | 🍎   | 127.0.0.1:3000    |
| aarch64      | macos   | Mac              | 🍎   | 127.0.0.1:3000    |
| x86_64       | windows | Windows PC       | 🪟   | 127.0.0.1:3000    |
| i686         | windows | Windows PC       | 🪟   | 127.0.0.1:3000    |
| Other        | Other   | Unknown Platform | 💻   | 127.0.0.1:3000    |

## 🎨 Visual Enhancements

### System Status Page
- **Dynamic Titles**: Page title changes based on detected platform
- **Color-coded Badges**: Orange for cross-compiled, green for native compilation
- **Platform-specific Sections**: Different information sections based on platform
- **Real-time Information**: Shows actual compilation target and runtime environment

### Camera Page
- **Platform Information**: Added platform information section in system info card
- **Live Updates**: Platform information is fetched via API when page loads
- **Error Handling**: Graceful fallback when platform information is unavailable

## 🔧 Technical Implementation

### Environment Variables Used
- `CARGO_CFG_TARGET_ARCH`: Target architecture
- `CARGO_CFG_TARGET_OS`: Target operating system
- `TARGET`: Full target triple

### Cross-compilation Detection
The system compares the compilation target with the runtime environment to determine if cross-compilation was used.

### Fallback Mechanisms
When environment variables are not available, the system falls back to Rust's built-in constants.

## ✅ Benefits

1. **Accurate Information**: Shows real platform information instead of hardcoded values
2. **Cross-platform Compatibility**: Works correctly on any platform
3. **Developer Experience**: Clear indication of compilation mode
4. **User Experience**: More informative system status pages
5. **Maintainability**: Centralized system detection logic
6. **Extensibility**: Easy to add support for new platforms

## 🧪 Testing

The feature has been tested with:
- ✅ Native compilation on macOS
- ✅ Cross-compilation for ARM64 (Raspberry Pi)
- ✅ Cross-compilation for x86_64 Linux
- ✅ All compilation modes compile successfully

## 🚀 Usage

The dynamic system information is automatically displayed when:
1. Visiting the system status page (`/status`)
2. Viewing the camera page (`/camera`)
3. Calling the API endpoint (`/api/system-info`)

No additional configuration is required - the system automatically detects the environment at compile time.