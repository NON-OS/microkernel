// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/uefi/variable/firmware.rs` and
//! `src/arch/x86_64/uefi/constants/revisions.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/uefi/variable/firmware.rs"]
pub mod firmware;

#[path = "../../../../../src/arch/x86_64/uefi/constants/revisions.rs"]
pub mod revisions;

pub fn firmwareinfo_uefi_major_version(this: firmware::FirmwareInfo) -> u16 {
    this.uefi_major_version()
}

pub fn firmwareinfo_uefi_minor_version(this: firmware::FirmwareInfo) -> u16 {
    this.uefi_minor_version()
}


pub fn uefi_revision(major: u16, minor: u16, patch: u16) -> u32 {
    revisions::uefi_revision(major, minor, patch)
}
