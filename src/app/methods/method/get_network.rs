//! Stellar RPC method getNetwork

use serde_json::{json, Value};

use crate::app::core::forms::FormState;
use crate::app::methods::{MethodId, MethodSpec};

/// Returns the method specification
pub fn spec() -> MethodSpec {
    MethodSpec {
        id: MethodId::GetNetwork,
        name: "getNetwork",
        http_method: "POST",
        fields: vec![],
        help: "Returns network configuration.",
    }
}

/// Builds the JSON-RPC params object from form values
pub fn build(_form: &FormState) -> Result<Value, String> {
    // Keep explicit empty params to produce a stable JSON-RPC request envelope
    Ok(json!({}))
}
