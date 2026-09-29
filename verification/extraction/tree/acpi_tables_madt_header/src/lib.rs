// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/madt/header.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn madt_has_legacy_pics(this: crate::arch::x86_64::acpi::tables::madt::header::Madt) -> bool {
    this.has_legacy_pics()
}

pub fn madt_entries_start(this: crate::arch::x86_64::acpi::tables::madt::header::Madt) -> usize {
    this.entries_start()
}

pub fn madt_entries_length(this: crate::arch::x86_64::acpi::tables::madt::header::Madt) -> u32 {
    this.entries_length()
}

pub fn madt_local_apic_addr(this: crate::arch::x86_64::acpi::tables::madt::header::Madt) -> u64 {
    this.local_apic_addr()
}

