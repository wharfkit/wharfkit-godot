//! `await_main_thread<T>` is the generic version of Anchor's pending-slot wait.
//!
//! The fast paths are immediately ready in unit tests without a live `SceneTree`.
//! The slow path awaits `SceneTree::process_frame` between polls so Godot
//! wakes the future from the main thread.

use godot::classes::{Engine, SceneTree};
use std::sync::{Arc, Mutex};
use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AwaitMainError {
    Cancelled,
    NoSceneTree,
}

fn poll_pending_or_cancelled<T>(
    pending: &Arc<Mutex<Option<T>>>,
    cancel: &CancellationToken,
) -> Option<Result<T, AwaitMainError>> {
    if let Some(value) = pending.lock().expect("pending lock").take() {
        return Some(Ok(value));
    }
    if cancel.is_cancelled() {
        return Some(Err(AwaitMainError::Cancelled));
    }
    None
}

pub async fn await_main_thread<T: Send + 'static>(
    pending: Arc<Mutex<Option<T>>>,
    cancel: CancellationToken,
) -> Result<T, AwaitMainError> {
    if let Some(result) = poll_pending_or_cancelled(&pending, &cancel) {
        return result;
    }

    let scene_tree = Engine::singleton()
        .get_main_loop()
        .and_then(|ml| ml.try_cast::<SceneTree>().ok())
        .ok_or(AwaitMainError::NoSceneTree)?;

    loop {
        if let Some(result) = poll_pending_or_cancelled(&pending, &cancel) {
            return result;
        }
        scene_tree.signals().process_frame().to_future().await;
    }
}

#[cfg(test)]
mod tests {
    use super::{poll_pending_or_cancelled, AwaitMainError};
    use std::sync::{Arc, Mutex};
    use tokio_util::sync::CancellationToken;

    #[test]
    fn pending_wins_over_cancelled_token() {
        let pending: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(Some(42)));
        let cancel = CancellationToken::new();
        cancel.cancel();

        assert_eq!(poll_pending_or_cancelled(&pending, &cancel), Some(Ok(42)));
    }

    #[test]
    fn cancelled_token_returns_cancelled_when_pending_empty() {
        let pending: Arc<Mutex<Option<i32>>> = Arc::new(Mutex::new(None));
        let cancel = CancellationToken::new();
        cancel.cancel();

        assert_eq!(
            poll_pending_or_cancelled(&pending, &cancel),
            Some(Err(AwaitMainError::Cancelled))
        );
    }
}
