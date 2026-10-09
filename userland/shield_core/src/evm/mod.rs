//! This wallet's own Ethereum account, the ordinary `0x` address beside the notes.
//!
//! It derives from the same words on m/44'/60'/0'/0/0, so any Ethereum wallet
//! restores it. The key stays in the core and is wiped when the session drops.
//! Screens see the address and nothing else.

mod account;
mod balances;
pub mod base_fee;
mod calls;
mod checksum;
mod const_hex;
mod derive;
mod gas;
mod held;
mod max;
pub mod network;
pub(crate) mod nonce;
mod nox;
mod nox_read;
mod nox_revert;
mod nox_rules;
mod reply;
mod reply_read;
mod review;
mod review_checks;
mod review_parts;
mod review_token;
mod rlp;
mod rpc;
mod send;
pub mod shield;
pub mod shield_batch;
mod shield_batch_txs;
mod shield_order;
mod shield_tx;
pub mod swap;
pub mod tx;
pub(crate) mod units;
mod usdc_revert;
mod used;
mod views;

pub use account::EvmAccount;
pub use balances::{balances, Holdings};
pub use checksum::checksummed;
pub use max::max_spend;
pub use network::{Chain, Network};
pub use review::review;
pub use review_parts::{Arrival, Checked, Order, Review};
pub use send::{broadcast, status, SendStatus};
pub use used::used;
pub use views::{view, Viewed};
