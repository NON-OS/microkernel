// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/smp/boot_budget.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/smp/boot_budget.rs"]
pub mod boot_budget;

pub fn budget_ticks(ms: u64, hz: u64, fallback: u64) -> u64 {
    boot_budget::budget_ticks(ms, hz, fallback)
}

