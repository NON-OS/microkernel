// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/dma/types/snapshot.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/dma/types/snapshot.rs"]
pub mod snapshot;

pub fn dmastatssnapshot_new() -> snapshot::DmaStatsSnapshot {
    snapshot::DmaStatsSnapshot::new()
}

