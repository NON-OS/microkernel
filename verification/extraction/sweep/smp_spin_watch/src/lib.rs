// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/smp/spin_watch.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/smp/spin_watch.rs"]
pub mod spin_watch;

pub fn slot_of(addr: usize) -> usize {
    spin_watch::slot_of(addr)
}

pub fn due(start: u64, now: u64, reported: bool) -> bool {
    spin_watch::due(start, now, reported)
}

