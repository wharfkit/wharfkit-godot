use crate::runtime::WharfkitRuntime;
use crate::types::{
    chain_definition::WharfkitChainDefinition, contract::WharfkitContract, name::WharfkitName,
};
use antelope::api::client::{APIClient, DefaultProvider};
use godot::prelude::*;
use std::sync::Arc;
use wharfkit_abicache::ABICache;
use wharfkit_contract::ContractKit;

#[derive(GodotClass)]
#[class(base=RefCounted, no_init)]
pub struct WharfkitContractKit {
    client: Arc<APIClient<DefaultProvider>>,
    kit: Arc<ContractKit>,
}

#[godot_api]
impl WharfkitContractKit {
    #[func]
    pub fn for_chain(chain: Gd<WharfkitChainDefinition>) -> Gd<Self> {
        let chain_def = chain.bind().inner();
        let client = Arc::new(
            APIClient::<DefaultProvider>::default_provider(chain_def.url().to_string(), None)
                .expect("APIClient::default_provider"),
        );
        let abi_cache = Arc::new(ABICache::new(client.clone()));
        let kit = Arc::new(ContractKit::new(abi_cache));
        Gd::from_init_fn(|_| Self { client, kit })
    }

    #[func]
    pub fn load(&self, account: Gd<WharfkitName>) -> Option<Gd<WharfkitContract>> {
        let account_inner = account.bind().inner();
        let kit_ref = self.kit.clone();
        let result = WharfkitRuntime::block_on(async move { kit_ref.load(account_inner).await });
        match result {
            Ok(contract) => Some(WharfkitContract::from_inner(contract, self.client.clone())),
            Err(e) => {
                godot_error!("ContractKit::load failed: {e:?}");
                None
            }
        }
    }
}
