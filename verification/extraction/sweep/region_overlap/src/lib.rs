// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/region/overlap.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/region/overlap.rs"]
pub mod overlap;

pub fn overlaps(a_start: u64, a_end: u64, b_start: u64, b_end: u64) -> bool {
    overlap::overlaps(a_start, a_end, b_start, b_end)
}

pub fn contains(start: u64, end: u64, addr: u64) -> bool {
    overlap::contains(start, end, addr)
}

pub fn contains_range(a_start: u64, a_end: u64, b_start: u64, b_end: u64) -> bool {
    overlap::contains_range(a_start, a_end, b_start, b_end)
}

