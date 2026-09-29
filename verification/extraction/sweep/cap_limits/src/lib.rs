// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/cap/limits.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/cap/limits.rs"]
pub mod limits;

pub fn domain_count(cap: u64) -> u32 {
    limits::domain_count(cap)
}

pub fn max_address_width(cap: u64) -> u8 {
    limits::max_address_width(cap)
}

