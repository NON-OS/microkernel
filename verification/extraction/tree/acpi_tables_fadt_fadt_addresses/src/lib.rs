// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/fadt/fadt_addresses.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn fadt_dsdt_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.dsdt_address()
}

pub fn fadt_firmware_control_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.firmware_control_address()
}

pub fn fadt_pm1a_event_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.pm1a_event_address()
}

pub fn fadt_pm1b_event_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.pm1b_event_address()
}

pub fn fadt_pm1a_control_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.pm1a_control_address()
}

pub fn fadt_pm1b_control_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.pm1b_control_address()
}

pub fn fadt_pm_timer_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.pm_timer_address()
}

pub fn fadt_gpe0_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.gpe0_address()
}

pub fn fadt_gpe1_address(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> u64 {
    this.gpe1_address()
}

