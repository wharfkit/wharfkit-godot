use godot::classes::Object;
use godot::prelude::*;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;
use wharfkit_session::Platform;

use crate::godot_platform::GodotPlatform;

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[godot(via = i32)]
pub enum PromptElementKind {
    #[default]
    Qr = 0,
    Link = 1,
    Button = 2,
    Countdown = 3,
    Accept = 4,
    Close = 5,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct WharfkitPromptElement {
    #[var]
    pub kind: PromptElementKind,
    #[var]
    pub id: GString,
    #[var]
    pub label: GString,
    #[var]
    pub href: GString,
    #[var]
    pub data: GString,
    #[var]
    pub end_unix_ms: i64,
    base: Base<Resource>,
}

#[godot_api]
impl WharfkitPromptElement {
    fn make(kind: PromptElementKind) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            kind,
            id: GString::new(),
            label: GString::new(),
            href: GString::new(),
            data: GString::new(),
            end_unix_ms: 0,
            base,
        })
    }

    #[func]
    pub fn qr(data: GString) -> Gd<Self> {
        let mut el = Self::make(PromptElementKind::Qr);
        el.bind_mut().data = data;
        el
    }

    #[func]
    pub fn link(id: GString, href: GString, label: GString) -> Gd<Self> {
        let mut el = Self::make(PromptElementKind::Link);
        {
            let mut b = el.bind_mut();
            b.id = id;
            b.href = href;
            b.label = label;
        }
        el
    }

    #[func]
    pub fn button(id: GString, label: GString) -> Gd<Self> {
        let mut el = Self::make(PromptElementKind::Button);
        {
            let mut b = el.bind_mut();
            b.id = id;
            b.label = label;
        }
        el
    }

    #[func]
    pub fn countdown(id: GString, label: GString, end_unix_ms: i64) -> Gd<Self> {
        let mut el = Self::make(PromptElementKind::Countdown);
        {
            let mut b = el.bind_mut();
            b.id = id;
            b.label = label;
            b.end_unix_ms = end_unix_ms;
        }
        el
    }

    #[func]
    pub fn accept(label: GString) -> Gd<Self> {
        let mut el = Self::make(PromptElementKind::Accept);
        el.bind_mut().label = label;
        el
    }

    #[func]
    pub fn close() -> Gd<Self> {
        Self::make(PromptElementKind::Close)
    }
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct WharfkitPromptArgs {
    #[var]
    pub title: GString,
    #[var]
    pub body: GString,
    #[var]
    pub optional: bool,
    #[var]
    pub elements: Array<Gd<WharfkitPromptElement>>,
    base: Base<Resource>,
}

#[godot_api]
impl WharfkitPromptArgs {
    // Plugin cdylibs (no Rust dep on this crate) can't construct
    // `Array<Gd<WharfkitPromptElement>>` directly; route through here instead.
    #[func]
    pub fn add_element(&mut self, element: Gd<WharfkitPromptElement>) {
        self.elements.push(&element);
    }
}

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[godot(via = i32)]
pub enum PromptResponseKind {
    #[default]
    Accepted = 0,
    Closed = 1,
    Expired = 2,
    ButtonPressed = 3,
    LinkOpened = 4,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct WharfkitPromptResponse {
    #[var]
    pub kind: PromptResponseKind,
    #[var]
    pub id: GString,
    base: Base<Resource>,
}

#[godot_api]
impl WharfkitPromptResponse {
    fn make(kind: PromptResponseKind, id: GString) -> Gd<Self> {
        Gd::from_init_fn(|base| Self { kind, id, base })
    }

    #[func]
    pub fn accepted() -> Gd<Self> {
        Self::make(PromptResponseKind::Accepted, GString::new())
    }
    #[func]
    pub fn closed() -> Gd<Self> {
        Self::make(PromptResponseKind::Closed, GString::new())
    }
    #[func]
    pub fn expired() -> Gd<Self> {
        Self::make(PromptResponseKind::Expired, GString::new())
    }
    #[func]
    pub fn button_pressed(id: GString) -> Gd<Self> {
        Self::make(PromptResponseKind::ButtonPressed, id)
    }
    #[func]
    pub fn link_opened(id: GString) -> Gd<Self> {
        Self::make(PromptResponseKind::LinkOpened, id)
    }
}

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct WharfkitPromptHandle {
    base: Base<RefCounted>,
}

#[godot_api]
impl WharfkitPromptHandle {
    #[signal]
    fn prompt_done(response: Gd<WharfkitPromptResponse>);

