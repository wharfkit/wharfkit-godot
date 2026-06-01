use wharfkit_godot_plugin_sdk::prompt::PromptResponseKind;

#[test]
fn from_i32_round_trips_all_variants() {
    for (raw, expected) in [
        (0, PromptResponseKind::Accepted),
        (1, PromptResponseKind::Closed),
        (2, PromptResponseKind::Expired),
        (3, PromptResponseKind::ButtonPressed),
        (4, PromptResponseKind::LinkOpened),
    ] {
        assert_eq!(PromptResponseKind::from_i32(raw), Some(expected));
        assert_eq!(expected as i32, raw);
    }
}

#[test]
fn from_i32_returns_none_for_unknown() {
    assert_eq!(PromptResponseKind::from_i32(99), None);
}
