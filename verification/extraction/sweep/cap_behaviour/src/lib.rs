// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/cap/behaviour.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/cap/behaviour.rs"]
pub mod behaviour;

pub fn requires_write_buffer_flush(cap: u64) -> bool {
    behaviour::requires_write_buffer_flush(cap)
}

pub fn caching_mode(cap: u64) -> bool {
    behaviour::caching_mode(cap)
}

