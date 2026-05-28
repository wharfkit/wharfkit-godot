use godot::classes::ClassDb;
use godot::prelude::*;

const PROMPT_ARGS_CLASS: &str = "WharfkitPromptArgs";

pub struct PromptArgsBuilder {
    title: String,
    body: Option<String>,
    optional: bool,
    elements: Array<Variant>,
}

impl Default for PromptArgsBuilder {
    fn default() -> Self {
        Self {
            title: String::new(),
            body: None,
            optional: false,
            elements: Array::new(),
        }
    }
}

impl PromptArgsBuilder {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            ..Self::default()
        }
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }

    pub fn optional(mut self, optional: bool) -> Self {
        self.optional = optional;
        self
    }

    pub fn push_element(mut self, element: Gd<godot::classes::Object>) -> Self {
        self.elements.push(&element.to_variant());
        self
    }

    pub fn build(self) -> Gd<godot::classes::Object> {
        let class_name = StringName::from(PROMPT_ARGS_CLASS);
        let variant = ClassDb::singleton().instantiate(&class_name);
        let mut obj: Gd<godot::classes::Object> = variant
            .try_to::<Gd<godot::classes::Object>>()
            .expect("ClassDb::instantiate(\"WharfkitPromptArgs\") -> Object");

        obj.set(
            &StringName::from("title"),
            &GString::from(self.title).to_variant(),
        );
        if let Some(body) = self.body {
            obj.set(&StringName::from("body"), &GString::from(body).to_variant());
        }
        obj.set(&StringName::from("optional"), &self.optional.to_variant());
        obj.set(&StringName::from("elements"), &self.elements.to_variant());

        obj
    }
}

pub struct PromptResponseView {
    inner: Gd<godot::classes::Object>,
}

impl PromptResponseView {
    pub fn new(inner: Gd<godot::classes::Object>) -> Self {
        Self { inner }
    }

    pub fn from_variant(v: &Variant) -> Option<Self> {
        v.try_to::<Gd<godot::classes::Object>>().ok().map(Self::new)
    }

    pub fn kind(&self) -> i32 {
        self.inner
            .get(&StringName::from("kind"))
            .try_to::<i32>()
            .unwrap_or(0)
    }

    pub fn id(&self) -> GString {
        self.inner
            .get(&StringName::from("id"))
            .try_to::<GString>()
            .unwrap_or_default()
    }

    pub fn as_object(&self) -> &Gd<godot::classes::Object> {
        &self.inner
    }
}

pub struct PromptHandleView {
    inner: Gd<godot::classes::Object>,
}

impl PromptHandleView {
    pub fn new(inner: Gd<godot::classes::Object>) -> Self {
        Self { inner }
    }

    pub fn as_object(&self) -> &Gd<godot::classes::Object> {
        &self.inner
    }

    pub fn into_object(self) -> Gd<godot::classes::Object> {
        self.inner
    }
}
