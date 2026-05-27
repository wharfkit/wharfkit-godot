//! Codegen'd contract modules produced by `wharfkit-cli`.
//!
//! Regenerate with:
//!   wharfkit-cli codegen --chain jungle4 --account eosio.token \
//!     --out src/generated/eosio_token.rs
//!
//! These files are checked into the repo so the Layer 3 cdylib builds without
//! a live chain connection. Refresh them when the contract ABI changes
//! upstream.

pub mod eosio_token;
