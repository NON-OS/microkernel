// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/madt/header.rs"]
pub mod header;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/madt/local_apic_entry.rs"]
pub mod local_apic_entry;

pub use header::{madt_flags, Madt, MadtEntryHeader};
