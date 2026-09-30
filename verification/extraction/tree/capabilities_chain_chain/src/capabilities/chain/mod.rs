// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../src/capabilities/chain/chain.rs"]
pub mod chain;

#[path = "../../../../../../../src/capabilities/chain/constants.rs"]
pub mod constants;

pub use chain::CapabilityChain;
pub use constants::{max_chain_depth, MAX_CHAIN_DEPTH};
