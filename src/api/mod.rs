use crate::camera::CameraState;
use async_stream;
use axum::{
    extract::State,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
};
use chrono;
use serde_json::json;
use std::env;

/// Simple API endpoint returning JSON
pub async fn hello_api() -> &'static str {
    r#"{
  "message": "Hello from Raspberry Pi!",
  "status": "success",
  "server": "Rust Axum",
  "timestamp": "2024-01-01T00:00:00Z",
  "korean_message": "라즈베리파이에서 안녕하세요!",
  "project": "rs_raspi_camera"
}"#
}

/// Camera stream endpoint (MJPEG)
pub async fn camera_stream(State(camera_state): State<CameraState>) -> impl IntoResponse {
    let mut frame_receiver = camera_state.frame_sender.subscribe();

    let stream = async_stream::stream! {
        // MJPEG boundary
        let boundary = "--boundary123456789";

        // Send initial headers
        yield Ok::<_, Box<dyn std::error::Error + Send + Sync>>(
            format!("{}\r\n", boundary).into_bytes()
        );

        while let Ok(frame_data) = frame_receiver.recv().await {
            // MJPEG frame headers
            let headers = format!(
                "Content-Type: image/jpeg\r\nContent-Length: {}\r\n\r\n",
                frame_data.len()
            );

            // Send headers
            yield Ok(headers.into_bytes());

            // Send frame data
            yield Ok(frame_data.to_vec());

            // Send boundary
            yield Ok(format!("\r\n{}\r\n", boundary).into_bytes());
        }
    };

    Response::builder()
        .status(StatusCode::OK)
        .header(
            header::CONTENT_TYPE,
            "multipart/x-mixed-replace; boundary=boundary123456",
        )
        .header(header::CACHE_CONTROL, "no-cache")
        .header(header::CONNECTION, "keep-alive")
        .body(axum::body::Body::from_stream(stream))
        .unwrap()
}

/// Camera status API endpoint
pub async fn camera_status(State(camera_state): State<CameraState>) -> impl IntoResponse {
    let is_streaming = {
        let streaming = camera_state.is_streaming.lock().unwrap();
        *streaming
    };

    let camera_mode = {
        let mode = camera_state.camera_mode.lock().unwrap();
        mode.clone()
    };

    let (backend, mode_desc) = match camera_mode.as_str() {
        "OpenCV" => ("OpenCV Camera", "Real Camera"),
        "Simulation" => ("Simulated Camera", "Development/Testing"),
        "Error" => ("Error", "Camera Unavailable"),
        _ => ("Unknown", "Initializing"),
    };

    let status = json!({
        "status": "success",
        "camera": {
            "streaming": is_streaming,
            "resolution": "640x480",
            "fps": "30",
            "format": "MJPEG",
            "backend": backend,
            "mode": mode_desc,
            "camera_mode": camera_mode
        },
        "server": "Rust Axum",
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "project": "rs_raspi_camera"
    });

    (StatusCode::OK, axum::Json(status))
}

/// System information API endpoint
pub async fn system_info() -> impl IntoResponse {
    // Detect system information
    let target_arch =
        env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_else(|_| std::env::consts::ARCH.to_string());

    let target_os =
        env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| std::env::consts::OS.to_string());

    let full_target = env::var("TARGET").unwrap_or_else(|_| {
        format!(
            "{}-unknown-{}-gnu",
            std::env::consts::ARCH,
            std::env::consts::OS
        )
    });

    // Determine platform information based on target architecture and OS
    let (platform_name, platform_icon) = match (target_arch.as_str(), target_os.as_str()) {
        ("aarch64", "linux") => ("라즈베리파이", "🍇"),
        ("arm", "linux") => ("라즈베리파이", "🍇"),
        ("x86_64", "linux") => ("Linux PC", "🐧"),
        ("x86_64", "macos") => ("Mac", "🍎"),
        ("aarch64", "macos") => ("Mac", "🍎"),
        ("x86_64", "windows") => ("Windows PC", "🪟"),
        ("i686", "windows") => ("Windows PC", "🪟"),
        _ => ("Unknown Platform", "💻"),
    };

    let system_info = json!({
        "status": "success",
        "platform": platform_name,
        "icon": platform_icon,
        "architecture": target_arch,
        "os": target_os,
        "target": full_target,
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "project": "rs_raspi_camera"
    });

    (StatusCode::OK, axum::Json(system_info))
}
