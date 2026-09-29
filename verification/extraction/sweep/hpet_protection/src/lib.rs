// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/tables/hpet/protection.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/tables/hpet/protection.rs"]
pub mod protection;

pub fn pageprotection_from_u8(value: u8) -> protection::PageProtection {
    protection::PageProtection::from_u8(value)
}

