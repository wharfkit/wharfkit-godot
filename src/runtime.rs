use godot::classes::Engine;
use godot::prelude::*;
use std::sync::{Arc, OnceLock};
use tokio::runtime::{Builder, Handle, Runtime};

pub const SINGLETON_NAME: &str = "WharfkitRuntime";

static CACHED_HANDLE: OnceLock<Handle> = OnceLock::new();

#[derive(GodotClass)]
#[class(base=Object, no_init)]
pub struct WharfkitRuntime {
    runtime: Arc<Runtime>,
}

#[godot_api]
impl WharfkitRuntime {
    #[func]
    fn ping(&self) -> bool {
        let _ = self.runtime.handle();
        true
    }
}

impl WharfkitRuntime {
    fn create() -> Gd<Self> {
        let rt = Builder::new_multi_thread()
            .worker_threads(2)
            .enable_all()
            .build()
            .expect("tokio runtime");
        Gd::from_init_fn(|_base| Self {
            runtime: Arc::new(rt),
        })
    }

    pub fn register() {
        let mut engine = Engine::singleton();
        let name = StringName::from(SINGLETON_NAME);
        if engine.has_singleton(&name) {
            return;
        }
        let instance = Self::create();
        let _ = CACHED_HANDLE.set(instance.bind().runtime.handle().clone());
        engine.register_singleton(&name, &instance.upcast::<godot::classes::Object>());
    }

    pub fn unregister() {
        let mut engine = Engine::singleton();
        let name = StringName::from(SINGLETON_NAME);
        if let Some(instance) = engine.get_singleton(&name) {
            engine.unregister_singleton(&name);
            instance.free();
        }
    }

    pub fn handle() -> Handle {
        if let Some(h) = CACHED_HANDLE.get() {
            return h.clone();
        }
        let name = StringName::from(SINGLETON_NAME);
        let obj = Engine::singleton()
            .get_singleton(&name)
            .expect("WharfkitRuntime singleton must be registered before use");
        let gd: Gd<Self> = obj.cast::<Self>();
        let handle = gd.bind().runtime.handle().clone();
        let _ = CACHED_HANDLE.set(handle.clone());
        handle
    }

    pub fn block_on<F: std::future::Future>(future: F) -> F::Output {
        Self::handle().block_on(future)
    }
}
