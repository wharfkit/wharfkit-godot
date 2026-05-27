use crate::types::{name::WharfkitName, table::WharfkitTable};
use antelope::api::client::{APIClient, DefaultProvider};
use godot::prelude::*;
use std::sync::Arc;
use wharfkit_contract::Contract;

#[derive(GodotClass)]
#[class(base=RefCounted, no_init)]
pub struct WharfkitContract {
    inner: Arc<Contract>,
    client: Arc<APIClient<DefaultProvider>>,
}

impl WharfkitContract {
    pub fn from_inner(inner: Contract, client: Arc<APIClient<DefaultProvider>>) -> Gd<Self> {
        Gd::from_init_fn(|_| Self {
            inner: Arc::new(inner),
            client,
        })
    }
}

#[godot_api]
impl WharfkitContract {
    #[func]
    pub fn account(&self) -> Gd<WharfkitName> {
        let name_str = self.inner.account().to_string();
        WharfkitName::from(GString::from(name_str))
    }

    #[func]
    pub fn table(
        &self,
        table_name: Gd<WharfkitName>,
        scope: Gd<WharfkitName>,
    ) -> Option<Gd<WharfkitTable>> {
        let requested = table_name.bind().inner().to_string();
        if requested != "accounts" || self.inner.account().to_string() != "eosio.token" {
            godot_error!(
                "WharfkitContract::table currently only supports eosio.token::accounts (got {}::{requested})",
                self.inner.account().to_string()
            );
            return None;
        }
        Some(WharfkitTable::eosio_token_accounts(
            scope.bind().inner(),
            self.client.clone(),
        ))
    }
}
