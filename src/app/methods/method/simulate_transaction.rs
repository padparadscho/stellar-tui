//! Stellar RPC method simulateTransaction

use serde_json::{json, Value};

use crate::app::core::forms::{FieldSpec, FieldType, FormState};
use crate::app::methods::{
    parse_optional_string, parse_optional_u64, parse_required_string, MethodId, MethodSpec,
};

/// Returns the method specification
pub fn spec() -> MethodSpec {
    MethodSpec {
        id: MethodId::SimulateTransaction,
        name: "simulateTransaction",
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
                key: "instructionLeeway",
                label: "Instruction leeway",
                field_type: FieldType::Number,
                optional: true,
                default_value: "",
                hint: "Optional instruction leeway",
            },
            FieldSpec {
                key: "authMode",
                label: "Auth mode",
                field_type: FieldType::Text,
                optional: true,
                default_value: "enforce",
                hint: "'enforce' (default), 'record' or 'record_allow_nonroot'",
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
        help: "Submits a trial contract invocation transaction.",
    }
}

/// Builds the JSON-RPC params object from form values
pub fn build(form: &FormState) -> Result<Value, String> {
    let tx = parse_required_string(form, "transaction")?;
    let instruction_leeway = parse_optional_u64(form, "instructionLeeway")?;
    let auth_mode = parse_optional_string(form, "authMode");
    let xdr_format = parse_optional_string(form, "xdrFormat");

    let mut params = serde_json::Map::new();
    params.insert("transaction".to_string(), json!(tx));

    if instruction_leeway.is_some() {
        // RPC expects instructionLeeway nested under resourceConfig
        let mut resource_config = serde_json::Map::new();
        if let Some(value) = instruction_leeway {
            resource_config.insert("instructionLeeway".to_string(), json!(value));
        }
        params.insert("resourceConfig".to_string(), Value::Object(resource_config));
    }

    if let Some(value) = auth_mode {
        params.insert("authMode".to_string(), json!(value));
    }
    if let Some(value) = xdr_format {
        params.insert("xdrFormat".to_string(), json!(value));
    }

    Ok(Value::Object(params))
}
