use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[godot(via = i32)]
pub enum WharfkitErrorKind {
    #[default]
    Internal = 0,
    UserRejected = 1,
    UserClosed = 2,
    Cancelled = 3,
    Expired = 4,
    Network = 5,
    ChainRejected = 6,
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct WharfkitError {
    #[var]
    pub kind: WharfkitErrorKind,
    #[var]
    pub message: GString,
    #[var]
    pub code: i32,
    #[var]
    pub retryable: bool,
    #[var]
    pub signatures: PackedByteArray,
    base: Base<Resource>,
}

#[godot_api]
impl WharfkitError {
    fn make(kind: WharfkitErrorKind, message: GString, retryable: bool, code: i32) -> Gd<Self> {
        Gd::from_init_fn(|base| Self {
            kind,
            message,
            code,
            retryable,
            signatures: PackedByteArray::new(),
            base,
        })
    }

    #[func]
    pub fn user_rejected(message: GString) -> Gd<Self> {
        Self::make(WharfkitErrorKind::UserRejected, message, false, 0)
    }

    #[func]
    pub fn user_closed(message: GString) -> Gd<Self> {
        Self::make(WharfkitErrorKind::UserClosed, message, true, 0)
    }

    #[func]
    pub fn cancelled(message: GString) -> Gd<Self> {
        Self::make(WharfkitErrorKind::Cancelled, message, true, 0)
    }

    #[func]
    pub fn expired(message: GString) -> Gd<Self> {
        Self::make(WharfkitErrorKind::Expired, message, true, 0)
    }

    #[func]
    pub fn network(message: GString) -> Gd<Self> {
        Self::make(WharfkitErrorKind::Network, message, true, 0)
    }

    #[func]
    pub fn chain_rejected(message: GString, code: i32) -> Gd<Self> {
        Self::make(WharfkitErrorKind::ChainRejected, message, false, code)
    }

    #[func]
    pub fn internal(message: GString) -> Gd<Self> {
        Self::make(WharfkitErrorKind::Internal, message, false, 0)
    }
}
