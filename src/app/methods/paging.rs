use serde_json::{json, Value};

use crate::app::core::forms::{FieldSpec, FieldType, FormState};

use super::{parse_optional_string, parse_optional_u64};

pub(crate) fn build_paged_request(form: &FormState) -> Result<Value, String> {
    let start_ledger = parse_optional_u64(form, "startLedger")?;
    let cursor = parse_optional_string(form, "cursor");
    let limit = parse_optional_u64(form, "limit")?;
    let xdr_format = parse_optional_string(form, "xdrFormat");

    // Stellar RPC pagination accepts a cursor or a start ledger seed, never both
    if cursor.is_some() && start_ledger.is_some() {
        return Err("Provide either startLedger or cursor, not both".to_string());
    }

    let mut params = serde_json::Map::new();
    if let Some(value) = start_ledger {
        params.insert("startLedger".to_string(), json!(value));
    }
    if let Some(value) = xdr_format {
        params.insert("xdrFormat".to_string(), json!(value));
    }

    if limit.is_some() || cursor.is_some() {
        // Keeps pagination nested to match RPC request schema
        let mut pagination = serde_json::Map::new();
        if let Some(value) = limit {
            pagination.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = cursor {
            pagination.insert("cursor".to_string(), json!(value));
        }
        params.insert("pagination".to_string(), Value::Object(pagination));
    }

    Ok(Value::Object(params))
}

/// Shared field schema for methods that expose the same ledger pagination contract
pub(crate) fn paged_fields() -> Vec<FieldSpec> {
    vec![
        FieldSpec {
            key: "startLedger",
            label: "Start ledger",
            field_type: FieldType::Number,
            optional: true,
            default_value: "",
            hint: "Inclusive start ledger sequence",
        },
        FieldSpec {
            key: "cursor",
            label: "Cursor",
            field_type: FieldType::Text,
            optional: true,
            default_value: "",
            hint: "Use instead of 'startLedger'",
        },
        FieldSpec {
            key: "limit",
            label: "Limit",
            field_type: FieldType::Number,
            optional: true,
            default_value: "10",
            hint: "1-10000 (default: 10)",
        },
        FieldSpec {
            key: "xdrFormat",
            label: "XDR format",
            field_type: FieldType::Text,
            optional: true,
            default_value: "json",
            hint: "'json' (default) or 'base64'",
        },
    ]
}
