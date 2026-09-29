// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/security/boot/secure_boot/types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod security;

pub fn bootmeasurements_new() -> crate::security::boot::secure_boot::types::BootMeasurements {
    crate::security::boot::secure_boot::types::BootMeasurements::new()
}

pub fn bootmeasurements_has_initrd(this: crate::security::boot::secure_boot::types::BootMeasurements) -> bool {
    this.has_initrd()
}

pub fn bootmeasurements_has_acpi(this: crate::security::boot::secure_boot::types::BootMeasurements) -> bool {
    this.has_acpi()
}

pub fn bootmeasurements_is_signature_valid(this: crate::security::boot::secure_boot::types::BootMeasurements) -> bool {
    this.is_signature_valid()
}

pub fn bootmeasurements_is_uefi_secure_boot(this: crate::security::boot::secure_boot::types::BootMeasurements) -> bool {
    this.is_uefi_secure_boot()
}

pub fn bootmeasurements_get_boot_timestamp(this: crate::security::boot::secure_boot::types::BootMeasurements) -> u64 {
    this.get_boot_timestamp()
}

pub fn bootmeasurements_is_chain_verified(this: crate::security::boot::secure_boot::types::BootMeasurements) -> bool {
    this.is_chain_verified()
}

pub fn trustedbootkeys_new() -> crate::security::boot::secure_boot::types::TrustedBootKeys {
    crate::security::boot::secure_boot::types::TrustedBootKeys::new()
}

pub fn trustedbootkeys_get_rotation_count(this: crate::security::boot::secure_boot::types::TrustedBootKeys) -> u64 {
    this.get_rotation_count()
}

pub fn trustedbootkeys_total_keys(this: crate::security::boot::secure_boot::types::TrustedBootKeys) -> usize {
    this.total_keys()
}

pub fn trustedkey_created_at(this: crate::security::boot::secure_boot::types::TrustedKey) -> u64 {
    this.created_at()
}

pub fn trustedkey_expires_at(this: crate::security::boot::secure_boot::types::TrustedKey) -> u64 {
    this.expires_at()
}

pub fn trustedkey_is_production(this: crate::security::boot::secure_boot::types::TrustedKey) -> bool {
    this.is_production()
}

pub fn trustedkey_is_expired(this: crate::security::boot::secure_boot::types::TrustedKey, current_time: u64) -> bool {
    this.is_expired(current_time)
}

