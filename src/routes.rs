use crate::{api, camera::CameraState, web};
use axum::{routing::get, Router};
use tower_http::services::ServeDir;

/// Configure all application routes
pub fn create_routes(camera_state: CameraState) -> Router {
    Router::new()
        // Web page routes
        .route("/", get(web::hello_world))
        .route("/camera", get(web::camera_page))
        .route("/status", get(web::system_status))
        // API routes
        .route("/api/hello", get(api::hello_api))
        .route("/api/camera/stream", get(api::camera_stream))
        .route("/api/camera/status", get(api::camera_status))
        .route("/api/system-info", get(api::system_info))
        // Static file serving
        .nest_service("/static", ServeDir::new("static"))
        // Share camera state across all handlers
        .with_state(camera_state)
}
