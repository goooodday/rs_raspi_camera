use bytes::Bytes;
use chrono;
use image::{ImageBuffer, Rgb, RgbImage};
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{sync::broadcast, time};

#[cfg(feature = "real_camera")]
use opencv::{
    core::{Mat, Vector},
    imgcodecs,
    prelude::*,
    videoio::{VideoCapture, CAP_ANY, CAP_V4L2},
};

/// Camera capture state shared across the application
#[derive(Clone)]
pub struct CameraState {
    pub frame_sender: broadcast::Sender<Bytes>,
    pub is_streaming: Arc<Mutex<bool>>,
    pub camera_mode: Arc<Mutex<String>>, // "OpenCV", "Simulation", "Error"
}

impl CameraState {
    pub fn new() -> Self {
        let (frame_sender, _) = broadcast::channel(16);
        Self {
            frame_sender,
            is_streaming: Arc::new(Mutex::new(false)),
            camera_mode: Arc::new(Mutex::new("Initializing".to_string())),
        }
    }
}

/// Main camera capture function with OpenCV integration and fallback strategy
pub async fn start_camera_capture(
    camera_state: CameraState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    {
        let mut streaming = camera_state.is_streaming.lock().unwrap();
        *streaming = true;
    }

    println!("📹 Initializing camera system...");

    // Try to open real camera first (only if OpenCV feature is enabled)
    #[cfg(feature = "real_camera")]
    {
        let camera_result = try_open_real_camera(&camera_state).await;

        match camera_result {
            Ok(_) => {
                println!("📹 Real camera capture completed");
                return Ok(());
            }
            Err(e) => {
                println!("⚠️ Real camera failed: {}", e);
                println!("📹 Falling back to simulation mode...");
            }
        }
    }

    #[cfg(not(feature = "real_camera"))]
    {
        println!("📹 Real camera support not compiled. Using simulation mode...");
    }

    {
        let mut mode = camera_state.camera_mode.lock().unwrap();
        *mode = "Simulation".to_string();
    }

    // Fallback to simulation
    if let Err(sim_err) = run_simulation_camera(&camera_state).await {
        eprintln!("❌ Simulation camera also failed: {}", sim_err);
        {
            let mut mode = camera_state.camera_mode.lock().unwrap();
            *mode = "Error".to_string();
        }
    }

    Ok(())
}

#[cfg(feature = "real_camera")]
/// Try to open and use real OpenCV camera
async fn try_open_real_camera(
    camera_state: &CameraState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Try different camera backends in priority order
    let camera_configs = vec![
        (0, CAP_V4L2), // V4L2 for Linux (Raspberry Pi)
        (0, CAP_ANY),  // Generic backend
        (1, CAP_ANY),  // Try camera index 1
        (2, CAP_ANY),  // Try camera index 2
    ];

    let mut cap = None;

    for (camera_id, backend) in camera_configs {
        match VideoCapture::new(camera_id, backend) {
            Ok(test_cap) => {
                if test_cap.is_opened()? {
                    println!(
                        "✅ Camera opened successfully: index {} with backend {}",
                        camera_id, backend
                    );
                    cap = Some(test_cap);
                    break;
                } else {
                    println!(
                        "⚠️ Camera index {} backend {} opened but not ready",
                        camera_id, backend
                    );
                }
            }
            Err(e) => {
                println!(
                    "⚠️ Failed to open camera index {} backend {}: {}",
                    camera_id, backend, e
                );
            }
        }
    }

    let mut cap = cap.ok_or("No working camera found")?;

    // Configure camera properties
    let _ = cap.set(opencv::videoio::CAP_PROP_FRAME_WIDTH, 640.0);
    let _ = cap.set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, 480.0);
    let _ = cap.set(opencv::videoio::CAP_PROP_FPS, 30.0);
    let _ = cap.set(opencv::videoio::CAP_PROP_BUFFERSIZE, 1.0); // Reduce buffer for lower latency

    {
        let mut mode = camera_state.camera_mode.lock().unwrap();
        *mode = "OpenCV".to_string();
    }

    println!("📹 Real camera capture started with OpenCV");

    let mut frame = Mat::default();
    let mut encode_buffer = Vector::<u8>::new();
    let encode_params = Vector::<i32>::new();

    loop {
        // Check if we should continue streaming
        {
            let streaming = camera_state.is_streaming.lock().unwrap();
            if !*streaming {
                break;
            }
        }

        match cap.read(&mut frame) {
            Ok(true) => {
                // Frame successfully read, check if it's not empty
                if !frame.empty() {
                    // Encode frame as JPEG
                    match imgcodecs::imencode(".jpg", &frame, &mut encode_buffer, &encode_params) {
                        Ok(_) => {
                            // Convert to bytes and send
                            let frame_bytes = Bytes::from(encode_buffer.to_vec());

                            // Send frame to all subscribers (ignore errors if no subscribers)
                            let _ = camera_state.frame_sender.send(frame_bytes);

                            // Clear buffer for next frame
                            encode_buffer.clear();
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to encode frame: {}", e);
                            // Continue with next frame
                        }
                    }
                } else {
                    // Frame is empty, skip and continue
                    eprintln!("⚠️ Received empty frame from camera");
                }
            }
            Ok(false) => {
                eprintln!("⚠️ Failed to read frame from camera");
                time::sleep(Duration::from_millis(100)).await;
            }
            Err(e) => {
                eprintln!("❌ Camera read error: {}", e);
                return Err(format!("Camera read error: {}", e).into());
            }
        }

        // Small delay to prevent overwhelming the CPU (~30 FPS)
        time::sleep(Duration::from_millis(33)).await;
    }

    println!("📹 Real camera capture stopped");
    Ok(())
}

