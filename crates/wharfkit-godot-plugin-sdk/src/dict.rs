//! Free helpers for reading typed values out of `Dictionary` payloads.
//!
//! Each helper returns `Option<T>` so callers choose between defaults,
//! propagation, or explicit missing-value handling.

use godot::prelude::*;

pub fn dict_string(d: &Dictionary, key: &str) -> Option<String> {
    d.get(key)
        .and_then(|v| v.try_to::<GString>().ok())
        .map(|s| s.to_string())
}

pub fn dict_i64(d: &Dictionary, key: &str) -> Option<i64> {
    d.get(key).and_then(|v| v.try_to::<i64>().ok())
}

pub fn dict_bool(d: &Dictionary, key: &str) -> Option<bool> {
    d.get(key).and_then(|v| v.try_to::<bool>().ok())
}

pub fn dict_dict(d: &Dictionary, key: &str) -> Option<Dictionary> {
    d.get(key).and_then(|v| v.try_to::<Dictionary>().ok())
}

pub fn dict_array(d: &Dictionary, key: &str) -> Option<Array<Variant>> {
    d.get(key).and_then(|v| v.try_to::<Array<Variant>>().ok())
}
