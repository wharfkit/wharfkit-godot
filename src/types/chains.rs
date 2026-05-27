use crate::types::chain_definition::WharfkitChainDefinition;
use godot::prelude::*;
use wharfkit_common::{ChainDefinition, Chains as CommonChains};

#[derive(GodotClass)]
#[class(base=RefCounted, init)]
pub struct WharfkitChains;

fn wrap(def: ChainDefinition) -> Gd<WharfkitChainDefinition> {
    WharfkitChainDefinition::from(
        GString::from(def.id().as_string()),
        GString::from(def.url().to_string()),
    )
}

#[godot_api]
impl WharfkitChains {
    #[func]
    pub fn jungle4() -> Gd<WharfkitChainDefinition> {
        wrap(CommonChains::jungle4())
    }

    #[func]
    pub fn eos() -> Gd<WharfkitChainDefinition> {
        wrap(CommonChains::eos())
    }

    #[func]
    pub fn wax() -> Gd<WharfkitChainDefinition> {
        wrap(CommonChains::wax())
    }

    #[func]
    pub fn telos() -> Gd<WharfkitChainDefinition> {
        wrap(CommonChains::telos())
    }

    #[func]
    pub fn vaulta() -> Gd<WharfkitChainDefinition> {
        wrap(CommonChains::vaulta())
    }
}