/// Simulation camera fallback
async fn run_simulation_camera(
    camera_state: &CameraState,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("📹 Simulation camera started");

    let mut frame_counter = 0u32;

    loop {
        // Check if we should continue streaming
        {
            let streaming = camera_state.is_streaming.lock().unwrap();
            if !*streaming {
                break;
            }
        }

        // Generate a simulated camera frame
        let frame_bytes = generate_test_frame(frame_counter)?;

        // Send frame to all subscribers (ignore errors if no subscribers)
        let _ = camera_state.frame_sender.send(frame_bytes);

        frame_counter = frame_counter.wrapping_add(1);

        // Small delay to prevent overwhelming the CPU (~30 FPS)
        time::sleep(Duration::from_millis(33)).await;
    }

    println!("📹 Simulation camera stopped");
    Ok(())
}

/// Generate a test frame (simulated camera output)
fn generate_test_frame(
    frame_counter: u32,
) -> Result<Bytes, Box<dyn std::error::Error + Send + Sync>> {
    let width = 640u32;
    let height = 480u32;

    // Create a dynamic image with animated content
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let time_factor = (frame_counter as f32 * 0.1).sin();
    let mut rng = rand::thread_rng();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        // Create animated gradient background
        let r = ((x as f32 / width as f32) * 255.0 * (1.0 + time_factor * 0.3)) as u8;
        let g = ((y as f32 / height as f32) * 255.0 * (1.0 + time_factor * 0.2)) as u8;
        let b = ((time_factor + 1.0) * 127.0) as u8;

        // Add some random noise for camera-like effect
        let noise = rng.gen_range(-10..10);
        let r = (r as i16 + noise).clamp(0, 255) as u8;
        let g = (g as i16 + noise).clamp(0, 255) as u8;
        let b = (b as i16 + noise).clamp(0, 255) as u8;

        *pixel = Rgb([r, g, b]);
    }

    // Add timestamp and frame counter text overlay
    let _timestamp = chrono::Utc::now()
        .format("%Y-%m-%d %H:%M:%S UTC")
        .to_string();

    // For simplicity, we'll create a basic pattern instead of actual text
    // Draw a moving circle to simulate motion
    let center_x = (width / 2) as i32 + ((frame_counter as f32 * 0.05).cos() * 100.0) as i32;
    let center_y = (height / 2) as i32 + ((frame_counter as f32 * 0.03).sin() * 50.0) as i32;
    let radius = 30;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = x as i32 - center_x;
        let dy = y as i32 - center_y;
        let distance = (dx * dx + dy * dy) as f32;

        if distance <= (radius * radius) as f32 {
            *pixel = Rgb([255, 255, 255]); // White circle
        }
    }

    // Draw frame counter in top-left corner (simple pixel pattern)
    for i in 0..10 {
        for j in 0..50 {
            if let Some(pixel) = img.get_pixel_mut_checked(j, i) {
                let intensity = if (j / 5) % 2 == (frame_counter / 30) % 2 {
                    255
                } else {
                    0
                };
                *pixel = Rgb([intensity, intensity, 0]);
            }
        }
    }

    // Convert to JPEG bytes
    let mut jpeg_bytes = Vec::new();
    let mut cursor = std::io::Cursor::new(&mut jpeg_bytes);
    img.write_to(&mut cursor, image::ImageFormat::Jpeg)?;

    Ok(Bytes::from(jpeg_bytes))
}
