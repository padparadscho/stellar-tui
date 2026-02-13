//! Stellar RPC method sendTransaction

use serde_json::{json, Value};

use crate::app::core::forms::{FieldSpec, FieldType, FormState};
use crate::app::methods::{parse_optional_string, parse_required_string, MethodId, MethodSpec};

/// Returns the method specification
pub fn spec() -> MethodSpec {
    MethodSpec {
        id: MethodId::SendTransaction,
        name: "sendTransaction",
        http_method: "POST",
        fields: vec![
            FieldSpec {
                key: "transaction",
                label: "Transaction envelope XDR",
                field_type: FieldType::Text,
                optional: false,
                default_value: "",
                hint: "Base64-encoded envelope",
            },
            FieldSpec {
                key: "xdrFormat",
                label: "XDR format",
                field_type: FieldType::Text,
                optional: true,
                default_value: "json",
                hint: "'json' (default) or 'base64'",
            },
        ],
        help: "Submits a transaction.",
    }
}

/// Builds the JSON-RPC params object from form values
pub fn build(form: &FormState) -> Result<Value, String> {
    // Endpoint accepts a signed transaction envelope in encoded XDR form
    let tx = parse_required_string(form, "transaction")?;
    let xdr_format = parse_optional_string(form, "xdrFormat");
    let mut params = serde_json::Map::new();
    params.insert("transaction".to_string(), json!(tx));
    if let Some(value) = xdr_format {
        params.insert("xdrFormat".to_string(), json!(value));
    }
    Ok(Value::Object(params))
}
