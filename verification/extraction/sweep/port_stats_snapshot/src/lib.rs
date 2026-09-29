// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/port/stats_snapshot.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/port/stats_snapshot.rs"]
pub mod stats_snapshot;

pub fn portstatssnapshot_total_ops(this: stats_snapshot::PortStatsSnapshot) -> u64 {
    this.total_ops()
}

pub fn portstatssnapshot_total_bytes(this: stats_snapshot::PortStatsSnapshot) -> u64 {
    this.total_bytes()
}

