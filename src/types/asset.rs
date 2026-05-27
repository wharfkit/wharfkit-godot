use antelope::chain::asset::Asset;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct WharfkitAsset {
    #[var]
    string_value: GString,
    inner: Asset,
}

#[godot_api]
impl WharfkitAsset {
    #[func]
    pub fn from(s: GString) -> Gd<Self> {
        let inner = Asset::from_string(&s.to_string());
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
    pub fn amount(&self) -> i64 {
        self.inner.amount()
    }

    #[func]
    pub fn symbol_code(&self) -> GString {
        GString::from(self.inner.symbol().code().to_string())
    }

    // Held in reserve for cross-binding use.
    #[allow(dead_code)]
    pub fn inner(&self) -> Asset {
        self.inner
    }
}
