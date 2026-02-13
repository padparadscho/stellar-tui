//! Stellar RPC method getTransactions

use serde_json::Value;

use crate::app::core::forms::FormState;
use crate::app::methods::{build_paged_request, paged_fields, MethodId, MethodSpec};

/// Returns the method specification
pub fn spec() -> MethodSpec {
    MethodSpec {
        id: MethodId::GetTransactions,
        name: "getTransactions",
        http_method: "POST",
        fields: paged_fields(),
        help: "Returns lists of transactions.",
    }
}

/// Builds the JSON-RPC params object from form values
pub fn build(form: &FormState) -> Result<Value, String> {
    // Reuses the shared pagination contract used by ledger and transaction list endpoints
    build_paged_request(form)
}
