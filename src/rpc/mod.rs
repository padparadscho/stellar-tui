pub mod types;

use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::rpc::types::{RpcRequest, RpcResponse};

/// HTTP client for Stellar JSON-RPC endpoints
pub struct RpcClient {
    endpoint: String,
    client: reqwest::Client,
}

impl RpcClient {
    /// Creates a client targeting the given RPC endpoint
    pub fn new(endpoint: String) -> Self {
        Self {
            endpoint,
            client: reqwest::Client::new(),
        }
    }

    /// Sends a JSON-RPC 2.0 request and deserializes the response
    pub async fn call<Req, Resp>(
        &self,
        method: &str,
        params: Req,
    ) -> anyhow::Result<RpcResponse<Resp>>
    where
        Req: Serialize,
        Resp: DeserializeOwned,
    {
        let request = RpcRequest::new(method, params);
        let response = self
            .client
            .post(&self.endpoint)
            .json(&request)
            .send()
            .await?
            .error_for_status()?;

        let body = response.json::<RpcResponse<Resp>>().await?;
        Ok(body)
    }
}
