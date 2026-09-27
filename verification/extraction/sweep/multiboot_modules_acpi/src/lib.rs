// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/multiboot/modules_acpi.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/multiboot/modules_acpi.rs"]
pub mod modules_acpi;

pub fn acpirsdp_is_acpi2(this: modules_acpi::AcpiRsdp) -> bool {
    this.is_acpi2()
}

pub fn acpirsdp_table_address(this: modules_acpi::AcpiRsdp) -> u64 {
    this.table_address()
}

pub fn acpirsdp_verify_checksum(this: modules_acpi::AcpiRsdp) -> bool {
    this.verify_checksum()
}

pub fn acpirsdp_verify_extended_checksum(this: modules_acpi::AcpiRsdp) -> bool {
    this.verify_extended_checksum()
}

