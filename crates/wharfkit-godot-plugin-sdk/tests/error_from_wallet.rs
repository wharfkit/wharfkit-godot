#![cfg(feature = "session")]

use wharfkit_godot_plugin_sdk::error::{ErrorKind, WharfkitErrorBuilder};
use wharfkit_session::WalletError;

fn kind_of(b: WharfkitErrorBuilder) -> ErrorKind {
    b.kind_for_test()
}

#[test]
fn buoy_error_maps_to_network() {
    let we = WalletError::Buoy(wharfkit_buoy_client::BuoyError::Closed);
    let b = WharfkitErrorBuilder::from(&we);
    assert_eq!(kind_of(b), ErrorKind::Network);
}

#[test]
fn user_closed_maps_to_user_closed() {
    let we = WalletError::UserClosed;
    let b = WharfkitErrorBuilder::from(&we);
    assert_eq!(kind_of(b), ErrorKind::UserClosed);
}

#[test]
fn cancelled_maps_to_cancelled() {
    let we = WalletError::Cancelled;
    let b = WharfkitErrorBuilder::from(&we);
    assert_eq!(kind_of(b), ErrorKind::Cancelled);
}

#[test]
fn expired_maps_to_expired() {
    let we = WalletError::Expired;
    let b = WharfkitErrorBuilder::from(&we);
    assert_eq!(kind_of(b), ErrorKind::Expired);
}

#[test]
fn internal_maps_to_internal() {
    let we = WalletError::Internal("boom".into());
    let b = WharfkitErrorBuilder::from(&we);
    assert_eq!(kind_of(b), ErrorKind::Internal);
}
