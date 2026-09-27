// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/stats.rs"]
pub mod stats;

pub fn init(total: u64) {
    stats::init(total)
}

pub fn add_used(bytes: u64) {
    stats::add_used(bytes)
}

pub fn sub_used(bytes: u64) {
    stats::sub_used(bytes)
}

pub fn used_bytes() -> u64 {
    stats::used_bytes()
}

pub fn total_bytes() -> u64 {
    stats::total_bytes()
}

pub fn used_mb() -> u64 {
    stats::used_mb()
}

pub fn total_mb() -> u64 {
    stats::total_mb()
}

pub fn free_mb() -> u64 {
    stats::free_mb()
}

pub fn usage_percent() -> u8 {
    stats::usage_percent()
}

