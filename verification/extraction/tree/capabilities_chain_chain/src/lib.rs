// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/capabilities/chain/chain.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod capabilities;

pub fn capabilitychain_empty() -> crate::capabilities::chain::chain::CapabilityChain {
    crate::capabilities::chain::chain::CapabilityChain::empty()
}

pub fn capabilitychain_len(this: crate::capabilities::chain::chain::CapabilityChain) -> usize {
    this.len()
}

pub fn capabilitychain_is_empty(this: crate::capabilities::chain::chain::CapabilityChain) -> bool {
    this.is_empty()
}

pub fn capabilitychain_max_depth() -> usize {
    crate::capabilities::chain::chain::CapabilityChain::max_depth()
}

