// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/gic/its/encode/attrs.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/gic/its/encode/attrs.rs"]
pub mod attrs;

pub fn attrs(coherent: bool) -> u64 {
    attrs::attrs(coherent)
}

pub fn coherent(readback: u64) -> bool {
    attrs::coherent(readback)
}

pub fn cbaser(phys: u64, pages: u64, coherent: bool) -> u64 {
    attrs::cbaser(phys, pages, coherent)
}

