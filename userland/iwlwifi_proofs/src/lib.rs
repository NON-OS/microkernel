// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the iwlwifi driver: the gen3 radio against a
//! modeled device (boot, sweeps, joins, the link and its failures), the setup
//! of an attempt, and the wire the panels and net.core speak. They prove the
//! driver's register sequences and messages; that a real Intel chip responds
//! to them still needs silicon.

// The driver is no_std and uses `alloc`; link it so the included sources
// (the supplicant's Vec replies) resolve when proven on the host.
extern crate alloc;

#[path = "mounts_radio.rs"]
mod mounts_radio;
#[path = "mounts_setup.rs"]
mod mounts_setup;
#[path = "mounts_wire.rs"]
mod mounts_wire;
pub use mounts_radio::*;
pub use mounts_setup::*;
pub use mounts_wire::*;

#[cfg(test)]
#[path = "tests_gen3.rs"]
mod tests_gen3;
#[cfg(test)]
#[path = "tests_join.rs"]
mod tests_join;
#[cfg(test)]
#[path = "tests_setup.rs"]
mod tests_setup;
#[cfg(test)]
use tests_gen3::*;
#[cfg(test)]
use tests_join::*;

// The lint switches stay in this file, where the allow baseline lists them.
/// The authenticator's messages, as the shared core's proofs build them.
/// Shared whole: the join proofs use some of its helpers, not all.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_core_proofs/src/ap_sim.rs"]
mod ap_sim;
#[cfg(test)]
#[allow(dead_code, unused_imports)]
#[path = "../../nonos_wifi_client/src/join_wire/mod.rs"]
mod client_join_wire;
/// The whole request wire the serving loop reads with: the header decode
/// every frame passes before dispatch, and the reply encoder a refusal is
/// answered with. Its ops file is also `protocol_ops`, loaded twice on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../../capsule_driver_iwlwifi/src/protocol/mod.rs"]
pub mod iwlwifi_wire;
/// The client's side of the same messages, to read the driver's replies
/// with. Shared whole: the proofs use its parsers, not all of it.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_client/src/network.rs"]
mod network;
