use futures_util::FutureExt;
use std::sync::{Arc, Mutex};
use tokio_util::sync::CancellationToken;
use wharfkit_godot_plugin_sdk::await_main::{await_main_thread, AwaitMainError};

#[test]
fn await_main_thread_returns_pending_value_when_set() {
    let pending: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));
    let cancel = CancellationToken::new();
    let got = await_main_thread(pending, cancel)
        .now_or_never()
        .expect("fast path resolves immediately");
    assert_eq!(got, Ok(42));
}

#[test]
fn await_main_thread_returns_none_when_cancelled() {
    let pending: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(None));
    let cancel = CancellationToken::new();
    cancel.cancel();
    let got = await_main_thread(pending, cancel)
        .now_or_never()
        .expect("cancelled fast path resolves immediately");
    assert_eq!(got, Err(AwaitMainError::Cancelled));
}

#[test]
fn await_main_thread_returns_pending_value_even_when_cancelled() {
    let pending: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));
    let cancel = CancellationToken::new();
    cancel.cancel();
    let got = await_main_thread(pending, cancel)
        .now_or_never()
        .expect("pending fast path resolves immediately");
    assert_eq!(got, Ok(42));
}
