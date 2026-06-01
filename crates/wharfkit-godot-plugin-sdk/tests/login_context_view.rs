use godot::prelude::*;
use wharfkit_godot_plugin_sdk::LoginContextView;

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn from_dictionary_reads_chain_id_and_app_name() {
    let mut chain = Dictionary::new();
    chain.set(
        "id",
        GString::from("aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906"),
    );
    chain.set("url", GString::from("https://jungle4.greymass.com"));
    let mut ctx = Dictionary::new();
    ctx.set("chain", chain);
    ctx.set("app_name", GString::from("Test App"));
    ctx.set("return_path", GString::from("game://return"));

    let view = LoginContextView::from_variant(&ctx.to_variant())
        .expect("from_variant on Dictionary returns Some");

    assert_eq!(
        view.chain_id().expect("chain_id"),
        "aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906"
    );
    assert_eq!(
        view.chain_url().as_deref(),
        Some("https://jungle4.greymass.com")
    );
    assert_eq!(view.app_name().as_deref(), Some("Test App"));
    assert_eq!(view.return_path().as_deref(), Some("game://return"));
}

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn chain_id_returns_none_when_missing() {
    let ctx = Dictionary::new();
    let view = LoginContextView::from_variant(&ctx.to_variant())
        .expect("from_variant on empty Dict returns Some");
    assert_eq!(view.chain_id(), None);
    assert_eq!(view.app_name(), None);
}
