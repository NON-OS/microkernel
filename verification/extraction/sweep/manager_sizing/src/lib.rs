// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/heap/manager/sizing.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/heap/manager/sizing.rs"]
pub mod sizing;

pub fn second_region_size(ram: u64, min: usize, max: usize, page: usize) -> usize {
    sizing::second_region_size(ram, min, max, page)
}

pub fn user_page_refused(user: bool, free: usize, floor: usize) -> bool {
    sizing::user_page_refused(user, free, floor)
}

