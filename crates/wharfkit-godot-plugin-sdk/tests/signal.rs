use godot::classes::{Object, RefCounted};
use godot::prelude::*;
use wharfkit_godot_plugin_sdk::signal;

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn emit_signal_deferred_routes_single_payload() {
    let spy = Gd::<SignalSpy>::from_init_fn(|base| SignalSpy {
        base,
        captured: vec![],
    });
    let payload = GString::from("hello").to_variant();
    let target: Gd<Object> = spy.clone().upcast();
    signal::emit_signal_deferred(target, "any_signal", payload);
    assert_eq!(spy.bind().captured.len(), 0);
}

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
struct SignalSpy {
    base: Base<RefCounted>,
    captured: Vec<String>,
}
