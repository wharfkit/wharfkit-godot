use godot::classes::ClassDb;
use godot::prelude::*;

const PROMPT_ARGS_CLASS: &str = "WharfkitPromptArgs";
const PROMPT_ELEMENT_CLASS: &str = "WharfkitPromptElement";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptBuildError {
    class_name: String,
}

impl PromptBuildError {
    pub fn missing_class(class_name: impl Into<String>) -> Self {
        Self {
            class_name: class_name.into(),
        }
    }

    pub fn class_name(&self) -> &str {
        &self.class_name
    }
}

impl std::fmt::Display for PromptBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "core addon not registered: {}", self.class_name)
    }
}

impl std::error::Error for PromptBuildError {}

fn instantiate_object(class_name: &str) -> Result<Gd<godot::classes::Object>, PromptBuildError> {
    let variant = ClassDb::singleton().instantiate(&StringName::from(class_name));
    variant
        .try_to::<Gd<godot::classes::Object>>()
        .map_err(|_| PromptBuildError::missing_class(class_name))
}

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

    pub fn try_build(self) -> Result<Gd<godot::classes::Object>, PromptBuildError> {
        let mut obj = instantiate_object(PROMPT_ARGS_CLASS)?;

        obj.set(
            &StringName::from("title"),
            &GString::from(self.title).to_variant(),
        );
        if let Some(body) = self.body {
            obj.set(&StringName::from("body"), &GString::from(body).to_variant());
        }
        obj.set(&StringName::from("optional"), &self.optional.to_variant());
        obj.set(&StringName::from("elements"), &self.elements.to_variant());

        Ok(obj)
    }

    pub fn build(self) -> Gd<godot::classes::Object> {
        self.try_build()
            .expect("ClassDb::instantiate(\"WharfkitPromptArgs\") -> Object")
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptElementKind {
    Qr = 0,
    Link = 1,
    Button = 2,
    Countdown = 3,
    Accept = 4,
    Close = 5,
}

impl PromptElementKind {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Qr),
            1 => Some(Self::Link),
            2 => Some(Self::Button),
            3 => Some(Self::Countdown),
            4 => Some(Self::Accept),
            5 => Some(Self::Close),
            _ => None,
        }
    }
}

pub struct PromptElementBuilder {
    kind: PromptElementKind,
    id: Option<String>,
    label: Option<String>,
    href: Option<String>,
    data: Option<String>,
    end_unix_ms: Option<i64>,
}

impl PromptElementBuilder {
    fn empty(kind: PromptElementKind) -> Self {
        Self {
            kind,
            id: None,
            label: None,
            href: None,
            data: None,
            end_unix_ms: None,
        }
    }

    pub fn qr(data: impl Into<String>) -> Self {
        let mut b = Self::empty(PromptElementKind::Qr);
        b.data = Some(data.into());
        b
    }

    pub fn link(id: impl Into<String>, href: impl Into<String>, label: impl Into<String>) -> Self {
        let mut b = Self::empty(PromptElementKind::Link);
        b.id = Some(id.into());
        b.href = Some(href.into());
        b.label = Some(label.into());
        b
    }

    pub fn button(id: impl Into<String>, label: impl Into<String>) -> Self {
        let mut b = Self::empty(PromptElementKind::Button);
        b.id = Some(id.into());
        b.label = Some(label.into());
        b
    }

    pub fn countdown(id: impl Into<String>, label: impl Into<String>, end_unix_ms: i64) -> Self {
        let mut b = Self::empty(PromptElementKind::Countdown);
        b.id = Some(id.into());
        b.label = Some(label.into());
        b.end_unix_ms = Some(end_unix_ms);
        b
    }

    pub fn accept(label: impl Into<String>) -> Self {
        let mut b = Self::empty(PromptElementKind::Accept);
        b.label = Some(label.into());
        b
    }

    pub fn close() -> Self {
        Self::empty(PromptElementKind::Close)
    }

    pub fn kind(&self) -> PromptElementKind {
        self.kind
    }

    pub fn id(&self) -> Option<String> {
        self.id.clone()
    }

    pub fn label(&self) -> Option<String> {
        self.label.clone()
    }

    pub fn href(&self) -> Option<String> {
        self.href.clone()
    }

    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }

    pub fn end_unix_ms(&self) -> Option<i64> {
        self.end_unix_ms
    }

    pub fn try_build(self) -> Result<Gd<godot::classes::Object>, PromptBuildError> {
        let mut obj = instantiate_object(PROMPT_ELEMENT_CLASS)?;

        obj.set(&StringName::from("kind"), &(self.kind as i32).to_variant());
        if let Some(id) = self.id {
            obj.set(&StringName::from("id"), &GString::from(id).to_variant());
        }
        if let Some(label) = self.label {
            obj.set(
                &StringName::from("label"),
                &GString::from(label).to_variant(),
            );
        }
        if let Some(href) = self.href {
            obj.set(&StringName::from("href"), &GString::from(href).to_variant());
        }
        if let Some(data) = self.data {
            obj.set(&StringName::from("data"), &GString::from(data).to_variant());
        }
        if let Some(end) = self.end_unix_ms {
            obj.set(&StringName::from("end_unix_ms"), &end.to_variant());
        }
        Ok(obj)
    }

    pub fn build(self) -> Gd<godot::classes::Object> {
        self.try_build()
            .expect("ClassDb::instantiate(\"WharfkitPromptElement\") -> Object")
    }
}

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptResponseKind {
    Accepted = 0,
    Closed = 1,
    Expired = 2,
    ButtonPressed = 3,
    LinkOpened = 4,
}

impl PromptResponseKind {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Accepted),
            1 => Some(Self::Closed),
            2 => Some(Self::Expired),
            3 => Some(Self::ButtonPressed),
            4 => Some(Self::LinkOpened),
            _ => None,
        }
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

    pub fn kind_typed(&self) -> Option<PromptResponseKind> {
        PromptResponseKind::from_i32(self.kind())
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
