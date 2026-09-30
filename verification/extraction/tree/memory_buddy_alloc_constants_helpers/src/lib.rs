// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/buddy_alloc/constants/helpers.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn order_to_size(order: usize) -> usize {
    crate::memory::buddy_alloc::constants::helpers::order_to_size(order)
}

pub fn size_to_order(size: usize) -> usize {
    crate::memory::buddy_alloc::constants::helpers::size_to_order(size)
}

pub fn buddy_address(addr: u64, order: usize) -> u64 {
    crate::memory::buddy_alloc::constants::helpers::buddy_address(addr, order)
}

