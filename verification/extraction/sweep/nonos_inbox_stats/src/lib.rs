// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/ipc/nonos_inbox/stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/ipc/nonos_inbox/stats.rs"]
pub mod stats;

pub fn inboxstats_new() -> stats::InboxStats {
    stats::InboxStats::new()
}

pub fn inboxstats_record_enqueue(this: stats::InboxStats, current_size: usize) {
    this.record_enqueue(current_size)
}

pub fn inboxstats_record_dequeue(this: stats::InboxStats) {
    this.record_dequeue()
}

pub fn inboxstats_record_dropped(this: stats::InboxStats) {
    this.record_dropped()
}

