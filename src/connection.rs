//! Connection management for Ogmios.
//!
//! This module provides types and functions for establishing and managing
//! WebSocket connections to an Ogmios server.

use crate::error::{OgmiosError, Result};
use crate::schema::{JsonRpcRequest, JsonRpcResponse};
use futures_util::{SinkExt, StreamExt};
use serde::{de::DeserializeOwned, Serialize};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::sync::{mpsc, oneshot, Mutex};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{handshake::client::Request, protocol::Message},
    MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, trace};

/// Default Ogmios host.
pub const DEFAULT_HOST: &str = "127.0.0.1";

/// Default Ogmios port.
pub const DEFAULT_PORT: u16 = 1337;

/// Default maximum payload size (128 MB).
pub const DEFAULT_MAX_PAYLOAD: usize = 128 * 1024 * 1024;

/// Connection configuration.
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Ogmios server host.
    pub host: String,
    /// Ogmios server port.
    pub port: u16,
    /// Use TLS (wss://).
    pub tls: bool,
    /// Maximum payload size in bytes.
    pub max_payload: usize,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_string(),
            port: DEFAULT_PORT,
            tls: false,
            max_payload: DEFAULT_MAX_PAYLOAD,
        }
    }
}

impl ConnectionConfig {
    /// Create a new connection configuration.
    pub fn new(host: impl Into<String>, port: u16) -> Self {
        Self {
            host: host.into(),
            port,
            ..Default::default()
        }
    }

    /// Enable TLS.
    pub fn with_tls(mut self) -> Self {
        self.tls = true;
        self
    }

    /// Set maximum payload size.
    pub fn with_max_payload(mut self, max_payload: usize) -> Self {
        self.max_payload = max_payload;
        self
    }
}

/// Connection addresses.
#[derive(Debug, Clone)]
pub struct ConnectionAddress {
    /// HTTP address for health checks.
    pub http: String,
    /// WebSocket address for protocol communication.
    pub websocket: String,
}

/// A connection object representing an Ogmios server connection.
#[derive(Debug, Clone)]
pub struct Connection {
    /// Maximum payload size.
    pub max_payload: usize,
    /// Connection addresses.
    pub address: ConnectionAddress,
}

impl Connection {
    /// Create a connection object from configuration.
    pub fn from_config(config: &ConnectionConfig) -> Self {
        let scheme = if config.tls { "https" } else { "http" };
        let ws_scheme = if config.tls { "wss" } else { "ws" };

        Self {
            max_payload: config.max_payload,
            address: ConnectionAddress {
                http: format!("{}://{}:{}", scheme, config.host, config.port),
                websocket: format!("{}://{}:{}", ws_scheme, config.host, config.port),
            },
        }
    }
}

/// Create a connection object from optional configuration.
pub fn create_connection_object(config: Option<ConnectionConfig>) -> Connection {
    Connection::from_config(&config.unwrap_or_default())
}

/// Interaction type for client connections.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InteractionType {
    /// Long-running connection (persists across multiple requests).
    LongRunning,
    /// One-time connection (closes after request).
    OneTime,
}

/// Internal message for WebSocket communication.
#[derive(Debug)]
enum WsMessage {
    /// Send a request and wait for a response.
    Request {
        payload: String,
        response_tx: oneshot::Sender<Result<String>>,
    },
    /// Send a message without waiting for response.
    Send { payload: String },
    /// Close the connection.
    Close,
}

/// Shared WebSocket state.
struct WebSocketState {
    /// Sender for WebSocket messages.
    tx: mpsc::Sender<WsMessage>,
    /// Connection status.
    is_open: std::sync::atomic::AtomicBool,
}

/// Interaction context for Ogmios clients.
///
/// This is the main context object that clients use to communicate with Ogmios.
pub struct InteractionContext {
    /// Connection configuration.
    pub connection: Connection,
    /// Interaction type.
    pub interaction_type: InteractionType,
    /// Request ID counter.
    request_id: AtomicU64,
    /// WebSocket state.
    ws_state: Arc<WebSocketState>,
    /// Background task handle.
    _task_handle: tokio::task::JoinHandle<()>,
}

impl InteractionContext {
    /// Check if the socket is open.
    pub fn is_socket_open(&self) -> bool {
        self.ws_state.is_open.load(Ordering::SeqCst)
    }

