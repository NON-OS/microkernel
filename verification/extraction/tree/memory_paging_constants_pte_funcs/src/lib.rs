// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/paging/constants/pte_funcs.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub mod memory;

pub fn pte_is_present(pte: u64) -> bool {
    crate::memory::paging::constants::pte_funcs::pte_is_present(pte)
}

pub fn pte_is_huge(pte: u64) -> bool {
    crate::memory::paging::constants::pte_funcs::pte_is_huge(pte)
}

pub fn pte_address(pte: u64) -> u64 {
    crate::memory::paging::constants::pte_funcs::pte_address(pte)
}

pub fn pte_is_writable(pte: u64) -> bool {
    crate::memory::paging::constants::pte_funcs::pte_is_writable(pte)
}

pub fn pte_is_executable(pte: u64) -> bool {
    crate::memory::paging::constants::pte_funcs::pte_is_executable(pte)
}

pub fn pte_is_user(pte: u64) -> bool {
    crate::memory::paging::constants::pte_funcs::pte_is_user(pte)
}

