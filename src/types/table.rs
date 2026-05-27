use crate::generated::eosio_token;
use crate::runtime::WharfkitRuntime;
use crate::types::asset::WharfkitAsset;
use antelope::api::client::{APIClient, DefaultProvider};
use antelope::chain::name::Name;
use godot::prelude::*;
use std::sync::Arc;
use wharfkit_contract::Table;

#[derive(GodotClass)]
#[class(base=RefCounted, no_init)]
pub struct WharfkitTable {
    inner: Arc<Table<eosio_token::types::Account>>,
    client: Arc<APIClient<DefaultProvider>>,
}

impl WharfkitTable {
    pub fn eosio_token_accounts(scope: Name, client: Arc<APIClient<DefaultProvider>>) -> Gd<Self> {
        let table = eosio_token::tables::accounts(scope);
        Gd::from_init_fn(|_| Self {
            inner: Arc::new(table),
            client,
        })
    }
}

#[godot_api]
impl WharfkitTable {
    #[func]
    pub fn first_token_balance(&self) -> Option<Gd<WharfkitAsset>> {
        let table = self.inner.clone();
        let client = self.client.clone();
        let result = WharfkitRuntime::block_on(async move { table.first(&client).await });
        let row = match result {
            Ok(opt) => opt?,
            Err(e) => {
                godot_error!("WharfkitTable::first_token_balance failed: {e:?}");
                return None;
            }
        };
        Some(WharfkitAsset::from(GString::from(row.balance.to_string())))
    }
}
