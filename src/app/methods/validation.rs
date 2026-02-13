use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::Value;
use std::collections::HashMap;

use crate::app::core::forms::{FieldType, FormState};

use super::{parse_list, parse_optional_json, parse_optional_string, MethodId, MethodSpec};

pub(crate) fn validate_request_form(
    spec: &MethodSpec,
    form: &FormState,
) -> HashMap<String, String> {
    let mut errors = HashMap::new();

    // Generic field type validation shared by all methods
    for field in &spec.fields {
        let raw = form.field_value(field.key).unwrap_or("");
        let trimmed = raw.trim();
        if !field.optional && trimmed.is_empty() {
            errors.insert(field.key.to_string(), "Required".to_string());
            continue;
        }

        if trimmed.is_empty() {
            continue;
        }

        match field.field_type {
            FieldType::Number => {
                if trimmed.parse::<u64>().is_err() {
                    errors.insert(field.key.to_string(), "Must be a number".to_string());
                }
            }
            FieldType::Json => {
                if serde_json::from_str::<Value>(trimmed).is_err() {
                    errors.insert(field.key.to_string(), "Invalid JSON".to_string());
                }
            }
            _ => {}
        }
    }

    match spec.id {
        MethodId::GetTransaction => {
            if let Some(hash) = parse_optional_string(form, "hash") {
                if !is_hex_64(&hash) {
                    errors.insert("hash".to_string(), "Must be 64-char hex".to_string());
                }
            }
        }
        MethodId::SendTransaction | MethodId::SimulateTransaction => {
            if let Some(tx) = parse_optional_string(form, "transaction") {
                if !is_base64(&tx) {
                    errors.insert("transaction".to_string(), "Must be base64".to_string());
                }
            }
        }
        MethodId::GetLedgerEntries => {
            for key in parse_list(form, "keys") {
                if !is_base64(&key) {
                    errors.insert("keys".to_string(), "All keys must be base64".to_string());
                    break;
                }
            }
        }
        _ => {}
    }

    // Shared pagination exclusivity for list style endpoints
    match spec.id {
        MethodId::GetEvents | MethodId::GetLedgers | MethodId::GetTransactions => {
            let start = parse_optional_string(form, "startLedger");
            let cursor = parse_optional_string(form, "cursor");
            if start.is_some() && cursor.is_some() {
                errors.insert(
                    "startLedger".to_string(),
                    "Use startLedger or cursor".to_string(),
                );
                errors.insert(
                    "cursor".to_string(),
                    "Use cursor or startLedger".to_string(),
                );
            }
        }
        _ => {}
    }

    // Keeps unusually wide ledger windows visible before request execution
    match spec.id {
        MethodId::GetEvents | MethodId::GetLedgers | MethodId::GetTransactions => {
            if let (Some(start_str), Some(end_str)) = (
                parse_optional_string(form, "startLedger"),
                parse_optional_string(form, "endLedger"),
            ) {
                if let (Ok(start), Ok(end)) = (start_str.parse::<u64>(), end_str.parse::<u64>()) {
                    if end > start && end - start > 100 {
                        errors.insert(
                            "endLedger".to_string(),
                            "Large range may slow response".to_string(),
                        );
                    }
                }
            }
        }
        _ => {}
    }

    if let Some(value) = parse_optional_string(form, "xdrFormat") {
        if value != "base64" && value != "json" {
            errors.insert("xdrFormat".to_string(), "Use base64 or json".to_string());
        }
    }

    if let Some(value) = parse_optional_string(form, "authMode") {
        let allowed = ["enforce", "record", "record_allow_nonroot"];
        if !allowed.contains(&value.as_str()) {
            errors.insert("authMode".to_string(), "Invalid auth mode".to_string());
        }
    }

    if let Some(value) = parse_optional_string(form, "type") {
        if value != "system" && value != "contract" {
            errors.insert("type".to_string(), "Use system or contract".to_string());
        }
    }

    if spec.id == MethodId::GetEvents {
        let contract_ids = parse_list(form, "contractIds");
        if contract_ids.len() > 5 {
            errors.insert("contractIds".to_string(), "Up to 5 values".to_string());
        }

        if let Ok(Some(Value::Array(topics))) = parse_optional_json(form, "topics") {
            if topics.len() > 5 {
                errors.insert("topics".to_string(), "Up to 5 values".to_string());
            }
        }

        let has_event_type = parse_optional_string(form, "type").is_some();
        let has_contract_ids = !contract_ids.is_empty();
        let has_topics = parse_optional_string(form, "topics").is_some();
        if (has_contract_ids || has_topics) && !has_event_type {
            errors.insert(
                "type".to_string(),
                "Required when contractIds/topics are set".to_string(),
            );
        }
    }

    if let Some(value) = parse_optional_string(form, "limit") {
        if let Ok(limit) = value.parse::<u64>() {
            if limit == 0 || limit > 10000 {
                errors.insert("limit".to_string(), "Range 1-10000".to_string());
            }
        }
    }

    errors
}

fn is_hex_64(value: &str) -> bool {
    if value.len() != 64 {
        return false;
    }
    value.chars().all(|ch| ch.is_ascii_hexdigit())
}

fn is_base64(value: &str) -> bool {
    STANDARD.decode(value.as_bytes()).is_ok()
}
