// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/hardware/broker/dma/pool/sizing.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/hardware/broker/dma/pool/sizing.rs"]
pub mod sizing;

pub fn low32_floor(pool_pages: usize) -> usize {
    sizing::low32_floor(pool_pages)
}

pub fn pressure_quarter(used: usize, pool_pages: usize) -> usize {
    sizing::pressure_quarter(used, pool_pages)
}

