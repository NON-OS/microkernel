// NONOS Operating System (AGPL-3.0-or-later)

//! Host proofs for the MT7921/MT7922 driver: its real source, mounted by
//! path (`mounts`), driven against a modelled chip (`model`): registers
//! with the ownership, remap and reset behaviour Linux v6.12 relies on;
//! the WFDMA rings read and written in the driver's own DMA memory; an
//! MCU that runs the patch and RAM download protocol and answers each
//! command; and the air, with the shared Wi-Fi core proofs' access point.
//! The firmware images are synthetic, built in the tests in the real
//! header layouts: no MediaTek file is read or carried.

extern crate alloc;

#[path = "mounts.rs"]
mod mounts;
pub use mounts::*;

#[cfg(test)]
pub mod model;
#[cfg(test)]
mod tests;

// The shared Wi-Fi core proofs' access point and the elements it sends,
// at the crate paths its files name.
#[cfg(test)]
#[path = "../../nonos_wifi_core_proofs/src/ap_sim.rs"]
pub mod ap_sim;
#[cfg(test)]
#[path = "../../nonos_wifi_core_proofs/src/cipher_sim.rs"]
pub mod cipher_sim;
#[cfg(test)]
#[path = "../../nonos_wifi_core_proofs/src/pmf_sim.rs"]
pub mod pmf_sim;
#[cfg(test)]
pub mod soft;
// The RTL8852 proofs' WPA3 access point (SAE by hash-to-element, PMF
// required), and the same access point in WPA3 transition mode.
#[cfg(test)]
#[path = "../../rtl8852_proofs/src/sae_air/mod.rs"]
pub mod sae_air;
#[cfg(test)]
pub mod sae_mixed;
