// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/tables/fadt/profile.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/tables/fadt/profile.rs"]
pub mod profile;

pub fn pmprofile_from_u8(value: u8) -> profile::PmProfile {
    profile::PmProfile::from_u8(value)
}

pub fn pmprofile_is_server(this: profile::PmProfile) -> bool {
    this.is_server()
}

pub fn pmprofile_is_mobile(this: profile::PmProfile) -> bool {
    this.is_mobile()
}

