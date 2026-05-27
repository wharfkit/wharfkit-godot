use godot::init::{ExtensionLibrary, InitLevel};
use godot::prelude::*;

mod base_classes;
#[allow(dead_code)]
mod generated;
mod godot_error;
mod godot_platform;
mod godot_storage;
mod plugin_dispatcher;
mod prompt_types;
mod qr_code;
mod runtime;
mod types;

struct WharfKitExtension;

#[gdextension]
unsafe impl ExtensionLibrary for WharfKitExtension {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            runtime::WharfkitRuntime::register();
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            runtime::WharfkitRuntime::unregister();
        }
    }
}
