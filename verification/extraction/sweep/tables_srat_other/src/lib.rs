// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/tables/srat_other.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/tables/srat_other.rs"]
pub mod srat_other;

pub fn sratgiccaffinity_is_enabled(this: srat_other::SratGiccAffinity) -> bool {
    this.is_enabled()
}

pub fn sratgenericinitiatoraffinity_is_enabled(this: srat_other::SratGenericInitiatorAffinity) -> bool {
    this.is_enabled()
}

pub fn sratgenericinitiatoraffinity_is_acpi_device(this: srat_other::SratGenericInitiatorAffinity) -> bool {
    this.is_acpi_device()
}

pub fn sratgenericinitiatoraffinity_is_pci_device(this: srat_other::SratGenericInitiatorAffinity) -> bool {
    this.is_pci_device()
}

