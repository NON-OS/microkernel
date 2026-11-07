// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/regs/cap/pages.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/regs/cap/pages.rs"]
pub mod pages;

pub fn best_leaf_level(cap: u64) -> u8 {
    pages::best_leaf_level(cap)
}

