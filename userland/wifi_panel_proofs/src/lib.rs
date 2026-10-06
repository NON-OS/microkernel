// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the WiFi settings panel. The real panel and wire source are
//! included and driven through the full detection and selection flow without a
//! device or a renderer. The client's join and link messages and its saved
//! network list (with the flags that keep a WPA3 network off WPA2) are
//! included the same way.

extern crate alloc;

pub mod wifi;

// The saved list names these at the crate root, as in the client crate.
pub use wifi::{join_wire, network};
#[path = "../../nonos_wifi_client/src/driver/fw_step.rs"]
pub mod fw_step;
#[cfg(test)]
mod fw_step_tests;
#[path = "../../nonos_wifi_client/src/wipe.rs"]
pub mod wipe;

#[cfg(test)]
mod saved;

#[cfg(test)]
mod join_tests;
#[cfg(test)]
mod panel_tests;
