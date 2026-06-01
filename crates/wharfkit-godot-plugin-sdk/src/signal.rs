//! Deferred signal emission helpers.
//!
//! Use these when worker tasks need Godot signal delivery on a later main-thread
//! frame via `Object.call_deferred("emit_signal", ...)`.

use godot::prelude::*;

pub fn emit_signal_deferred(target: Gd<godot::classes::Object>, signal: &str, payload: Variant) {
    emit_signal_deferred_args(target, signal, &[payload]);
}

pub fn emit_signal_deferred_args(
    target: Gd<godot::classes::Object>,
    signal: &str,
    args: &[Variant],
) {
    let mut target = target;
    let mut all: Vec<Variant> = Vec::with_capacity(args.len() + 1);
    all.push(StringName::from(signal).to_variant());
    all.extend_from_slice(args);
    let _ = target.call_deferred(&StringName::from("emit_signal"), &all);
}
