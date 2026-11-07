// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/block_device/store_copy/span.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/block_device/store_copy/span.rs"]
pub mod span;

pub fn held(len: usize) -> usize {
    span::held(len)
}

