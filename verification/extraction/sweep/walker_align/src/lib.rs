// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/fdt/walker/align.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/fdt/walker/align.rs"]
pub mod align;

pub fn align4(n: usize) -> usize {
    align::align4(n)
}

