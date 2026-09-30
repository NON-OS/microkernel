// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/acpi/tables/sdt/entry_count.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/acpi/tables/sdt/entry_count.rs"]
pub mod entry_count;

pub fn sdt_entry_count(table_length: usize, header_size: usize, entry_size: usize) -> usize {
    entry_count::sdt_entry_count(table_length, header_size, entry_size)
}

