use godot::prelude::*;

use crate::dict;
use crate::prompt::PromptHandleView;

enum Inner {
    Object(Gd<godot::classes::Object>),
    Dict(Dictionary),
}

pub struct TransactContextView {
    inner: Inner,
}

impl TransactContextView {
    pub fn new(inner: Gd<godot::classes::Object>) -> Self {
        Self::from_object(inner)
    }

    pub fn from_object(obj: Gd<godot::classes::Object>) -> Self {
        Self {
            inner: Inner::Object(obj),
        }
    }

    pub fn from_dictionary(d: Dictionary) -> Self {
        Self {
            inner: Inner::Dict(d),
        }
    }

    pub fn from_variant(v: &Variant) -> Option<Self> {
        if let Ok(d) = v.try_to::<Dictionary>() {
            return Some(Self::from_dictionary(d));
        }
        v.try_to::<Gd<godot::classes::Object>>()
            .ok()
            .map(Self::from_object)
    }

    pub fn chain_id(&self) -> Option<String> {
        match &self.inner {
            Inner::Dict(d) => match dict::dict_dict(d, "chain") {
                Some(chain) => dict::dict_string(&chain, "id"),
                None => d
                    .get("chain")
                    .and_then(|v| v.try_to::<Gd<godot::classes::Object>>().ok())
                    .and_then(|mut o| {
                        o.call(&StringName::from("chain_id"), &[])
                            .try_to::<GString>()
                            .ok()
                            .map(|s| s.to_string())
                    }),
            },
            Inner::Object(o) => o
                .get(&StringName::from("chain_id"))
                .try_to::<GString>()
                .ok()
                .map(|s| s.to_string()),
        }
    }

    pub fn chain_url(&self) -> Option<String> {
        match &self.inner {
            Inner::Dict(d) => match dict::dict_dict(d, "chain") {
                Some(chain) => dict::dict_string(&chain, "url"),
                None => d
                    .get("chain")
                    .and_then(|v| v.try_to::<Gd<godot::classes::Object>>().ok())
                    .and_then(|mut o| {
                        o.call(&StringName::from("url_value"), &[])
                            .try_to::<GString>()
                            .ok()
                            .map(|s| s.to_string())
                    }),
            },
            Inner::Object(o) => o
                .get(&StringName::from("chain_url"))
                .try_to::<GString>()
                .ok()
                .map(|s| s.to_string()),
        }
    }

    pub fn permission_level(&self) -> Option<(String, String)> {
        match &self.inner {
            Inner::Dict(d) => {
                let perm = dict::dict_dict(d, "permission_level")?;
                let actor = dict::dict_string(&perm, "actor")?;
                let permission = dict::dict_string(&perm, "permission")?;
                Some((actor, permission))
            }
            Inner::Object(o) => {
                let actor = o
                    .get(&StringName::from("permission_actor"))
                    .try_to::<GString>()
                    .ok()
                    .map(|s| s.to_string())?;
                let permission = o
                    .get(&StringName::from("permission_permission"))
                    .try_to::<GString>()
                    .ok()
                    .map(|s| s.to_string())?;
                Some((actor, permission))
            }
        }
    }

    pub fn return_path(&self) -> Option<String> {
        match &self.inner {
            Inner::Dict(d) => dict::dict_string(d, "return_path"),
            Inner::Object(o) => o
                .get(&StringName::from("return_path"))
                .try_to::<GString>()
                .ok()
                .map(|s| s.to_string()),
        }
    }

    pub fn ui(&self) -> Option<Gd<godot::classes::Object>> {
        match &self.inner {
            Inner::Dict(d) => d
                .get("ui")
                .and_then(|v| v.try_to::<Gd<godot::classes::Object>>().ok()),
            Inner::Object(o) => o
                .get(&StringName::from("ui"))
                .try_to::<Gd<godot::classes::Object>>()
                .ok(),
        }
    }

    pub fn resolved_signing_request(&self) -> Option<Dictionary> {
        match &self.inner {
            Inner::Dict(d) => d
                .get("resolved_request")
                .and_then(|v| v.try_to::<Dictionary>().ok()),
            Inner::Object(o) => o
                .get(&StringName::from("resolved_request"))
                .try_to::<Dictionary>()
                .ok(),
        }
    }

    pub fn platform_name(&self) -> GString {
        match &self.inner {
            Inner::Dict(d) => dict::dict_string(d, "platform_name")
                .map(GString::from)
                .unwrap_or_default(),
            Inner::Object(o) => o
                .get(&StringName::from("platform_name"))
                .try_to::<GString>()
                .unwrap_or_default(),
        }
    }

    pub fn is_known_mobile(&self) -> bool {
        match &self.inner {
            Inner::Dict(d) => dict::dict_bool(d, "is_known_mobile").unwrap_or(false),
            Inner::Object(o) => {
                let mut obj = o.clone();
                obj.call(&StringName::from("is_known_mobile"), &[])
                    .try_to::<bool>()
                    .unwrap_or(false)
            }
        }
    }

    pub fn shell_open(&self, uri: &str) {
        if let Inner::Object(o) = &self.inner {
            let mut obj = o.clone();
            obj.call(
                &StringName::from("shell_open"),
                &[GString::from(uri).to_variant()],
            );
        }
    }

    pub fn request_prompt(&self, args: Gd<godot::classes::Object>) -> Option<PromptHandleView> {
        if let Inner::Object(o) = &self.inner {
            let mut obj = o.clone();
            let result = obj.call(&StringName::from("request_prompt"), &[args.to_variant()]);
            return result
                .try_to::<Gd<godot::classes::Object>>()
                .ok()
                .map(PromptHandleView::new);
        }
        None
    }

    pub fn as_object(&self) -> &Gd<godot::classes::Object> {
        self.try_as_object()
            .expect("TransactContextView is not Object-backed")
    }

    pub fn try_as_object(&self) -> Option<&Gd<godot::classes::Object>> {
        match &self.inner {
            Inner::Object(o) => Some(o),
            Inner::Dict(_) => None,
        }
    }
}
