// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/timer/broadcast/halves.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/timer/broadcast/halves.rs"]
pub mod halves;

pub fn combine(high: u32, low: u32, again: u32) -> u64 {
    halves::combine(high, low, again)
}

