// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/acpi/tables/srat_types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn srat_entries_offset(this: crate::arch::x86_64::acpi::tables::srat_types::Srat) -> usize {
    this.entries_offset()
}

pub fn srat_entries_length(this: crate::arch::x86_64::acpi::tables::srat_types::Srat) -> u32 {
    this.entries_length()
}

