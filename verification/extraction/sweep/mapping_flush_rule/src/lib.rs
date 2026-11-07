// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/mapping/flush_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/mapping/flush_rule.rs"]
pub mod flush_rule;

pub fn invalidates(mapped: bool, caching_mode: bool) -> bool {
    flush_rule::invalidates(mapped, caching_mode)
}

