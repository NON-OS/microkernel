// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/phys/bitmap/index.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn byte_of(idx: usize) -> usize {
    crate::memory::phys::bitmap::index::byte_of(idx)
}

pub fn bit_mask(idx: usize) -> u8 {
    crate::memory::phys::bitmap::index::bit_mask(idx)
}

