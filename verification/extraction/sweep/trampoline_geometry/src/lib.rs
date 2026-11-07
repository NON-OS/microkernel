// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/mmu/trampoline/geometry.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/mmu/trampoline/geometry.rs"]
pub mod geometry;

pub fn table(pa: u64) -> u64 {
    geometry::table(pa)
}

pub fn page(pa: u64, bits: u64) -> u64 {
    geometry::page(pa, bits)
}

