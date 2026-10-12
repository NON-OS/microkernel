// NONOS Operating System (AGPL-3.0-or-later)
//! The proofs of joins, the link and the serving loop, with the simulated
//! access point and the client's side of the wire.

/// The authenticator's messages, as the shared core's proofs build them.
/// Shared whole: the join proofs use some of its helpers, not all.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_core_proofs/src/ap_sim.rs"]
pub(crate) mod ap_sim;
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_client/src/join_wire.rs"]
pub(crate) mod client_join_wire;
#[cfg(test)]
mod exchange_clock_tests;
#[cfg(test)]
pub(crate) mod join_rig;
#[cfg(test)]
mod join_tests;
#[cfg(test)]
mod link_gate_tests;
/// The client's side of the same messages, to read the driver's replies
/// with. Shared whole: the proofs use its parsers, not all of it.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_client/src/network.rs"]
pub(crate) mod network;
#[cfg(test)]
mod serve_tests;
#[cfg(test)]
mod watch_tests;
