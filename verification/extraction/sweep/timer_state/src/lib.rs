// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/interrupts/timer/state.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/interrupts/timer/state.rs"]
pub mod state;

pub fn get_ticks() -> u64 {
    state::get_ticks()
}

pub fn increment_ticks() {
    state::increment_ticks()
}

pub fn reset_ticks() {
    state::reset_ticks()
}

