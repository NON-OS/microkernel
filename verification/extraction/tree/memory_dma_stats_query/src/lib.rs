// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/dma/stats/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn dmastats_total_memory(this: crate::memory::dma::stats::types::DmaStats) -> u64 {
    this.total_memory()
}

pub fn dmastats_coherent_count(this: crate::memory::dma::stats::types::DmaStats) -> usize {
    this.coherent_count()
}

pub fn dmastats_streaming_count(this: crate::memory::dma::stats::types::DmaStats) -> usize {
    this.streaming_count()
}

