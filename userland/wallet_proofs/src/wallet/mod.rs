// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's own module shape, so files that reach for
//! `crate::wallet::num` resolve here exactly as they do in the wallet.

/* The guard on the presses that move value or sign. */
pub mod arm;
#[path = "../../../capsule_wallet_nonos/src/wallet/chain.rs"]
pub mod chain;
/* The capsule's event::hex_digit, which a fresh address is read with. */
pub mod event {
    pub use crate::address_text;
    pub use crate::hex_digit;
}
#[cfg(test)]
pub mod meter;
pub mod net;
/* The names the capsule's nox module gives the pure files the fetch reads. */
pub mod nox {
    pub use crate::nox::apr_bps::apr_bps;
    pub use crate::nox::calldata_addr::calldata_addr;
    pub use crate::nox::constants;
    pub use crate::nox::q32_to_u128::q32_to_u128;
    pub use crate::nox::staking::SEL_GET_STAKE_INFO;
}
/* The one gate the wallet copies through: addresses only. */
#[path = "../../../capsule_wallet_nonos/src/wallet/copy/rule.rs"]
pub mod copy_rule;
pub mod num;
/* The capsule's order module, which the consent's terms name absolutely. */
pub use crate::order;
#[path = "../../../capsule_wallet_nonos/src/wallet/rpc/mod.rs"]
pub mod rpc;
/* What can be pressed, as the guard names it. */
pub mod screen;
pub mod send;
#[cfg(test)]
pub mod settle;
pub mod state;
pub mod swap;
pub mod vault;
