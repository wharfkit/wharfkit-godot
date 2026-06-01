use godot::prelude::*;
use wharfkit_godot_plugin_sdk::dict;

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn dict_string_returns_some_for_string_value() {
    let mut d = Dictionary::new();
    d.set("name", GString::from("alice"));
    assert_eq!(dict::dict_string(&d, "name"), Some("alice".to_string()));
}

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn dict_string_returns_none_for_missing_key() {
    let d = Dictionary::new();
    assert_eq!(dict::dict_string(&d, "missing"), None);
}

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn dict_string_returns_none_for_wrong_type() {
    let mut d = Dictionary::new();
    d.set("count", 42_i64);
    assert_eq!(dict::dict_string(&d, "count"), None);
}

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn dict_i64_round_trips() {
    let mut d = Dictionary::new();
    d.set("count", 42_i64);
    assert_eq!(dict::dict_i64(&d, "count"), Some(42));
}

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn dict_bool_round_trips() {
    let mut d = Dictionary::new();
    d.set("flag", true);
    assert_eq!(dict::dict_bool(&d, "flag"), Some(true));
}

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn dict_dict_round_trips() {
    let mut inner = Dictionary::new();
    inner.set("key", GString::from("value"));
    let mut d = Dictionary::new();
    d.set("nested", inner);
    let got = dict::dict_dict(&d, "nested").expect("dict_dict returns Some");
    assert_eq!(dict::dict_string(&got, "key"), Some("value".to_string()));
}

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn dict_array_round_trips() {
    let mut arr: Array<Variant> = Array::new();
    arr.push(&GString::from("a").to_variant());
    arr.push(&GString::from("b").to_variant());
    let mut d = Dictionary::new();
    d.set("items", arr);
    let got = dict::dict_array(&d, "items").expect("dict_array returns Some");
    assert_eq!(got.len(), 2);
}
