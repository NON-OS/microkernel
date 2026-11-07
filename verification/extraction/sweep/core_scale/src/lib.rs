// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/sys/clock/core/scale.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/sys/clock/core/scale.rs"]
pub mod scale;

pub fn ticks_to_ms(ticks: u64, hz: u64) -> u64 {
    scale::ticks_to_ms(ticks, hz)
}

