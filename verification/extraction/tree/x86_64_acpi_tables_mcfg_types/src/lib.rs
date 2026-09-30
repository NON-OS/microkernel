// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/mcfg_types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn mcfg_entry_count(this: crate::arch::x86_64::acpi::tables::mcfg_types::Mcfg) -> usize {
    this.entry_count()
}

pub fn mcfg_entries_offset(this: crate::arch::x86_64::acpi::tables::mcfg_types::Mcfg) -> usize {
    this.entries_offset()
}

pub fn mcfgentry_bus_count(this: crate::arch::x86_64::acpi::tables::mcfg_types::McfgEntry) -> u16 {
    this.bus_count()
}

pub fn mcfgentry_contains_bus(this: crate::arch::x86_64::acpi::tables::mcfg_types::McfgEntry, bus: u8) -> bool {
    this.contains_bus(bus)
}

pub fn mcfgentry_memory_size(this: crate::arch::x86_64::acpi::tables::mcfg_types::McfgEntry) -> u64 {
    this.memory_size()
}

