// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/paging/constants/index_funcs.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn pml4_index(va: u64) -> usize {
    crate::memory::paging::constants::index_funcs::pml4_index(va)
}

pub fn pdpt_index(va: u64) -> usize {
    crate::memory::paging::constants::index_funcs::pdpt_index(va)
}

pub fn pd_index(va: u64) -> usize {
    crate::memory::paging::constants::index_funcs::pd_index(va)
}

pub fn pt_index(va: u64) -> usize {
    crate::memory::paging::constants::index_funcs::pt_index(va)
}

pub fn page_offset(va: u64) -> usize {
    crate::memory::paging::constants::index_funcs::page_offset(va)
}

