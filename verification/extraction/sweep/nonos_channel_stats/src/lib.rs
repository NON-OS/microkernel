// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/ipc/nonos_channel/stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/ipc/nonos_channel/stats.rs"]
pub mod stats;

pub fn busstats_new() -> stats::BusStats {
    stats::BusStats::new()
}

