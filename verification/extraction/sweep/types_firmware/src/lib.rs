// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/boot/handoff/types/firmware.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/boot/handoff/types/firmware.rs"]
pub mod firmware;

pub fn firmwareentry_empty() -> firmware::FirmwareEntry {
    firmware::FirmwareEntry::empty()
}

pub fn firmwarehandoff_new() -> firmware::FirmwareHandoff {
    firmware::FirmwareHandoff::new()
}

