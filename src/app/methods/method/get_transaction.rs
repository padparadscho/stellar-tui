//! Stellar RPC method getTransaction

use serde_json::{json, Value};

use crate::app::core::forms::{FieldSpec, FieldType, FormState};
use crate::app::methods::{parse_optional_string, parse_required_string, MethodId, MethodSpec};

/// Returns the method specification
pub fn spec() -> MethodSpec {
    MethodSpec {
        id: MethodId::GetTransaction,
        name: "getTransaction",
        http_method: "POST",
        fields: vec![
            FieldSpec {
                key: "hash",
                label: "Transaction hash",
                field_type: FieldType::Text,
                optional: false,
                default_value: "",
                hint: "64-char hex",
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
        help: "Returns transaction details.",
    }
}

/// Builds the JSON-RPC params object from form values
pub fn build(form: &FormState) -> Result<Value, String> {
    // Transaction hash is the only lookup key accepted by the endpoint
    let hash = parse_required_string(form, "hash")?;
    let xdr_format = parse_optional_string(form, "xdrFormat");
    let mut params = serde_json::Map::new();
    params.insert("hash".to_string(), json!(hash));
    if let Some(value) = xdr_format {
        params.insert("xdrFormat".to_string(), json!(value));
    }
    Ok(Value::Object(params))
}
