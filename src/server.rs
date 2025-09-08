use warp::{Filter, ws::{Message, WebSocket}, Reply};
use futures::{FutureExt, StreamExt, SinkExt};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use base64;
use log::{info, warn, error};
use anyhow::Result;

type Clients = Arc<Mutex<HashMap<String, Client>>>;
type FrameSender = broadcast::Sender<Vec<u8>>;
type FrameReceiver = broadcast::Receiver<Vec<u8>>;

#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,
    pub sender: Option<futures::channel::mpsc::UnboundedSender<Message>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamMessage {
    pub message_type: String,
    pub data: String,
    pub timestamp: u64,
}

pub struct StreamServer {
    clients: Clients,
    frame_sender: FrameSender,
    port: u16,
}

impl StreamServer {
    pub fn new(port: u16) -> Self {
        let clients = Arc::new(Mutex::new(HashMap::new()));
        let (frame_sender, _) = broadcast::channel(100);
        
        StreamServer {
            clients,
            frame_sender,
            port,
        }
    }

    pub fn get_frame_sender(&self) -> FrameSender {
        self.frame_sender.clone()
    }

    pub async fn start(&self) -> Result<()> {
        let clients = self.clients.clone();
        let frame_sender = self.frame_sender.clone();

        // Health check endpoint
        let health = warp::path("health")
            .and(warp::get())
            .map(|| warp::reply::with_status("OK", warp::http::StatusCode::OK));

        // Static files route for serving the web client
        let static_files = warp::path("static")
            .and(warp::fs::dir("static"));

        // Main page route
        let index = warp::path::end()
            .and(warp::get())
            .and(warp::fs::file("static/index.html"));

        // WebSocket route for video streaming
        let websocket = warp::path("ws")
            .and(warp::ws())
            .and(warp::any().map(move || clients.clone()))
            .and(warp::any().map(move || frame_sender.clone()))
            .map(|ws: warp::ws::Ws, clients, frame_sender| {
                ws.on_upgrade(move |socket| handle_websocket(socket, clients, frame_sender))
            });

        let routes = health
            .or(websocket)
            .or(index)
            .or(static_files)
            .with(warp::cors().allow_any_origin());

        info!("Starting web server on port {}", self.port);
        info!("Camera stream will be available at: http://localhost:{}", self.port);

        warp::serve(routes)
            .run(([0, 0, 0, 0], self.port))
            .await;

        Ok(())
    }

    pub async fn broadcast_frame(&self, frame_data: Vec<u8>) -> Result<()> {
        match self.frame_sender.send(frame_data) {
            Ok(_) => {},
            Err(e) => {
                // This is expected when no clients are connected
                // Only log as warning if there are connected clients
                let client_count = self.clients.lock().unwrap().len();
                if client_count > 0 {
                    warn!("Failed to broadcast frame to {} clients: {}", client_count, e);
                }
            }
        }
        Ok(())
    }
}

async fn handle_websocket(
    websocket: WebSocket,
    clients: Clients,
    frame_sender: FrameSender,
) {
    let client_id = uuid::Uuid::new_v4().to_string();
    info!("New WebSocket connection: {}", client_id);

    let (mut client_sender, mut client_receiver) = websocket.split();
    let (tx, mut rx) = futures::channel::mpsc::unbounded();

    // Add client to the clients map
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.insert(client_id.clone(), Client {
            id: client_id.clone(),
            sender: Some(tx),
        });
    }

    info!("Client {} connected. Total clients: {}", client_id, clients.lock().unwrap().len());

    // Subscribe to frame broadcasts
    let mut frame_receiver = frame_sender.subscribe();

    // Handle outgoing messages (frames to client)
    let client_id_clone = client_id.clone();
    let outgoing_task = tokio::spawn(async move {
        loop {
            tokio::select! {
                // Handle frames from camera
                frame_result = frame_receiver.recv() => {
                    match frame_result {
                        Ok(frame_data) => {
                            let base64_data = base64::encode(&frame_data);
                            let message = StreamMessage {
                                message_type: "frame".to_string(),
                                data: base64_data,
                                timestamp: std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_millis() as u64,
                            };
                            
                            match serde_json::to_string(&message) {
                                Ok(json_message) => {
                                    if client_sender.send(Message::text(json_message)).await.is_err() {
                                        warn!("Failed to send frame to client {}", client_id_clone);
                                        break;
                                    }
                                },
                                Err(e) => {
                                    error!("Failed to serialize frame message: {}", e);
                                }
                            }
                        },
                        Err(e) => {
                            warn!("Frame receiver error for client {}: {}", client_id_clone, e);
                            break;
                        }
                    }
                },
                // Handle messages from our internal sender
                msg = rx.next() => {
                    match msg {
                        Some(message) => {
                            if client_sender.send(message).await.is_err() {
                                warn!("Failed to send message to client {}", client_id_clone);
                                break;
                            }
                        },
                        None => break,
                    }
                }
            }
        }
    });

    // Handle incoming messages from client
    let clients_for_incoming = clients.clone();
    let client_id_for_incoming = client_id.clone();
    let incoming_task = tokio::spawn(async move {
        while let Some(result) = client_receiver.next().await {
            match result {
                Ok(msg) => {
                    if msg.is_text() {
                        if let Ok(text) = msg.to_str() {
                            info!("Received message from client {}: {}", client_id_for_incoming, text);
                            // Handle client messages here if needed
                        }
                    } else if msg.is_close() {
                        info!("Client {} disconnected", client_id_for_incoming);
                        break;
                    }
                },
                Err(e) => {
                    warn!("WebSocket error for client {}: {}", client_id_for_incoming, e);
                    break;
                }
            }
        }
    });

    // Wait for either task to complete
    tokio::select! {
        _ = outgoing_task => {},
        _ = incoming_task => {},
    }

    // Clean up client
    {
        let mut clients_lock = clients.lock().unwrap();
        clients_lock.remove(&client_id);
    }
    
    info!("Client {} disconnected. Remaining clients: {}", client_id, clients.lock().unwrap().len());
}

// Add uuid dependency to Cargo.toml for generating unique client IDs
mod uuid {
    use std::fmt;
    
    pub struct Uuid(String);
    
    impl Uuid {
        pub fn new_v4() -> Self {
            let id = format!("client_{}", std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos());
            Uuid(id)
        }
    }
    
    impl fmt::Display for Uuid {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}
