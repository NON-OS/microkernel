// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/dma/kept_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/dma/kept_rule.rs"]
pub mod kept_rule;

pub fn keeps(pci: bool, confined: bool, released: bool) -> bool {
    kept_rule::keeps(pci, confined, released)
}

