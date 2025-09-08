mod camera;
mod server;

use anyhow::Result;
use log::{info, error};
use std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    env_logger::init();
    
    info!("Starting Raspberry Pi Camera Stream Server...");
    
    // Configuration
    let camera_device_id = 0; // Default camera device
    let camera_width = 640;
    let camera_height = 480;
    let camera_fps = 30;
    let server_port = 3030;
    
    // Create stream server
    let server = Arc::new(server::StreamServer::new(server_port));
    let frame_sender = server.get_frame_sender();
    
    // Create camera stream
    let camera_stream = match camera::CameraStream::new(
        camera_device_id, 
        camera_width, 
        camera_height, 
        camera_fps
    ) {
        Ok(stream) => stream,
        Err(e) => {
            error!("Failed to initialize camera: {}", e);
            error!("Make sure your camera is connected and accessible.");
            return Err(e);
        }
    };
    
    info!("Camera initialized successfully");
    
    // Start server in a separate task
    let server_clone = server.clone();
    let server_task = tokio::spawn(async move {
        if let Err(e) = server_clone.start().await {
            error!("Server error: {}", e);
        }
    });
    
    // Start camera streaming
    let streaming_task = tokio::spawn(async move {
        camera_stream.start_streaming(move |frame_data| {
            let frame_sender = frame_sender.clone();
            let server = server.clone();
            
            tokio::spawn(async move {
                if let Err(e) = server.broadcast_frame(frame_data).await {
                    error!("Failed to broadcast frame: {}", e);
                }
            });
        }).await
    });
    
    info!("All services started successfully");
    info!("Access the camera stream at: http://localhost:{}", server_port);
    
    // Wait for either task to complete
    tokio::select! {
        result = server_task => {
            match result {
                Ok(_) => info!("Server task completed"),
                Err(e) => error!("Server task failed: {}", e),
            }
        },
        result = streaming_task => {
            match result {
                Ok(Ok(())) => info!("Streaming task completed"),
                Ok(Err(e)) => error!("Streaming error: {}", e),
                Err(e) => error!("Streaming task failed: {}", e),
            }
        }
    }
    
    Ok(())
}
