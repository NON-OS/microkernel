// NONOS Operating System (AGPL-3.0-or-later)
//! Groups the pure WiFi panel source (network model, wire codec, the pending
//! scan or join) under one module so their `super::` references resolve exactly as in
//! the capsule, while the proofs drive them against synthetic inputs.

// The saved list reads its join flags from here.
#[cfg(test)]
#[path = "../../../nonos_wifi_client/src/join_wire/mod.rs"]
pub mod join_wire;
#[path = "../../../nonos_wifi_client/src/network.rs"]
pub mod network;
#[path = "../../../nonos_wifi_client/src/wire.rs"]
pub mod wire;
// A scan or join asked for and run from the panel's ticks.
#[path = "../../../capsule_settings/src/settings/state/wifi/pending.rs"]
pub mod pending;
#[path = "../../../capsule_settings/src/settings/state/wifi/pending_tick.rs"]
pub mod pending_tick;
