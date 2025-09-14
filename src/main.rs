use std::net::SocketAddr;

mod api;
mod camera;
mod routes;
mod system_info;
mod web;

use camera::{start_camera_capture, CameraState};
use routes::create_routes;
use system_info::SystemInfo;

#[tokio::main]
async fn main() {
    // Initialize camera state
    let camera_state = CameraState::new();

    // Start camera capture task
    let camera_task = camera_state.clone();
    tokio::spawn(async move {
        if let Err(e) = start_camera_capture(camera_task).await {
            eprintln!("Camera capture error: {}", e);
        }
    });

    // Create application with all routes configured
    let app = create_routes(camera_state);

    // Get system information for display
    let system_info = SystemInfo::detect();
    let network_info = system_info.get_network_info();

    // For Raspberry Pi, bind to all interfaces (0.0.0.0) so it's accessible from other devices
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!(
        "🚀 {} Camera Web Server running on http://{}",
        system_info.platform_name,
        network_info
            .replace("0.0.0.0:", "")
            .replace("127.0.0.1:", "")
    );
    println!("📄 Main page: http://{}", network_info);
    println!("📹 Camera page: http://{}/camera", network_info);
    println!("🔧 API endpoint: http://{}/api/hello", network_info);
    println!("📊 System status: http://{}/status", network_info);
    println!(
        "Access from other devices using your {}'s IP address",
        system_info.platform_name
    );

    axum::serve(listener, app).await.unwrap();
}
