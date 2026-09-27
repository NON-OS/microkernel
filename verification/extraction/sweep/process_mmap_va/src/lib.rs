// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/process/mmap_va.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/process/mmap_va.rs"]
pub mod mmap_va;

pub fn mmapva_new() -> mmap_va::MmapVa {
    mmap_va::MmapVa::new()
}

