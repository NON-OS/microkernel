// NONOS Operating System (AGPL-3.0-or-later)
//! The pure WiFi panel source (network model, wire codec, panel state machine)
//! under one module so their `super::` references resolve as in the capsule,
//! mounted as capsule_settings_proofs mounts them.

#[path = "../../../nonos_wifi_client/src/join_wire.rs"]
pub mod join_wire;
#[path = "../../../nonos_wifi_client/src/network.rs"]
pub mod network;
// The panel drives the connect-request encoder, which is test-only in the
// client crate, so the panel compiles under test too.
#[cfg(test)]
#[path = "../../../capsule_settings/src/wifi/panel.rs"]
pub mod panel;
#[path = "../../../nonos_wifi_client/src/wire.rs"]
pub mod wire;

#[cfg(test)]
pub use panel::{WifiPanel, WifiStatus};
pub use wire::parse_scan;
