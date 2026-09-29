// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/mmio/types/stats_snapshot.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/mmio/types/stats_snapshot.rs"]
pub mod stats_snapshot;

pub fn mmiostatssnapshot_new() -> stats_snapshot::MmioStatsSnapshot {
    stats_snapshot::MmioStatsSnapshot::new()
}

pub fn mmiostatssnapshot_total_operations(this: stats_snapshot::MmioStatsSnapshot) -> u64 {
    this.total_operations()
}

