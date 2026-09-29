// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/madt/x2apic_local.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn madtlocalx2apic_is_enabled(this: crate::arch::x86_64::acpi::tables::madt::x2apic_local::MadtLocalX2Apic) -> bool {
    this.is_enabled()
}

pub fn madtlocalx2apic_is_online_capable(this: crate::arch::x86_64::acpi::tables::madt::x2apic_local::MadtLocalX2Apic) -> bool {
    this.is_online_capable()
}

pub fn madtlocalx2apic_is_usable(this: crate::arch::x86_64::acpi::tables::madt::x2apic_local::MadtLocalX2Apic) -> bool {
    this.is_usable()
}

