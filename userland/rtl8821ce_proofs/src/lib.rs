// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the RTL8821CE driver: its real source, included by path and
//! driven against a modeled register file, checked without hardware.

extern crate alloc;

#[path = "mounts.rs"]
mod mounts;
pub use mounts::*;

// The connect stage's answer to a finished join, at the module path its
// `pub(in crate::serve)` items name (src/serve/connect/mod.rs).
#[cfg(test)]
pub mod serve;

#[cfg(test)]
#[path = "tests_all.rs"]
mod tests_all;

// The lint switch stays in this file, where the allow baseline lists it.
// The access point's half of the key handshakes, shared with the core proofs;
// the link proofs use only part of it.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_core_proofs/src/ap_sim.rs"]
mod ap_sim;
#[cfg(test)]
mod pwr_mock;
