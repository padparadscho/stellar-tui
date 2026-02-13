//! Stellar RPC method getEvents

use serde_json::{json, Value};

use crate::app::core::forms::{FieldSpec, FieldType, FormState};
use crate::app::methods::{
    parse_list, parse_optional_json, parse_optional_string, parse_optional_u64, MethodId,
    MethodSpec,
};

/// Returns the method specification
pub fn spec() -> MethodSpec {
    MethodSpec {
        id: MethodId::GetEvents,
        name: "getEvents",
        http_method: "POST",
        fields: vec![
            FieldSpec {
                key: "startLedger",
                label: "Start ledger",
                field_type: FieldType::Number,
                optional: true,
                default_value: "",
                hint: "Inclusive start ledger sequence",
            },
            FieldSpec {
                key: "endLedger",
                label: "End ledger",
                field_type: FieldType::Number,
                optional: true,
                default_value: "",
                hint: "Exclusive end ledger sequence",
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
                key: "type",
                label: "Event type",
                field_type: FieldType::Text,
                optional: true,
                default_value: "",
                hint: "'system' or 'contract'",
            },
            FieldSpec {
                key: "contractIds",
                label: "Contract IDs",
                field_type: FieldType::List,
                optional: true,
                default_value: "",
                hint: "Comma-separated contract IDs, up to 5",
            },
            FieldSpec {
                key: "topics",
                label: "Topics",
                field_type: FieldType::Json,
                optional: true,
                default_value: "",
                hint: "JSON array, up to 5",
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
        help: "Returns contract events.",
    }
}

/// Builds the JSON-RPC params object from form values
pub fn build(form: &FormState) -> Result<Value, String> {
    let start_ledger = parse_optional_u64(form, "startLedger")?;
    let end_ledger = parse_optional_u64(form, "endLedger")?;
    let cursor = parse_optional_string(form, "cursor");
    let limit = parse_optional_u64(form, "limit")?;
    let event_type = parse_optional_string(form, "type");
    let contract_ids = parse_list(form, "contractIds");
    let topics = parse_optional_json(form, "topics")?;
    let xdr_format = parse_optional_string(form, "xdrFormat");

    if cursor.is_some() && start_ledger.is_some() {
        return Err("Provide either startLedger or cursor, not both".to_string());
    }

    if contract_ids.len() > 5 {
        return Err("contractIds supports up to 5 values".to_string());
    }

    if let Some(Value::Array(values)) = &topics {
        if values.len() > 5 {
            return Err("topics supports up to 5 values".to_string());
        }
    }

    let event_type_enabled = event_type
        .as_deref()
        .map(|value| {
            let normalized = value.trim().to_ascii_lowercase();
            normalized == "system" || normalized == "contract"
        })
        .unwrap_or(false);

    // contractIds and topics are only meaningful when event type is explicitly constrained
    if (!contract_ids.is_empty() || topics.is_some()) && !event_type_enabled {
        return Err("Event type is required when using contractIds or topics".to_string());
    }

    let mut params = serde_json::Map::new();
    if let Some(value) = start_ledger {
        params.insert("startLedger".to_string(), json!(value));
    }
    if let Some(value) = end_ledger {
        params.insert("endLedger".to_string(), json!(value));
    }
    if let Some(value) = xdr_format {
        params.insert("xdrFormat".to_string(), json!(value));
    }

    if limit.is_some() || cursor.is_some() {
        let mut pagination = serde_json::Map::new();
        if let Some(value) = limit {
            pagination.insert("limit".to_string(), json!(value));
        }
        if let Some(value) = cursor {
            pagination.insert("cursor".to_string(), json!(value));
        }
        params.insert("pagination".to_string(), Value::Object(pagination));
    }

    if event_type.is_some() || !contract_ids.is_empty() || topics.is_some() {
        let mut filters = serde_json::Map::new();
        if let Some(value) = event_type {
            filters.insert("type".to_string(), json!(value));
        }
        if !contract_ids.is_empty() {
            filters.insert("contractIds".to_string(), json!(contract_ids));
        }
        if let Some(value) = topics {
            filters.insert("topics".to_string(), value);
        }
        // RPC expects filters as an array even for a single filter block
        params.insert(
            "filters".to_string(),
            Value::Array(vec![Value::Object(filters)]),
        );
    }

    Ok(Value::Object(params))
}
