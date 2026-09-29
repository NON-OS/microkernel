// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/mmu/constants/helpers.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn pml4_index(va: u64) -> usize {
    crate::memory::mmu::constants::helpers::pml4_index(va)
}

pub fn pdpt_index(va: u64) -> usize {
    crate::memory::mmu::constants::helpers::pdpt_index(va)
}

pub fn pd_index(va: u64) -> usize {
    crate::memory::mmu::constants::helpers::pd_index(va)
}

pub fn pt_index(va: u64) -> usize {
    crate::memory::mmu::constants::helpers::pt_index(va)
}

pub fn pte_is_present(entry: u64) -> bool {
    crate::memory::mmu::constants::helpers::pte_is_present(entry)
}

pub fn pte_address(entry: u64) -> u64 {
    crate::memory::mmu::constants::helpers::pte_address(entry)
}

