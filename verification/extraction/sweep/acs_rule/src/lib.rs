// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/acs/rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/acs/rule.rs"]
pub mod rule;

pub fn closed(ctl: u16) -> bool {
    rule::closed(ctl)
}

