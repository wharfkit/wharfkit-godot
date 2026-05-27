use antelope::chain::checksum::Checksum256;
use godot::prelude::*;
use wharfkit_common::ChainDefinition;

#[derive(GodotClass)]
#[class(base=Resource, init)]
pub struct WharfkitChainDefinition {
    #[export]
    chain_id_hex: GString,
    #[export]
    url: GString,
}

#[godot_api]
impl WharfkitChainDefinition {
    #[func]
    pub fn from(chain_id_hex: GString, url: GString) -> Gd<Self> {
        Gd::from_init_fn(|_base| Self { chain_id_hex, url })
    }

    #[func]
    pub fn chain_id(&self) -> GString {
        self.chain_id_hex.clone()
    }

    #[func]
    pub fn url_value(&self) -> GString {
        self.url.clone()
    }

    pub fn inner(&self) -> ChainDefinition {
        let id = Checksum256::from_hex(&self.chain_id_hex.to_string()).expect("valid chain id hex");
        ChainDefinition::new(id, self.url.to_string())
    }
}
