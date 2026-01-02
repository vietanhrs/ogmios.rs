//! Error types for the Ogmios client.

use thiserror::Error;

/// Main error type for the Ogmios client.
#[derive(Error, Debug)]
pub enum OgmiosError {
    /// WebSocket connection error
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// HTTP handshake error
    #[error("HTTP handshake error: {0}")]
    HttpHandshake(String),

    /// HTTP request error (for health checks)
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Server not ready error
    #[error("Server not ready: network synchronization is {synchronization:.2}%, minimum required is {minimum:.2}%")]
    ServerNotReady {
        synchronization: f64,
        minimum: f64,
    },

    /// Connection closed unexpectedly
    #[error("Connection closed unexpectedly")]
    ConnectionClosed,

    /// Socket not open
    #[error("Socket is not open (state: {state})")]
    SocketNotOpen { state: String },

    /// Invalid response from server
    #[error("Invalid response from server: {message}")]
    InvalidResponse { message: String },

    /// Request timeout
    #[error("Request timed out after {timeout_ms}ms")]
    Timeout { timeout_ms: u64 },

    /// Intersection not found during chain sync
    #[error("Intersection not found: {tip:?}")]
    IntersectionNotFound { tip: Option<String> },

    /// Transaction submission error
    #[error("Transaction submission failed: {0}")]
    SubmissionError(String),

    /// Transaction evaluation error
    #[error("Transaction evaluation failed: {0}")]
    EvaluationError(String),

    /// Ledger state acquisition error
    #[error("Failed to acquire ledger state: {0}")]
    AcquisitionError(String),

    /// Query error
    #[error("Query failed: {0}")]
    QueryError(String),

    /// URL parsing error
    #[error("URL parsing error: {0}")]
    UrlParse(#[from] url::ParseError),

    /// Generic I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Channel send error
    #[error("Channel send error: {0}")]
    ChannelSend(String),

    /// Channel receive error
    #[error("Channel receive error: receiver dropped")]
    ChannelRecv,
}

/// Result type alias for Ogmios operations.
pub type Result<T> = std::result::Result<T, OgmiosError>;
