// NONOS Operating System (AGPL-3.0-or-later)

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/entry_count.rs"]
pub mod entry_count;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/header.rs"]
pub mod header;

#[path = "../../../../../../../../../../src/arch/x86_64/acpi/tables/sdt/span.rs"]
pub mod span;

pub use header::SdtHeader;
pub use span::table_span;
