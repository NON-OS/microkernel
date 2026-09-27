// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/dma/coherency/mode.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/dma/coherency/mode.rs"]
pub mod mode;

pub fn coherency_from_bool(coherent: bool) -> mode::Coherency {
    mode::Coherency::from_bool(coherent)
}

pub fn coherency_is_coherent(this: mode::Coherency) -> bool {
    this.is_coherent()
}

pub fn coherency_requires_cache_maintenance(this: mode::Coherency) -> bool {
    this.requires_cache_maintenance()
}

