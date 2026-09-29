// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/usercopy/walk/leaf.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod usercopy;

pub fn userleaf_bytes_remaining_in_page(this: crate::usercopy::walk::leaf::UserLeaf) -> u64 {
    this.bytes_remaining_in_page()
}