    #[signal]
    fn prompt_failed(error: Gd<crate::godot_error::WharfkitError>);
}

// Send-safe state for tokio tasks. `Gd<Object>` is !Send and intentionally
// not stored here; it lives on the Godot-side context class.
pub(crate) struct ContextShared {
    pub(crate) platform: Arc<dyn Platform>,
    #[allow(dead_code)]
    pub(crate) cancel: CancellationToken,
    pub(crate) prompt_in_flight: AtomicBool,
}

impl ContextShared {
    #[allow(dead_code)]
    pub(crate) fn new(platform: Arc<dyn Platform>, cancel: CancellationToken) -> Arc<Self> {
        Arc::new(Self {
            platform,
            cancel,
            prompt_in_flight: AtomicBool::new(false),
        })
    }

    pub(crate) fn try_begin_prompt(&self) -> bool {
        self.prompt_in_flight
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)
            .is_ok()
    }

    pub(crate) fn end_prompt(&self) {
        self.prompt_in_flight.store(false, Ordering::Release);
    }
}

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct WharfkitLoginContext {
    shared: Arc<ContextShared>,
    ui: Option<Gd<Object>>,
    #[var]
    pub platform_name: GString,
    #[var]
    pub arbitrary: Dictionary,
    base: Base<RefCounted>,
}

#[godot_api]
impl WharfkitLoginContext {
    #[signal]
    fn cancelled();

    #[func]
    pub fn is_known_mobile(&self) -> bool {
        self.shared.platform.is_known_mobile()
    }

    #[func]
    pub fn shell_open(&self, uri: GString) {
        self.shared.platform.shell_open(&uri.to_string());
    }

    #[func]
    pub fn request_prompt(&self, args: Gd<WharfkitPromptArgs>) -> Gd<WharfkitPromptHandle> {
        crate::plugin_dispatcher::route_prompt(self.shared.clone(), self.ui.clone(), args)
    }
}

#[allow(dead_code)]
impl WharfkitLoginContext {
    pub fn make(shared: Arc<ContextShared>, ui: Option<Gd<Object>>) -> Gd<Self> {
        let platform_name = platform_label(shared.platform.name());
        Gd::from_init_fn(|base| Self {
            shared,
            ui,
            platform_name: GString::from(platform_name),
            arbitrary: Dictionary::new(),
            base,
        })
    }

    pub fn standalone() -> Gd<Self> {
        let shared = ContextShared::new(
            Arc::new(GodotPlatform) as Arc<dyn Platform>,
            CancellationToken::new(),
        );
        Self::make(shared, None)
    }
}

#[derive(GodotClass)]
#[class(no_init, base=RefCounted)]
pub struct WharfkitTransactContext {
    shared: Arc<ContextShared>,
    ui: Option<Gd<Object>>,
    #[var]
    pub platform_name: GString,
    base: Base<RefCounted>,
}

#[godot_api]
impl WharfkitTransactContext {
    #[signal]
    fn cancelled();

    #[func]
    pub fn is_known_mobile(&self) -> bool {
        self.shared.platform.is_known_mobile()
    }

    #[func]
    pub fn shell_open(&self, uri: GString) {
        self.shared.platform.shell_open(&uri.to_string());
    }

    #[func]
    pub fn request_prompt(&self, args: Gd<WharfkitPromptArgs>) -> Gd<WharfkitPromptHandle> {
        crate::plugin_dispatcher::route_prompt(self.shared.clone(), self.ui.clone(), args)
    }
}

#[allow(dead_code)]
impl WharfkitTransactContext {
    pub fn make(shared: Arc<ContextShared>, ui: Option<Gd<Object>>) -> Gd<Self> {
        let platform_name = platform_label(shared.platform.name());
        Gd::from_init_fn(|base| Self {
            shared,
            ui,
            platform_name: GString::from(platform_name),
            base,
        })
    }
}

fn platform_label(name: wharfkit_session::PlatformName) -> &'static str {
    match name {
        wharfkit_session::PlatformName::Macos => "macOS",
        wharfkit_session::PlatformName::Windows => "Windows",
        wharfkit_session::PlatformName::Linux => "Linux",
        wharfkit_session::PlatformName::IOS => "iOS",
        wharfkit_session::PlatformName::Android => "Android",
        wharfkit_session::PlatformName::Web => "Web",
        wharfkit_session::PlatformName::Headless => "Headless",
    }
}
