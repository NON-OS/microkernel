// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/kernel_core/surface_registry/pin/register_race.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/kernel_core/surface_registry/pin/register_race.rs"]
pub mod register_race;

pub fn ranges_meet(a: u64, a_end: u64, b: u64, b_end: u64) -> bool {
    register_race::ranges_meet(a, a_end, b, b_end)
}

