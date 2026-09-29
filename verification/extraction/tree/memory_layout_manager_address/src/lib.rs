// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/layout/manager/address.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn in_kernel_space(va: u64) -> bool {
    crate::memory::layout::manager::address::in_kernel_space(va)
}

pub fn in_user_space(va: u64) -> bool {
    crate::memory::layout::manager::address::in_user_space(va)
}

pub fn is_canonical(va: u64) -> bool {
    crate::memory::layout::manager::address::is_canonical(va)
}

pub fn selfref_l4_va() -> u64 {
    crate::memory::layout::manager::address::selfref_l4_va()
}

