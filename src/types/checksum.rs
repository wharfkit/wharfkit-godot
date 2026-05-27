use antelope::chain::checksum::Checksum256;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct WharfkitChecksum256 {
    #[var]
    hex_value: GString,
    inner: Checksum256,
}

#[godot_api]
impl WharfkitChecksum256 {
    #[func]
    pub fn from_hex(s: GString) -> Gd<Self> {
        let inner = Checksum256::from_hex(&s.to_string()).expect("invalid Checksum256 hex");
        Gd::from_init_fn(|_| Self {
            hex_value: s,
            inner,
        })
    }

    #[func]
    pub fn to_hex(&self) -> GString {
        self.hex_value.clone()
    }

    // Held in reserve for cross-binding use.
    #[allow(dead_code)]
    pub fn inner(&self) -> Checksum256 {
        self.inner
    }
}
