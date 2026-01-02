//! JSON-RPC types for Ogmios communication.

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC version.
pub const JSONRPC_VERSION: &str = "2.0";

/// JSON-RPC request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest<P = Value> {
    /// JSON-RPC version.
    pub jsonrpc: String,
    /// Method name.
    pub method: String,
    /// Request parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<P>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
}

impl<P> JsonRpcRequest<P> {
    /// Create a new JSON-RPC request.
    pub fn new(method: impl Into<String>, params: Option<P>) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.into(),
            params,
            id: None,
        }
    }

    /// Create a new JSON-RPC request with an ID.
    pub fn with_id(method: impl Into<String>, params: Option<P>, id: Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method: method.into(),
            params,
            id: Some(id),
        }
    }
}

/// JSON-RPC response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse<R = Value> {
    /// JSON-RPC version.
    pub jsonrpc: String,
    /// Result (mutually exclusive with error).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<R>,
    /// Error (mutually exclusive with result).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    /// Request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
}

impl<R> JsonRpcResponse<R> {
    /// Check if the response is an error.
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    /// Get the result, returning an error if the response contains an error.
    pub fn into_result(self) -> Result<R, JsonRpcError> {
        if let Some(error) = self.error {
            Err(error)
        } else if let Some(result) = self.result {
            Ok(result)
        } else {
            Err(JsonRpcError {
                code: -32600,
                message: "Invalid response: no result or error".to_string(),
                data: None,
            })
        }
    }
}

/// JSON-RPC error.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Error code.
    pub code: i32,
    /// Error message.
    pub message: String,
    /// Additional error data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

impl std::fmt::Display for JsonRpcError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "JSON-RPC error {}: {}", self.code, self.message)
    }
}

impl std::error::Error for JsonRpcError {}

/// Standard JSON-RPC error codes.
pub mod error_codes {
    /// Parse error - Invalid JSON.
    pub const PARSE_ERROR: i32 = -32700;
    /// Invalid Request - The JSON is not a valid Request object.
    pub const INVALID_REQUEST: i32 = -32600;
    /// Method not found.
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Invalid params.
    pub const INVALID_PARAMS: i32 = -32602;
    /// Internal error.
    pub const INTERNAL_ERROR: i32 = -32603;
    /// Server error range start.
    pub const SERVER_ERROR_START: i32 = -32000;
    /// Server error range end.
    pub const SERVER_ERROR_END: i32 = -32099;
}

/// Ogmios-specific response types.
pub mod responses {
    use serde::{Deserialize, Serialize};
    use serde_json::Value as JsonValue;
    use super::super::block::Block;
    use super::super::primitives::{Point, Slot, Tip, TransactionId};
    use super::super::transaction::{Transaction, EvaluationResult};

    /// Chain sync next block response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(tag = "direction", rename_all = "camelCase")]
    pub enum NextBlockResponse {
        /// Forward direction - new block.
        Forward {
            block: Block,
            tip: Tip,
        },
        /// Backward direction - rollback.
        Backward {
            point: Point,
            tip: Tip,
        },
    }

    /// Find intersection response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct FindIntersectionResponse {
        /// The intersection point found.
        pub intersection: Option<Point>,
        /// Current tip.
        pub tip: Tip,
    }

    /// Acquire ledger state response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AcquireLedgerStateResponse {
        /// Acquired point.
        pub acquired: String,
        /// Slot number.
        pub slot: Slot,
    }

    /// Acquire mempool response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct AcquireMempoolResponse {
        /// Acquired slot.
        pub acquired: String,
        /// Slot number.
        pub slot: Slot,
    }

    /// Submit transaction response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct SubmitTransactionResponse {
        /// Transaction ID.
        pub transaction: TransactionIdWrapper,
    }

    /// Transaction ID wrapper.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct TransactionIdWrapper {
        /// Transaction ID.
        pub id: TransactionId,
    }

    /// Evaluate transaction response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum EvaluateTransactionResponse {
        /// Successful evaluation.
        Success(Vec<EvaluationResult>),
        /// Evaluation with errors.
        Error { error: JsonValue },
    }

    /// Next transaction response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NextTransactionResponse {
        /// Transaction (null if none available).
        pub transaction: Option<TransactionOrId>,
    }

    /// Transaction or just its ID.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(untagged)]
    pub enum TransactionOrId {
        /// Full transaction.
        Full(Transaction),
        /// Just the ID.
        Id { id: TransactionId },
    }

    /// Has transaction response.
    #[derive(Debug, Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct HasTransactionResponse {
        /// Whether the transaction is in the mempool.
        pub has_transaction: bool,
    }
}
