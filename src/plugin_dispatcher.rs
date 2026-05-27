use godot::classes::{Engine, Object};
use godot::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::godot_error::WharfkitError;
use crate::prompt_types::{ContextShared, WharfkitPromptArgs, WharfkitPromptHandle};

const GODOT_EMIT_SIGNAL: &str = "emit_signal";
const GODOT_CALL: &str = "call";
const PROMPT_METHOD: &str = "_prompt";
const PROMPT_DONE: &str = "prompt_done";
const PROMPT_FAILED: &str = "prompt_failed";

#[allow(dead_code)]
pub fn validate_method(target: &Gd<Object>, method: &str) -> bool {
    target.has_method(&StringName::from(method))
}

pub struct OutputSignals<'a> {
    pub done: &'a str,
    pub failed: &'a str,
}

type Connection = (Gd<Object>, StringName, Callable);

#[allow(clippy::too_many_arguments)]
pub fn dispatch_into(
    target: Gd<Object>,
    method_name: &str,
    args: Vec<Variant>,
    target_done: &str,
    target_failed: &str,
    output: Gd<Object>,
    output_signals: OutputSignals<'_>,
    cancel_emitter: Option<(Gd<Object>, &str)>,
    timeout_emitter: Option<(Gd<Object>, &str)>,
) {
    let completed = Arc::new(AtomicBool::new(false));
    let connections: Rc<RefCell<Vec<Connection>>> = Rc::new(RefCell::new(Vec::new()));

    let done_signal_out = StringName::from(output_signals.done);
    let failed_signal_out = StringName::from(output_signals.failed);

    let make_forward = |signal_out: StringName,
                        output: Gd<Object>,
                        completed: Arc<AtomicBool>,
                        connections: Rc<RefCell<Vec<Connection>>>,
                        debug_name: &str|
     -> Callable {
        let name = debug_name.to_string();
        Callable::from_local_fn(&name, move |args| {
            if completed.swap(true, Ordering::SeqCst) {
                return Ok(Variant::nil());
            }
            let payload = args.first().map(|v| (*v).clone()).unwrap_or_default();
            let mut output = output.clone();
            let _ = output.call_deferred(
                &StringName::from(GODOT_EMIT_SIGNAL),
                &[signal_out.to_variant(), payload],
            );
            disconnect_all(&connections);
            Ok(Variant::nil())
        })
    };

    let fwd_done = make_forward(
        done_signal_out,
        output.clone(),
        completed.clone(),
        connections.clone(),
        "wharfkit_fwd_done",
    );
    let fwd_failed = make_forward(
        failed_signal_out.clone(),
        output.clone(),
        completed.clone(),
        connections.clone(),
        "wharfkit_fwd_failed",
    );

    let mut target_mut = target.clone();
    let done_name = StringName::from(target_done);
    let failed_name = StringName::from(target_failed);
    target_mut.connect(&done_name, &fwd_done);
    target_mut.connect(&failed_name, &fwd_failed);
    {
        let mut conns = connections.borrow_mut();
        conns.push((target_mut.clone(), done_name, fwd_done));
        conns.push((target_mut.clone(), failed_name, fwd_failed));
    }

    if let Some((mut emitter, signal_name)) = cancel_emitter {
        let cb = make_synthetic_failure(
            "wharfkit_dispatch_cancel",
            failed_signal_out.clone(),
            output.clone(),
            completed.clone(),
            connections.clone(),
            WharfkitError::cancelled(GString::from("operation cancelled")),
        );
        let signal = StringName::from(signal_name);
        emitter.connect(&signal, &cb);
        connections.borrow_mut().push((emitter, signal, cb));
    }

    if let Some((mut emitter, signal_name)) = timeout_emitter {
        let cb = make_synthetic_failure(
            "wharfkit_dispatch_timeout",
            failed_signal_out,
            output.clone(),
            completed.clone(),
            connections.clone(),
            WharfkitError::expired(GString::from("operation timed out")),
        );
        let signal = StringName::from(signal_name);
        emitter.connect(&signal, &cb);
        connections.borrow_mut().push((emitter, signal, cb));
    }

    let mut deferred_args: Vec<Variant> = Vec::with_capacity(args.len() + 1);
    deferred_args.push(StringName::from(method_name).to_variant());
    deferred_args.extend(args);
    let mut t = target;
    let _ = t.call_deferred(&StringName::from(GODOT_CALL), &deferred_args);
}

fn make_synthetic_failure(
    debug_name: &str,
    signal: StringName,
    output: Gd<Object>,
    completed: Arc<AtomicBool>,
    connections: Rc<RefCell<Vec<Connection>>>,
    err: Gd<WharfkitError>,
) -> Callable {
    let name = debug_name.to_string();
    let err_variant = err.to_variant();
    Callable::from_local_fn(&name, move |_args| {
        if completed.swap(true, Ordering::SeqCst) {
            return Ok(Variant::nil());
        }
        let mut output = output.clone();
        let _ = output.call_deferred(
            &StringName::from(GODOT_EMIT_SIGNAL),
            &[signal.to_variant(), err_variant.clone()],
        );
        disconnect_all(&connections);
        Ok(Variant::nil())
    })
}

fn disconnect_all(connections: &Rc<RefCell<Vec<Connection>>>) {
    let drained: Vec<Connection> = std::mem::take(&mut *connections.borrow_mut());
    for (mut emitter, signal, callable) in drained {
        if emitter.is_connected(&signal, &callable) {
            emitter.disconnect(&signal, &callable);
        }
    }
}

pub fn route_prompt(
    shared: Arc<ContextShared>,
    ui: Option<Gd<Object>>,
    args: Gd<WharfkitPromptArgs>,
) -> Gd<WharfkitPromptHandle> {
    let mut handle = WharfkitPromptHandle::new_gd();

    if !shared.try_begin_prompt() {
        let err = WharfkitError::internal(GString::from("prompt already in flight"));
        let signal = StringName::from(PROMPT_FAILED).to_variant();
        let _ = handle.call_deferred(
            &StringName::from(GODOT_EMIT_SIGNAL),
            &[signal, err.to_variant()],
        );
        return handle;
    }

    let Some(ui) = ui else {
        let err = WharfkitError::internal(GString::from(
            "no UserInterface registered on the SessionKit",
        ));
        shared.end_prompt();
        let signal = StringName::from(PROMPT_FAILED).to_variant();
        let _ = handle.call_deferred(
            &StringName::from(GODOT_EMIT_SIGNAL),
            &[signal, err.to_variant()],
        );
        return handle;
    };

    let output: Gd<Object> = handle.clone().upcast::<Object>();

    {
        let mut h = handle.clone();
        let shared = shared.clone();
        let cb = Callable::from_local_fn("wharfkit_prompt_cleanup", move |_args| {
            shared.end_prompt();
            Ok(Variant::nil())
        });
        h.connect(&StringName::from(PROMPT_DONE), &cb);
        h.connect(&StringName::from(PROMPT_FAILED), &cb);
    }

    dispatch_into(
        ui,
        PROMPT_METHOD,
        vec![args.to_variant()],
        PROMPT_DONE,
        PROMPT_FAILED,
        output,
        OutputSignals {
            done: PROMPT_DONE,
            failed: PROMPT_FAILED,
        },
        None,
        None,
    );

    handle
}

#[allow(dead_code)]
pub fn runtime_singleton_present() -> bool {
    Engine::singleton().has_singleton(&StringName::from(crate::runtime::SINGLETON_NAME))
}
