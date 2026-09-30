// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/madt/header.rs"]
pub mod header;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/madt/io_apic.rs"]
pub mod io_apic;

pub use header::{madt_flags, Madt, MadtEntryHeader};
pub use io_apic::{MadtIoApic, MadtIoSapic};
