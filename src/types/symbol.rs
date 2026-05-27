use antelope::chain::asset::{Symbol, SymbolCode};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct WharfkitSymbol {
    #[var]
    string_value: GString,
    inner: Symbol,
}

#[godot_api]
impl WharfkitSymbol {
    #[func]
    pub fn from(s: GString) -> Gd<Self> {
        let raw = s.to_string();
        let parts: Vec<&str> = raw.splitn(2, ',').collect();
        assert!(
            parts.len() == 2,
            "invalid Symbol: expected '<precision>,<code>'"
        );
        let precision: u8 = parts[0].parse().expect("symbol precision must be u8");
        let _ = SymbolCode::new(parts[1]);
        let inner = Symbol::new(parts[1], precision);
        Gd::from_init_fn(|_| Self {
            string_value: s,
            inner,
        })
    }

    #[func]
    pub fn to_string_value(&self) -> GString {
        self.string_value.clone()
    }

    #[func]
    pub fn precision(&self) -> u8 {
        self.inner.precision() as u8
    }

    #[func]
    pub fn code(&self) -> GString {
        GString::from(self.inner.code().to_string())
    }

    // Held in reserve for cross-binding use.
    #[allow(dead_code)]
    pub fn inner(&self) -> Symbol {
        self.inner
    }
}