    /// Get the next request ID.
    fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Send a JSON-RPC request and wait for the response.
    pub async fn request<P, R>(&self, method: &str, params: Option<P>) -> Result<R>
    where
        P: Serialize,
        R: DeserializeOwned,
    {
        ensure_socket_is_open(self)?;

        let id = self.next_request_id();
        let request = JsonRpcRequest::with_id(method, params, serde_json::Value::Number(id.into()));

        let payload = serde_json::to_string(&request)?;
        trace!("Sending request: {}", payload);

        let (response_tx, response_rx) = oneshot::channel();
        self.ws_state
            .tx
            .send(WsMessage::Request {
                payload,
                response_tx,
            })
            .await
            .map_err(|e| OgmiosError::ChannelSend(e.to_string()))?;

        let response_str = response_rx.await.map_err(|_| OgmiosError::ChannelRecv)??;
        trace!("Received response: {}", response_str);

        let response: JsonRpcResponse<R> = serde_json::from_str(&response_str)?;

        response
            .into_result()
            .map_err(|e| OgmiosError::InvalidResponse {
                message: e.to_string(),
            })
    }

    /// Send a JSON-RPC notification (no response expected).
    pub async fn notify<P: Serialize>(&self, method: &str, params: Option<P>) -> Result<()> {
        ensure_socket_is_open(self)?;

        let request = JsonRpcRequest::new(method, params);
        let payload = serde_json::to_string(&request)?;
        trace!("Sending notification: {}", payload);

        self.ws_state
            .tx
            .send(WsMessage::Send { payload })
            .await
            .map_err(|e| OgmiosError::ChannelSend(e.to_string()))?;

        Ok(())
    }

    /// Close the connection.
    pub async fn shutdown(&self) -> Result<()> {
        let _ = self.ws_state.tx.send(WsMessage::Close).await;
        self.ws_state.is_open.store(false, Ordering::SeqCst);
        Ok(())
    }
}

/// Ensure the WebSocket is open.
pub fn ensure_socket_is_open(context: &InteractionContext) -> Result<()> {
    if !context.is_socket_open() {
        return Err(OgmiosError::SocketNotOpen {
            state: "closed".to_string(),
        });
    }
    Ok(())
}

/// Error handler callback type.
pub type ErrorHandler = Box<dyn Fn(OgmiosError) + Send + Sync>;

/// Close handler callback type.
pub type CloseHandler = Box<dyn Fn() + Send + Sync>;

/// Options for creating an interaction context.
pub struct InteractionContextOptions {
    /// Connection configuration.
    pub connection: ConnectionConfig,
    /// Interaction type.
    pub interaction_type: InteractionType,
    /// Error handler.
    pub error_handler: Option<ErrorHandler>,
    /// Close handler.
    pub close_handler: Option<CloseHandler>,
}

impl Default for InteractionContextOptions {
    fn default() -> Self {
        Self {
            connection: ConnectionConfig::default(),
            interaction_type: InteractionType::LongRunning,
            error_handler: None,
            close_handler: None,
        }
    }
}

/// Create an interaction context.
///
/// This establishes a WebSocket connection to the Ogmios server and returns
/// a context that can be used to make requests.
pub async fn create_interaction_context(
    options: InteractionContextOptions,
) -> Result<InteractionContext> {
    let connection = Connection::from_config(&options.connection);
    let ws_url = &connection.address.websocket;

    debug!("Connecting to Ogmios at {}", ws_url);

    // Build WebSocket request
    let request = Request::builder()
        .uri(ws_url)
        .header(
            "Host",
            format!("{}:{}", options.connection.host, options.connection.port),
        )
        .header("Connection", "Upgrade")
        .header("Upgrade", "websocket")
        .header("Sec-WebSocket-Version", "13")
        .header(
            "Sec-WebSocket-Key",
            tokio_tungstenite::tungstenite::handshake::client::generate_key(),
        )
        .body(())
        .map_err(|e| OgmiosError::HttpHandshake(e.to_string()))?;

    let (ws_stream, _) = connect_async(request)
        .await
        .map_err(|e| OgmiosError::WebSocket(e.to_string()))?;
    debug!("WebSocket connection established");

    let (tx, rx) = mpsc::channel::<WsMessage>(100);
    let is_open = std::sync::atomic::AtomicBool::new(true);

    let ws_state = Arc::new(WebSocketState { tx, is_open });

    let ws_state_clone = ws_state.clone();
    let error_handler = options.error_handler;
    let close_handler = options.close_handler;

    // Spawn background task to handle WebSocket messages
    let task_handle = tokio::spawn(async move {
        handle_websocket(ws_stream, rx, ws_state_clone, error_handler, close_handler).await;
    });

    Ok(InteractionContext {
        connection,
        interaction_type: options.interaction_type,
        request_id: AtomicU64::new(1),
        ws_state,
        _task_handle: task_handle,
    })
}

