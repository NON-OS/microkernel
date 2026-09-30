// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the WiFi settings panel. The real panel source is included
//! via #[path] and exercised against synthetic inputs: adapter discovery, the
//! untrusted scan-result parser, and the detect/select/enter-key/connect state
//! machine.

extern crate alloc;

#[path = "../../capsule_settings/src/wifi/interface.rs"]
pub mod interface;

/// The pure WiFi panel modules, grouped so their `super::` references resolve as
/// they do in the capsule. Adapter enumeration (`adapters`) is left out because
/// it calls the broker; the logic it drives (`interface::discover`) is proven.
pub mod wifi;

/// The saved-network record from `nonos_wifi_client`: its slots, its
/// encoding and its sealing, driven with keys the tests choose. The vfs and
/// TPM halves are left out; they are syscalls.
#[cfg(test)]
mod saved;
#[cfg(test)]
use wifi::network;
#[cfg(test)]
#[path = "../../nonos_wifi_client/src/wipe.rs"]
mod wipe;

#[cfg(test)]
mod interface_tests;
#[cfg(test)]
mod panel_tests;
#[cfg(test)]
mod wire_tests;
