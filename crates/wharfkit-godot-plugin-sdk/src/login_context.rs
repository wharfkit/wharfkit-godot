use godot::prelude::*;

use crate::prompt::PromptHandleView;

pub struct LoginContextView {
    inner: Gd<godot::classes::Object>,
}

impl LoginContextView {
    pub fn new(inner: Gd<godot::classes::Object>) -> Self {
        Self { inner }
    }

    pub fn from_variant(v: &Variant) -> Option<Self> {
        v.try_to::<Gd<godot::classes::Object>>().ok().map(Self::new)
    }

    pub fn platform_name(&self) -> GString {
        self.inner
            .get(&StringName::from("platform_name"))
            .try_to::<GString>()
            .unwrap_or_default()
    }

    pub fn is_known_mobile(&self) -> bool {
        let mut obj = self.inner.clone();
        obj.call(&StringName::from("is_known_mobile"), &[])
            .try_to::<bool>()
            .unwrap_or(false)
    }

    pub fn shell_open(&self, uri: &str) {
        let mut obj = self.inner.clone();
        obj.call(
            &StringName::from("shell_open"),
            &[GString::from(uri).to_variant()],
        );
    }

    pub fn arbitrary(&self) -> Dictionary {
        self.inner
            .get(&StringName::from("arbitrary"))
            .try_to::<Dictionary>()
            .unwrap_or_default()
    }

    pub fn request_prompt(&self, args: Gd<godot::classes::Object>) -> Option<PromptHandleView> {
        let mut obj = self.inner.clone();
        let result = obj.call(&StringName::from("request_prompt"), &[args.to_variant()]);
        result
            .try_to::<Gd<godot::classes::Object>>()
            .ok()
            .map(PromptHandleView::new)
    }

    pub fn as_object(&self) -> &Gd<godot::classes::Object> {
        &self.inner
    }
}
