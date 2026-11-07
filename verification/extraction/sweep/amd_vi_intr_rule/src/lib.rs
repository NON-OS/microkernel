// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/iommu/amd_vi/intr_rule.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/iommu/amd_vi/intr_rule.rs"]
pub mod intr_rule;

pub fn remapped(table: u64) -> u64 {
    intr_rule::remapped(table)
}

pub fn passes_any(q: u64) -> bool {
    intr_rule::passes_any(q)
}

pub fn irte(vector: u8, dest: u8) -> u32 {
    intr_rule::irte(vector, dest)
}