/// Handle WebSocket message loop.
async fn handle_websocket(
    ws_stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    mut rx: mpsc::Receiver<WsMessage>,
    ws_state: Arc<WebSocketState>,
    error_handler: Option<ErrorHandler>,
    close_handler: Option<CloseHandler>,
) {
    let (mut write, mut read) = ws_stream.split();

    // Pending requests waiting for responses
    let pending: Arc<Mutex<Vec<oneshot::Sender<Result<String>>>>> =
        Arc::new(Mutex::new(Vec::new()));
    let pending_clone = pending.clone();

    // Spawn read task
    let read_task = tokio::spawn(async move {
        while let Some(msg_result) = read.next().await {
            match msg_result {
                Ok(Message::Text(text)) => {
                    let mut pending = pending_clone.lock().await;
                    if let Some(tx) = pending.pop() {
                        let _ = tx.send(Ok(text));
                    }
                }
                Ok(Message::Close(_)) => {
                    debug!("WebSocket closed by server");
                    break;
                }
                Ok(Message::Ping(data)) => {
                    trace!("Received ping: {:?}", data);
                    // Pong is handled automatically by tungstenite
                }
                Ok(_) => {}
                Err(e) => {
                    error!("WebSocket read error: {}", e);
                    let err_msg = e.to_string();
                    let mut pending = pending_clone.lock().await;
                    while let Some(tx) = pending.pop() {
                        let _ = tx.send(Err(OgmiosError::WebSocket(err_msg.clone())));
                    }
                    break;
                }
            }
        }
    });

    // Handle outgoing messages
    while let Some(msg) = rx.recv().await {
        match msg {
            WsMessage::Request {
                payload,
                response_tx,
            } => {
                {
                    let mut pending = pending.lock().await;
                    pending.push(response_tx);
                }
                if let Err(e) = write.send(Message::Text(payload)).await {
                    error!("Failed to send WebSocket message: {}", e);
                    let mut pending = pending.lock().await;
                    if let Some(tx) = pending.pop() {
                        let _ = tx.send(Err(OgmiosError::WebSocket(e.to_string())));
                    }
                }
            }
            WsMessage::Send { payload } => {
                if let Err(e) = write.send(Message::Text(payload)).await {
                    error!("Failed to send WebSocket message: {}", e);
                    if let Some(ref handler) = error_handler {
                        handler(OgmiosError::WebSocket(e.to_string()));
                    }
                }
            }
            WsMessage::Close => {
                let _ = write.send(Message::Close(None)).await;
                break;
            }
        }
    }

    ws_state.is_open.store(false, Ordering::SeqCst);
    read_task.abort();

    if let Some(handler) = close_handler {
        handler();
    }
}

/// Send a request and get a response using an interaction context.
///
/// This is a helper function that wraps the request/response cycle.
pub async fn send<P, R>(context: &InteractionContext, method: &str, params: Option<P>) -> Result<R>
where
    P: Serialize,
    R: DeserializeOwned,
{
    context.request(method, params).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_config_default() {
        let config = ConnectionConfig::default();
        assert_eq!(config.host, DEFAULT_HOST);
        assert_eq!(config.port, DEFAULT_PORT);
        assert!(!config.tls);
        assert_eq!(config.max_payload, DEFAULT_MAX_PAYLOAD);
    }

    #[test]
    fn test_connection_from_config() {
        let config = ConnectionConfig::new("localhost", 1338).with_tls();
        let connection = Connection::from_config(&config);

        assert_eq!(connection.address.http, "https://localhost:1338");
        assert_eq!(connection.address.websocket, "wss://localhost:1338");
    }

    #[test]
    fn test_create_connection_object() {
        let connection = create_connection_object(None);
        assert_eq!(connection.address.http, "http://127.0.0.1:1337");
        assert_eq!(connection.address.websocket, "ws://127.0.0.1:1337");
    }
}
