use godot::prelude::*;
use wharfkit_godot_plugin_sdk::TransactContextView;

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn from_dictionary_reads_chain_permission_and_return_path() {
    let mut chain = Dictionary::new();
    chain.set(
        "id",
        GString::from("aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906"),
    );
    chain.set("url", GString::from("https://jungle4.greymass.com"));
    let mut permission = Dictionary::new();
    permission.set("actor", GString::from("teamgreymass"));
    permission.set("permission", GString::from("active"));
    let mut ctx = Dictionary::new();
    ctx.set("chain", chain);
    ctx.set("permission_level", permission);
    ctx.set("return_path", GString::from("game://return"));

    let view = TransactContextView::from_variant(&ctx.to_variant())
        .expect("from_variant on Dictionary returns Some");

    assert_eq!(
        view.chain_id().as_deref(),
        Some("aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906")
    );
    assert_eq!(
        view.chain_url().as_deref(),
        Some("https://jungle4.greymass.com")
    );
    let (actor, permission) = view.permission_level().expect("permission_level");
    assert_eq!(actor, "teamgreymass");
    assert_eq!(permission, "active");
    assert_eq!(view.return_path().as_deref(), Some("game://return"));
}

#[test]
#[ignore = "requires Godot runtime; plain cargo test segfaults without engine init"]
fn resolved_signing_request_returns_live_request_dict_from_ctx() {
    let mut resolved = Dictionary::new();
    resolved.set(
        "chain_id",
        GString::from("aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906"),
    );
    resolved.set("broadcast", false);

    let mut ctx = Dictionary::new();
    ctx.set("resolved_request", resolved);

    let view = TransactContextView::from_variant(&ctx.to_variant()).expect("view");
    let got = view.resolved_signing_request().expect("Some");
    assert_eq!(
        got.get("chain_id")
            .and_then(|v| v.try_to::<GString>().ok())
            .map(|s| s.to_string())
            .as_deref(),
        Some("aca376f206b8fc25a6ed44dbdc66547c36c6c33e3a119ffbeaef943642f0e906")
    );
    assert_eq!(
        got.get("broadcast").and_then(|v| v.try_to::<bool>().ok()),
        Some(false)
    );
}
