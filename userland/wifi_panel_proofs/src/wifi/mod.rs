// NONOS Operating System (AGPL-3.0-or-later)
//! The pure WiFi client source (network model, wire codec, join messages)
//! under one module so their `super::` references resolve as in the capsule,
//! mounted as capsule_settings_proofs mounts them.

#[path = "../../../nonos_wifi_client/src/join_wire/mod.rs"]
pub mod join_wire;
#[path = "../../../nonos_wifi_client/src/network.rs"]
pub mod network;
#[path = "../../../nonos_wifi_client/src/wire.rs"]
pub mod wire;

pub use wire::parse_scan;
