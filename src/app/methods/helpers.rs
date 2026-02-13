use serde_json::Value;

use crate::app::core::forms::FormState;

/// Normalizes optional text input by trimming and collapsing empty values to None
pub(crate) fn parse_optional_string(form: &FormState, key: &str) -> Option<String> {
    form.field_value(key)
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
        .map(|value| value.to_string())
}

pub(crate) fn parse_required_string(form: &FormState, key: &str) -> Result<String, String> {
    parse_optional_string(form, key).ok_or_else(|| format!("{} is required", key))
}

pub(crate) fn parse_optional_u64(form: &FormState, key: &str) -> Result<Option<u64>, String> {
    match parse_optional_string(form, key) {
        Some(value) => value
            .parse::<u64>()
            .map(Some)
            .map_err(|_| format!("{} must be a number", key)),
        None => Ok(None),
    }
}

/// Preserves user provided order while dropping blank comma separated entries
pub(crate) fn parse_list(form: &FormState, key: &str) -> Vec<String> {
    form.field_value(key)
        .map(|value| {
            value
                .split(',')
                .map(str::trim)
                .filter(|item| !item.is_empty())
        })
        .into_iter()
        .flatten()
        .map(|item| item.to_string())
        .collect()
}

/// Parses JSON only when a non empty value exists so optional JSON fields stay absent by default
pub(crate) fn parse_optional_json(form: &FormState, key: &str) -> Result<Option<Value>, String> {
    match parse_optional_string(form, key) {
        Some(value) => serde_json::from_str(&value)
            .map(Some)
            .map_err(|err| format!("{} invalid JSON: {}", key, err)),
        None => Ok(None),
    }
}
