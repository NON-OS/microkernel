// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/uefi/secure_boot_status.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/uefi/secure_boot_status.rs"]
pub mod secure_boot_status;

pub fn securebootstatus_is_fully_configured(this: secure_boot_status::SecureBootStatus) -> bool {
    this.is_fully_configured()
}

pub fn securebootstatus_can_modify_keys(this: secure_boot_status::SecureBootStatus) -> bool {
    this.can_modify_keys()
}

