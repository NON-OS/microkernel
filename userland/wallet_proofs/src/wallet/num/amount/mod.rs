// NONOS Operating System (AGPL-3.0-or-later)
//! The keypad's own module shape, so the shipping files compile unchanged.

#[allow(dead_code)]
#[path = "../../../../../capsule_wallet_nonos/src/wallet/num/amount/types.rs"]
pub mod types;
#[allow(dead_code)]
#[path = "../../../../../capsule_wallet_nonos/src/wallet/num/amount/edit.rs"]
pub mod edit;
#[allow(dead_code)]
#[path = "../../../../../capsule_wallet_nonos/src/wallet/num/amount/scale.rs"]
pub mod scale;
#[allow(dead_code)]
#[path = "../../../../../capsule_wallet_nonos/src/wallet/num/amount/set.rs"]
pub mod set;

pub use types::Amount;
