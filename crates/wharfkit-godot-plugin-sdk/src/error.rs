use godot::classes::ClassDb;
use godot::prelude::*;

const CLASS_NAME: &str = "WharfkitError";

#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorKind {
    Internal = 0,
    UserRejected = 1,
    UserClosed = 2,
    Cancelled = 3,
    Expired = 4,
    Network = 5,
    ChainRejected = 6,
}

pub struct WharfkitErrorBuilder {
    kind: ErrorKind,
    message: String,
    code: i32,
    retryable: bool,
}

impl WharfkitErrorBuilder {
    fn new(kind: ErrorKind, message: impl Into<String>, retryable: bool) -> Self {
        Self {
            kind,
            message: message.into(),
            code: 0,
            retryable,
        }
    }

    pub fn user_rejected(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::UserRejected, message, false)
    }

    pub fn user_closed(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::UserClosed, message, true)
    }

    pub fn cancelled(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Cancelled, message, true)
    }

    pub fn expired(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Expired, message, true)
    }

    pub fn network(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Network, message, true)
    }

    pub fn chain_rejected(message: impl Into<String>, code: i32) -> Self {
        let mut b = Self::new(ErrorKind::ChainRejected, message, false);
        b.code = code;
        b
    }

    pub fn internal(message: impl Into<String>) -> Self {
        Self::new(ErrorKind::Internal, message, false)
    }

    pub fn with_code(mut self, code: i32) -> Self {
        self.code = code;
        self
    }

    #[doc(hidden)]
    pub fn kind_for_test(&self) -> ErrorKind {
        self.kind
    }

    pub fn build(self) -> Gd<godot::classes::Object> {
        let class_name = StringName::from(CLASS_NAME);
        let variant = ClassDb::singleton().instantiate(&class_name);
        let mut obj: Gd<godot::classes::Object> = variant
            .try_to::<Gd<godot::classes::Object>>()
            .expect("ClassDb::instantiate(\"WharfkitError\") -> Object");

        obj.set(&StringName::from("kind"), &(self.kind as i32).to_variant());
        obj.set(
            &StringName::from("message"),
            &GString::from(self.message).to_variant(),
        );
        obj.set(&StringName::from("code"), &self.code.to_variant());
        obj.set(&StringName::from("retryable"), &self.retryable.to_variant());

        obj
    }
}

#[cfg(feature = "session")]
mod session_conversions {
    use wharfkit_session::WalletError;

    use super::WharfkitErrorBuilder;

    impl From<&WalletError> for WharfkitErrorBuilder {
        fn from(err: &WalletError) -> Self {
            match err {
                WalletError::UserClosed => WharfkitErrorBuilder::user_closed("user closed prompt"),
                WalletError::UserRejected(message) => {
                    WharfkitErrorBuilder::user_rejected(message.clone())
                }
                WalletError::Cancelled => WharfkitErrorBuilder::cancelled("operation cancelled"),
                WalletError::Expired => WharfkitErrorBuilder::expired("operation expired"),
                WalletError::Buoy(err) => WharfkitErrorBuilder::network(format!("buoy: {err}")),
                WalletError::Esr(err) => WharfkitErrorBuilder::internal(format!("esr: {err}")),
                WalletError::Internal(message) => WharfkitErrorBuilder::internal(message.clone()),
            }
        }
    }
}
