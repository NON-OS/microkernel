// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/cpu/msr_stats.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/cpu/msr_stats.rs"]
pub mod msr_stats;

pub fn increment_reads() {
    msr_stats::increment_reads()
}

pub fn increment_writes() {
    msr_stats::increment_writes()
}

pub fn msr_reads() -> u64 {
    msr_stats::msr_reads()
}

pub fn msr_writes() -> u64 {
    msr_stats::msr_writes()
}

