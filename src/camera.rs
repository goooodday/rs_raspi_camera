use anyhow::{Result, anyhow};
use opencv::{
    prelude::*,
    videoio::{VideoCapture, CAP_ANY, CAP_PROP_FRAME_WIDTH, CAP_PROP_FRAME_HEIGHT},
    imgcodecs::{imencode, IMWRITE_JPEG_QUALITY},
    core::Vector,
};
use std::sync::{Arc, Mutex};
use tokio::time::{interval, Duration};
use log::{info, error, warn};

pub struct Camera {
    cap: Arc<Mutex<VideoCapture>>,
    frame_rate: u64,
}

impl Camera {
    pub fn new(device_id: i32, width: i32, height: i32, fps: u64) -> Result<Self> {
        info!("Initializing camera with device ID: {}", device_id);
        
        let mut cap = VideoCapture::new(device_id, CAP_ANY)?;
        
        if !cap.is_opened()? {
            return Err(anyhow!("Unable to open camera device {}", device_id));
        }

        // Set camera properties
        cap.set(CAP_PROP_FRAME_WIDTH, width as f64)?;
        cap.set(CAP_PROP_FRAME_HEIGHT, height as f64)?;
        
        info!("Camera initialized successfully - {}x{} @ {}fps", width, height, fps);
        
        Ok(Camera {
            cap: Arc::new(Mutex::new(cap)),
            frame_rate: fps,
        })
    }

    pub async fn capture_frame(&self) -> Result<Vec<u8>> {
        let mut frame = Mat::default();
        
        // Capture frame from camera
        {
            let mut cap = self.cap.lock().unwrap();
            cap.read(&mut frame)?;
        }
        
        if frame.empty() {
            return Err(anyhow!("Failed to capture frame - empty frame"));
        }

        // Encode frame to JPEG
        let mut buf = Vector::new();
        let params = Vector::from(vec![
            IMWRITE_JPEG_QUALITY,
            85, // JPEG quality (0-100)
        ]);
        
        imencode(".jpg", &frame, &mut buf, &params)?;
        
        Ok(buf.to_vec())
    }

    pub fn get_frame_interval(&self) -> Duration {
        Duration::from_millis(1000 / self.frame_rate)
    }
}

pub struct CameraStream {
    camera: Camera,
}

impl CameraStream {
    pub fn new(device_id: i32, width: i32, height: i32, fps: u64) -> Result<Self> {
        let camera = Camera::new(device_id, width, height, fps)?;
        Ok(CameraStream { camera })
    }

    pub async fn start_streaming<F>(&self, mut frame_callback: F) -> Result<()>
    where
        F: FnMut(Vec<u8>) + Send + 'static,
    {
        let mut interval = interval(self.camera.get_frame_interval());
        
        info!("Starting camera streaming...");
        
        loop {
            interval.tick().await;
            
            match self.camera.capture_frame().await {
                Ok(frame_data) => {
                    frame_callback(frame_data);
                },
                Err(e) => {
                    error!("Failed to capture frame: {}", e);
                    // Continue trying to capture frames even if one fails
                    continue;
                }
            }
        }
    }
}
