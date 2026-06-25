//! Small helpers for cursor-based Kalshi pagination.

use serde_json::Value;

/// Extracts a non-empty `cursor` field from a response JSON value.
pub fn cursor(value: &Value) -> Option<&str> {
    value
        .get("cursor")
        .and_then(Value::as_str)
        .filter(|cursor| !cursor.is_empty())
}

/// Adds or replaces a `cursor` field on a JSON query object.
///
/// This helper is mostly useful for generic pagination code. Typed query params in
/// [`crate::params`] should be preferred when a generated params struct exists.
pub fn with_cursor(mut query: Value, cursor: impl Into<String>) -> Value {
    let cursor = cursor.into();
    match &mut query {
        Value::Object(map) => {
            map.insert("cursor".to_owned(), Value::String(cursor));
            query
        }
        _ => serde_json::json!({ "cursor": cursor }),
    }
}
