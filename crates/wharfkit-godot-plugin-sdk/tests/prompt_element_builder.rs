use wharfkit_godot_plugin_sdk::prompt::{
    PromptBuildError, PromptElementBuilder, PromptElementKind,
};

#[test]
fn qr_builder_carries_kind_and_data() {
    let b = PromptElementBuilder::qr("esr:abc123");
    assert_eq!(b.kind(), PromptElementKind::Qr);
    assert_eq!(b.data(), Some("esr:abc123".to_string()));
}

#[test]
fn link_builder_carries_id_href_label() {
    let b = PromptElementBuilder::link("launch_anchor", "esr:abc", "Launch Anchor");
    assert_eq!(b.kind(), PromptElementKind::Link);
    assert_eq!(b.id(), Some("launch_anchor".to_string()));
    assert_eq!(b.href(), Some("esr:abc".to_string()));
    assert_eq!(b.label(), Some("Launch Anchor".to_string()));
}

#[test]
fn button_builder_carries_id_label() {
    let b = PromptElementBuilder::button("ok", "OK");
    assert_eq!(b.kind(), PromptElementKind::Button);
    assert_eq!(b.id(), Some("ok".to_string()));
    assert_eq!(b.label(), Some("OK".to_string()));
}

#[test]
fn countdown_builder_carries_id_label_end_unix_ms() {
    let b = PromptElementBuilder::countdown("timer", "Approve within", 1_700_000_000_000);
    assert_eq!(b.kind(), PromptElementKind::Countdown);
    assert_eq!(b.id(), Some("timer".to_string()));
    assert_eq!(b.label(), Some("Approve within".to_string()));
    assert_eq!(b.end_unix_ms(), Some(1_700_000_000_000));
}

#[test]
fn accept_builder_carries_label_only() {
    let b = PromptElementBuilder::accept("Accept");
    assert_eq!(b.kind(), PromptElementKind::Accept);
    assert_eq!(b.label(), Some("Accept".to_string()));
}

#[test]
fn close_builder_carries_kind_only() {
    let b = PromptElementBuilder::close();
    assert_eq!(b.kind(), PromptElementKind::Close);
}

#[test]
fn prompt_build_error_names_missing_class() {
    let err = PromptBuildError::missing_class("WharfkitPromptElement");
    assert_eq!(err.class_name(), "WharfkitPromptElement");
    assert_eq!(
        err.to_string(),
        "core addon not registered: WharfkitPromptElement"
    );
}
