// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/common/low32_sizing.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/common/low32_sizing.rs"]
pub mod low32_sizing;

pub fn low32_target_pages(low_usable: u64, max: usize) -> usize {
    low32_sizing::low32_target_pages(low_usable, max)
}

pub fn low32_fit(target: usize, region_pages: usize) -> usize {
    low32_sizing::low32_fit(target, region_pages)
}

