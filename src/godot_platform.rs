use godot::classes::Os;
use godot::prelude::*;
use wharfkit_session::{Platform, PlatformName};

#[allow(dead_code)]
pub struct GodotPlatform;

impl Platform for GodotPlatform {
    fn name(&self) -> PlatformName {
        let os_name = Os::singleton().get_name().to_string();
        match os_name.as_str() {
            "macOS" => PlatformName::Macos,
            "Windows" | "UWP" => PlatformName::Windows,
            "iOS" => PlatformName::IOS,
            "Android" => PlatformName::Android,
            "Web" => PlatformName::Web,
            "Linux" | "FreeBSD" | "NetBSD" | "OpenBSD" | "BSD" => PlatformName::Linux,
            _ => PlatformName::Headless,
        }
    }

    fn shell_open(&self, uri: &str) {
        let mut os = Os::singleton();
        let _ = os.shell_open(&GString::from(uri));
    }
}
