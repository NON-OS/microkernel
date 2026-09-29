// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/dma/stats/record.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn dmastats_record_coherent_alloc(this: crate::memory::dma::stats::types::DmaStats, size: usize) {
    this.record_coherent_alloc(size)
}

pub fn dmastats_record_coherent_free(this: crate::memory::dma::stats::types::DmaStats, size: usize) {
    this.record_coherent_free(size)
}

pub fn dmastats_record_streaming_map(this: crate::memory::dma::stats::types::DmaStats) {
    this.record_streaming_map()
}

pub fn dmastats_record_streaming_unmap(this: crate::memory::dma::stats::types::DmaStats) {
    this.record_streaming_unmap()
}

pub fn dmastats_record_bounce_usage(this: crate::memory::dma::stats::types::DmaStats, used: bool) {
    this.record_bounce_usage(used)
}

