// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/fadt/fadt_boot.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn fadt_has_8042(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.has_8042()
}

pub fn fadt_has_legacy_devices(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.has_legacy_devices()
}

pub fn fadt_has_vga(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.has_vga()
}

pub fn fadt_has_msi(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.has_msi()
}

pub fn fadt_has_cmos_rtc(this: crate::arch::x86_64::acpi::tables::fadt::fadt_struct::Fadt) -> bool {
    this.has_cmos_rtc()
}

