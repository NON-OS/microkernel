// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/mmio/constants.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/mmio/constants.rs"]
pub mod constants;

pub fn align_up(value: usize, align: usize) -> usize {
    constants::align_up(value, align)
}

