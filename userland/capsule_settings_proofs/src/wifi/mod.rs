// NONOS Operating System (AGPL-3.0-or-later)
//! Groups the pure WiFi panel source (network model, wire codec, panel state
//! machine) under one module so their `super::` references resolve exactly as in
//! the capsule, while the proofs drive them against synthetic inputs.

// The saved list reads its join flags from here.
#[cfg(test)]
#[path = "../../../nonos_wifi_client/src/join_wire.rs"]
pub mod join_wire;
#[path = "../../../nonos_wifi_client/src/network.rs"]
pub mod network;
// The connect state machine drives the connect-request encoder, which is
// test-only in the capsule, so the panel proofs compile it under test too.
#[cfg(test)]
#[path = "../../../capsule_settings/src/wifi/panel.rs"]
pub mod panel;
#[path = "../../../nonos_wifi_client/src/wire.rs"]
pub mod wire;
// A scan or join asked for and run from the panel's ticks.
#[path = "../../../capsule_settings/src/settings/state/wifi_pending.rs"]
pub mod pending;
