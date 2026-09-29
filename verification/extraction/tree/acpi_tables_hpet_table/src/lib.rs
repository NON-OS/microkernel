// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/hpet/table.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn hpet_comparator_count(this: crate::arch::x86_64::acpi::tables::hpet::table::Hpet) -> u8 {
    this.comparator_count()
}

pub fn hpet_vendor_id(this: crate::arch::x86_64::acpi::tables::hpet::table::Hpet) -> u16 {
    this.vendor_id()
}

pub fn hpet_hardware_revision(this: crate::arch::x86_64::acpi::tables::hpet::table::Hpet) -> u8 {
    this.hardware_revision()
}

pub fn hpet_is_64bit(this: crate::arch::x86_64::acpi::tables::hpet::table::Hpet) -> bool {
    this.is_64bit()
}

pub fn hpet_supports_legacy_replacement(this: crate::arch::x86_64::acpi::tables::hpet::table::Hpet) -> bool {
    this.supports_legacy_replacement()
}

pub fn hpet_address(this: crate::arch::x86_64::acpi::tables::hpet::table::Hpet) -> u64 {
    this.address()
}

pub fn hpet_is_valid(this: crate::arch::x86_64::acpi::tables::hpet::table::Hpet) -> bool {
    this.is_valid()
}

pub fn hpet_oem_attr(this: crate::arch::x86_64::acpi::tables::hpet::table::Hpet) -> u8 {
    this.oem_attr()
}

