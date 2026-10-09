// NONOS Operating System (AGPL-3.0-or-later)
//! Host-runnable proofs for the iwlwifi firmware-load path.
//!
//! The FH (Flow Handler) firmware-into-device transfer is a fixed sequence of
//! MMIO register writes defined by the hardware spec. These proofs include the
//! real `load_sections` / `load_firmware_chunk` code and run it against a
//! modeled device (`MockMmio`) that records every write and reports the FH
//! transfer as complete. The proofs then assert the exact register sequence,
//! values, and addressing the driver programs, so the code is validated
//! against the documented spec without any hardware.
//!
//! This validates the driver's register sequence. It does not and cannot
//! assert that a real Intel chip responds correctly; that still needs silicon.

// The driver is no_std and uses `alloc`; link it so the included sources
// (the supplicant's Vec replies) resolve when proven on the host.
extern crate alloc;

#[path = "../../capsule_driver_iwlwifi/src/constants/mod.rs"]
pub mod constants;
// The chip wake and the release of an attempt's grants when it fails, run
// against a register window in host memory.
#[path = "../../capsule_driver_iwlwifi/src/setup/grants.rs"]
pub mod grants;
#[path = "../../capsule_driver_iwlwifi/src/init.rs"]
pub mod init;
#[path = "../../capsule_driver_iwlwifi/src/firmware/load.rs"]
pub mod load;
#[path = "../../capsule_driver_iwlwifi/src/regs.rs"]
pub mod regs;
#[path = "../../capsule_driver_iwlwifi/src/firmware/tlv.rs"]
pub mod tlv;

// The adapter decision discovery makes from a broker record, and what a
// refused interrupt bind means for the attempt: pure, so proven exactly.
pub mod firmware;
#[path = "../../capsule_driver_iwlwifi/src/setup/irq_plan.rs"]
pub mod irq_plan;
#[path = "../../capsule_driver_iwlwifi/src/pci_match.rs"]
pub mod pci_match;
#[cfg(test)]
mod discover_tests;
#[cfg(test)]
mod generation_tests;
#[cfg(test)]
mod regs_tests;

// The 802.11 frame layer: pure IEEE encoding, no hardware, so it is checked
// exactly rather than modeled. `src/dot11/mod.rs` pulls in the real files.
pub mod dot11;

// The host-command queue: header/sequence encoding, TFD ring math, and the
// doorbell register write.
pub mod hcmd;

// The receive path: response packet framing and the receive-ring math.
pub mod rx;

// The WPA2 key derivation: SHA-1, HMAC-SHA1, PBKDF2, the 802.11i PRF, and the
// PMK/PTK derivation, checked against RFC and IEEE known-answer vectors.
pub mod wpa;

// The EAPOL-Key handshake message layer: frame parse and MIC verification.
pub mod eapol;

// WPA2 CCMP data protection: AES-128 in CCM mode.
pub mod ccmp;

#[cfg(test)]
mod grants_tests;
#[cfg(test)]
mod iwlwifi_tests;

#[cfg(test)]
mod hcmd_tests;

#[cfg(test)]
mod rx_tests;

#[cfg(test)]
mod harden_tests;

#[cfg(test)]
mod wpa_tests;

#[cfg(test)]
mod eapol_tests;

#[cfg(test)]
mod dot11_tests;

#[cfg(test)]
mod ccmp_tests;
#[cfg(test)]
mod data_tests;
#[path = "../../capsule_driver_iwlwifi/src/mlme/mod.rs"]
pub mod mlme;
#[cfg(test)]
mod mlme_tests;
#[cfg(test)]
mod supplicant_tests;

// The gen3 (AX210-class) firmware self-load: the context-information structure,
// the peripheral-scratch control block, the image parser, and the boot
// registers. Pure layout and parsing, checked byte-exact against the real
// so-a0-gf-a0 image and the documented struct offsets.
#[path = "../../capsule_driver_iwlwifi/src/firmware/gen3/mod.rs"]
pub mod gen3;

#[path = "../../capsule_driver_iwlwifi/src/protocol/ops.rs"]
mod protocol_ops;
/// The protocol op numbers the serving loop's guard names.
pub mod protocol {
    pub use crate::protocol_ops::*;
}
/// The whole request wire the serving loop reads with: the header decode
/// every frame passes before dispatch, and the reply encoder a refusal is
/// answered with. Its ops file is also `protocol_ops`, loaded twice on purpose.
#[allow(clippy::duplicate_mod)]
#[path = "../../capsule_driver_iwlwifi/src/protocol/mod.rs"]
pub mod iwlwifi_wire;
#[path = "../../capsule_driver_iwlwifi/src/server/control.rs"]
pub mod control;
/// The Wi-Fi client's table of drivers, held to what this driver answers.
#[path = "../../nonos_wifi_client/src/driver/services.rs"]
pub mod wifi_services;
#[path = "../../capsule_driver_iwlwifi/src/server/guard.rs"]
pub mod guard;

#[cfg(test)]
mod dram_map_tests;
#[cfg(test)]
mod gen3_image_tests;
#[cfg(test)]
mod gen3_tests;
#[cfg(test)]
mod request_refusal_tests;
#[cfg(test)]
mod prph_scratch_tests;
#[cfg(test)]
mod blob_scan_tests;
#[cfg(test)]
mod gen3_model;
#[cfg(test)]
mod gen3_boot_tests;
#[cfg(test)]
mod gen3_parse_tests;
#[cfg(test)]
mod gen3_pnvm_tests;
#[cfg(test)]
mod gen3_layout_tests;
#[cfg(test)]
mod gen3_radio_tests;
#[cfg(test)]
mod station_cmd_tests;
#[cfg(test)]
mod tx_path_tests;
/// The authenticator's messages, as the shared core's proofs build them.
/// Shared whole: the join proofs use some of its helpers, not all.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_core_proofs/src/ap_sim.rs"]
mod ap_sim;
#[cfg(test)]
mod join_rig;
#[cfg(test)]
mod join_tests;
#[cfg(test)]
mod exchange_clock_tests;
/// The control family's join messages as the driver reads and answers them.
#[path = "../../capsule_driver_iwlwifi/src/server/join_wire.rs"]
pub mod join_wire;
/// The client's side of the same messages, to read the driver's replies
/// with. Shared whole: the proofs use its parsers, not all of it.
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_client/src/network.rs"]
mod network;
#[cfg(test)]
#[allow(dead_code)]
#[path = "../../nonos_wifi_client/src/join_wire.rs"]
mod client_join_wire;
#[cfg(test)]
mod serve_tests;
