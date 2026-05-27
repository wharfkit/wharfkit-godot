use godot::prelude::*;

use crate::godot_error::WharfkitError;
use crate::prompt_types::WharfkitPromptResponse;

#[derive(GodotClass)]
#[class(init, base=RefCounted)]
pub struct WharfkitWalletPlugin {
    base: Base<RefCounted>,
}

#[godot_api]
impl WharfkitWalletPlugin {
    #[signal]
    fn login_done(response: Variant);
    #[signal]
    fn login_failed(error: Gd<WharfkitError>);

    #[signal]
    fn sign_done(response: Variant);
    #[signal]
    fn sign_failed(error: Gd<WharfkitError>);

    #[signal]
    fn logout_done();
    #[signal]
    fn logout_failed(error: Gd<WharfkitError>);
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct WharfkitUserInterface {
    base: Base<Node>,
}

#[godot_api]
impl WharfkitUserInterface {
    #[signal]
    fn login_done(response: Variant);
    #[signal]
    fn login_failed(error: Gd<WharfkitError>);

    #[signal]
    fn prompt_done(response: Gd<WharfkitPromptResponse>);
    #[signal]
    fn prompt_failed(error: Gd<WharfkitError>);
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct WharfkitDeepLinkBridge {
    base: Base<Node>,
}

#[godot_api]
impl WharfkitDeepLinkBridge {
    #[signal]
    fn deeplink_received(url: GString);
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct NoOpDeepLinkBridge {
    base: Base<Node>,
}

#[godot_api]
impl NoOpDeepLinkBridge {
    #[signal]
    fn deeplink_received(url: GString);
}
