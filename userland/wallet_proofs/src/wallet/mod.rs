// NONOS Operating System (AGPL-3.0-or-later)
//! The capsule's own module shape, so files that reach for
//! `crate::wallet::num` resolve here exactly as they do in the wallet.

#[path = "../../../capsule_wallet_nonos/src/wallet/chain.rs"]
pub mod chain;
pub mod net;
/* The names the capsule's nox module gives the pure files the fetch reads. */
pub mod nox {
    pub use crate::nox::apr_bps::apr_bps;
    pub use crate::nox::calldata_addr::calldata_addr;
    pub use crate::nox::constants;
    pub use crate::nox::q32_to_u128::q32_to_u128;
    pub use crate::nox::staking::SEL_GET_STAKE_INFO;
}
pub mod num;
pub mod paint;
#[path = "../../../capsule_wallet_nonos/src/wallet/rpc/mod.rs"]
pub mod rpc;
pub mod send;
pub mod swap;
pub mod vault;
