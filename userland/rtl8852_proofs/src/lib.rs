// NONOS Operating System (AGPL-3.0-or-later)
//! Host proofs for the RTL8852BE driver: its real source, included by path
//! and run against a modelled chip (`model`), checked without hardware.

extern crate alloc;

#[path = "mounts.rs"]
mod mounts;
pub use mounts::*;

/// The modelled chip: registers, firmware CPU, DMA, and the air.
pub mod model;

/// A WPA3 access point for the modelled air.
pub mod sae_air;

// The simulated access point and its air, shared with the Wi-Fi core
// proofs, mounted at the paths their files name.
#[path = "../../nonos_wifi_core_proofs/src/ap_sim.rs"]
pub mod ap_sim;
#[path = "../../nonos_wifi_core_proofs/src/cipher_sim.rs"]
pub mod cipher_sim;
#[path = "../../nonos_wifi_core_proofs/src/pmf_sim.rs"]
pub mod pmf_sim;
#[path = "../../nonos_wifi_core_proofs/src/soft/mod.rs"]
pub mod soft;

#[cfg(test)]
mod tests;
