// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/header.rs"]
pub mod header;

pub use header::SdtHeader;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/entry_count.rs"]
mod entry_count;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/span.rs"]
mod span;
