// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/region/constants/helpers.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/region/constants/helpers.rs"]
pub mod helpers;

pub fn align_up(value: u64, align: u64) -> u64 {
    helpers::align_up(value, align)
}

pub fn align_down(value: u64, align: u64) -> u64 {
    helpers::align_down(value, align)
}

pub fn align_size(size: usize, align: usize) -> usize {
    helpers::align_size(size, align)
}

