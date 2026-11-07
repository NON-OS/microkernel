// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/hw/madt_cpu.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/hw/madt_cpu.rs"]
pub mod madt_cpu;

pub fn processor_usable(flags: u32, madt_revision: u8) -> bool {
    madt_cpu::processor_usable(flags, madt_revision)
}

pub fn keep_entry(apic_id: u32, is_x2apic: bool, table_has_xapic_cpus: bool) -> bool {
    madt_cpu::keep_entry(apic_id, is_x2apic, table_has_xapic_cpus)
}

