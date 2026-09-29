// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/drivers/security/mmio_range.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/drivers/security/mmio_range.rs"]
pub mod mmio_range;

pub fn range_ok(base: usize, size: usize) -> bool {
    mmio_range::range_ok(base, size)
}

