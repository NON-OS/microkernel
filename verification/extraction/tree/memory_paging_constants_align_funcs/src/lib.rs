// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/paging/constants/align_funcs.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn page_align_down(addr: u64) -> u64 {
    crate::memory::paging::constants::align_funcs::page_align_down(addr)
}

pub fn page_align_up(addr: u64) -> u64 {
    crate::memory::paging::constants::align_funcs::page_align_up(addr)
}

pub fn pages_needed(size: usize) -> usize {
    crate::memory::paging::constants::align_funcs::pages_needed(size)
}

