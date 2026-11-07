// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/iommu/smmu/encode/pte.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/iommu/smmu/encode/pte.rs"]
pub mod pte;

pub fn table(next_phys: u64) -> u64 {
    pte::table(next_phys)
}

pub fn is_valid(entry: u64) -> bool {
    pte::is_valid(entry)
}

pub fn address(entry: u64) -> u64 {
    pte::address(entry)
}

pub fn level_index(iova: u64, level: usize) -> usize {
    pte::level_index(iova, level)
}

pub fn in_range(iova: u64, size: u64) -> bool {
    pte::in_range(iova, size)
}

