use serde::{Deserialize, Serialize};

/// Outgoing JSON-RPC 2.0 request envelope
#[derive(Debug, Serialize)]
pub struct RpcRequest<P> {
    pub jsonrpc: &'static str,
    /// Request ID always 1 since the TUI sends one request at a time
    pub id: u64,
    pub method: String,
    pub params: P,
}

impl<P> RpcRequest<P> {
    pub fn new(method: &str, params: P) -> Self {
        Self {
            jsonrpc: "2.0",
            id: 1,
            method: method.to_string(),
            params,
        }
    }
}

/// JSON-RPC 2.0 error object
#[derive(Debug, Deserialize, Serialize)]
pub struct RpcError {
    pub code: i64,
    pub message: String,
}

/// Incoming JSON-RPC 2.0 response envelope
#[derive(Debug, Deserialize, Serialize)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: u64,
    /// Present on success
    pub result: Option<T>,
    /// Present on failure
    pub error: Option<RpcError>,
}
