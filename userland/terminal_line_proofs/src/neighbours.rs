// NONOS Operating System (AGPL-3.0-or-later)
//! Files from other capsules that share this crate: the editor's and net.core's.

// The editor shares this crate rather than growing a second proofs crate for
// two pure functions.
#[path = "../../capsule_text_editor/src/editor/goto_line.rs"]
pub mod goto_line;
#[path = "../../capsule_text_editor/src/editor/quick_open.rs"]
pub mod quick_open;

#[path = "../../capsule_net_core/src/server/runner/cadence.rs"]
pub mod cadence;

#[cfg(test)]
mod cadence_tests;
#[cfg(test)]
mod goto_tests;
#[cfg(test)]
mod quick_open_tests;
