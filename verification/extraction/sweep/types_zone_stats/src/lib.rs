// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/phys/types/zone_stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/phys/types/zone_stats.rs"]
pub mod zone_stats;

pub fn zonestats_new(total: usize, free: usize) -> zone_stats::ZoneStats {
    zone_stats::ZoneStats::new(total, free)
}

pub fn zonestats_frames_allocated(this: zone_stats::ZoneStats) -> usize {
    this.frames_allocated()
}

pub fn zonestats_usage_percent(this: zone_stats::ZoneStats) -> usize {
    this.usage_percent()
}

pub fn zonestats_total_bytes(this: zone_stats::ZoneStats, page_size: usize) -> usize {
    this.total_bytes(page_size)
}

pub fn zonestats_free_bytes(this: zone_stats::ZoneStats, page_size: usize) -> usize {
    this.free_bytes(page_size)
}

