use antelope::chain::name::Name;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct WharfkitName {
    #[var]
    string_value: GString,
    inner: Name,
}

#[godot_api]
impl WharfkitName {
    #[func]
    pub fn from(s: GString) -> Gd<Self> {
        let inner = Name::new_from_str(&s.to_string());
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
    pub fn equals(&self, other: Gd<WharfkitName>) -> bool {
        self.inner == other.bind().inner
    }

    pub fn inner(&self) -> Name {
        self.inner
    }
}
