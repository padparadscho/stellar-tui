//! Stellar RPC method getLedgerEntries

use serde_json::{json, Value};

use crate::app::core::forms::{FieldSpec, FieldType, FormState};
use crate::app::methods::{parse_list, parse_optional_string, MethodId, MethodSpec};

/// Returns the method specification
pub fn spec() -> MethodSpec {
    MethodSpec {
        id: MethodId::GetLedgerEntries,
        name: "getLedgerEntries",
        http_method: "POST",
        fields: vec![
            FieldSpec {
                key: "keys",
                label: "Ledger keys",
                field_type: FieldType::List,
                optional: false,
                default_value: "",
                hint: "Base64 LedgerKey values",
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
        help: "Returns ledger entries.",
    }
}

/// Builds the JSON-RPC params object from form values
pub fn build(form: &FormState) -> Result<Value, String> {
    let keys = parse_list(form, "keys");
    let xdr_format = parse_optional_string(form, "xdrFormat");
    // Empty key sets are rejected by the endpoint and should fail fast in UI
    if keys.is_empty() {
        return Err("At least one ledger key is required".to_string());
    }
    let mut params = serde_json::Map::new();
    params.insert("keys".to_string(), json!(keys));
    if let Some(value) = xdr_format {
        params.insert("xdrFormat".to_string(), json!(value));
    }
    Ok(Value::Object(params))
}
