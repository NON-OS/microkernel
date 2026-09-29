// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/madt/local_apic_nmi.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn madtlocalapicnmi_applies_to_all(this: crate::arch::x86_64::acpi::tables::madt::local_apic_nmi::MadtLocalApicNmi) -> bool {
    this.applies_to_all()
}

pub fn madtlocalapicnmi_polarity(this: crate::arch::x86_64::acpi::tables::madt::local_apic_nmi::MadtLocalApicNmi) -> u8 {
    this.polarity()
}

pub fn madtlocalapicnmi_trigger_mode(this: crate::arch::x86_64::acpi::tables::madt::local_apic_nmi::MadtLocalApicNmi) -> u8 {
    this.trigger_mode()
}

