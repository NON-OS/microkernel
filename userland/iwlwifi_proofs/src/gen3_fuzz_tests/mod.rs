// NONOS Operating System (AGPL-3.0-or-later)
//! The gen3 parsers fed noise and mutated well-formed input from a seeded
//! generator (the Wi-Fi core proofs' own, `fuzz.rs`): what the firmware
//! writes into a receive buffer, what the air brings, and the firmware and
//! PNVM files. Nothing may panic, and whatever a parser hands back must lie
//! inside what it was given.

mod files;
mod frames;
mod notifs;

/// Whether `inner` is a slice of `outer`.
pub(super) fn within(outer: &[u8], inner: &[u8]) -> bool {
    let (o, i) = (outer.as_ptr() as usize, inner.as_ptr() as usize);
    i >= o && i + inner.len() <= o + outer.len()
}
