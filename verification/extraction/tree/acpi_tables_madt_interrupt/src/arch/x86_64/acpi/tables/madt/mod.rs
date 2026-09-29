// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/madt/header.rs"]
pub mod header;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/madt/interrupt.rs"]
pub mod interrupt;

pub use header::{madt_flags, Madt, MadtEntryHeader};
pub use interrupt::{MadtInterruptOverride, MadtNmiSource};
